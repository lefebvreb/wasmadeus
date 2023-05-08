use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;

use crate::signal::SignalError;

use super::Result;

/// A pointer to a value whose type was erased.
type Erased = NonNull<()>;

/// A closure that reacts to a new value, passed by reference.
///
/// Must not mutate the reference.
type NotifyFn = dyn FnMut(Erased);

/// The ID of a subscription to a signal, can be used to unsubscribe from
/// this signal.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SubscriberId(usize);

/// A subscriber to a signal, with it's ID, notify closure
/// and wether it is still active or is awaiting being dropped.
struct Subscriber {
    id: SubscriberId,
    active: Cell<bool>,
    notify: NonNull<NotifyFn>,
}

impl Subscriber {
    /// Returns the ID of this subscriber.
    #[inline]
    fn id(&self) -> SubscriberId {
        self.id
    }

    /// Returns true iff this subscriber is still willing
    /// to receive more values, and false if it needs to be
    /// dropped.
    #[inline]
    fn active(&self) -> bool {
        self.active.get()
    }
}

impl Drop for Subscriber {
    #[inline]
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.notify.as_mut()) };
    }
}

/// The internal state of a [`Broadcast`].
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum State {
    Idling,
    Notifying,
    Subscribing,
    Uninit,
}

struct Broadcast {
    state: Cell<State>,
    next_id: Cell<usize>,
    needs_retain: Cell<bool>,
    subscribers: UnsafeCell<Vec<Subscriber>>,
}

impl Broadcast {
    #[inline]
    fn new(state: State) -> Self {
        Self {
            state: Cell::new(state),
            next_id: Cell::new(0),
            needs_retain: Cell::new(false),
            subscribers: UnsafeCell::new(Vec::new()),
        }
    }

    #[inline]
    fn state(&self) -> State {
        self.state.get()
    }

    fn next_id(&self) -> SubscriberId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        SubscriberId(id)
    }

    /// # Safety
    ///
    /// The state must neither be `Subscribing` or `Notifying`, so
    /// that none of the subscribers are currently borrowed.
    unsafe fn retain(&self) {
        if self.needs_retain.replace(false) {
            let subscribers = self.subscribers.get();
            (*subscribers).retain(Subscriber::active)
        }
    }

    fn push_subscriber(&self, id: SubscriberId, mut notify: NonNull<NotifyFn>, value: Erased) {
        let subscribers = self.subscribers.get();

        unsafe {
            (*subscribers).push(Subscriber {
                id,
                active: Cell::new(true),
                notify,
            });
        }

        match self.state() {
            State::Idling => unsafe {
                self.state.set(State::Subscribing);
                notify.as_mut()(value);
                self.state.set(State::Idling);
                self.retain();
            },
            State::Subscribing => unsafe {
                notify.as_mut()(value);
            },
            _ => (),
        }
    }

    fn unsubscribe(&self, id: SubscriberId) -> Option<()> {
        let subscribers = self.subscribers.get();

        let index = unsafe {
            (*subscribers)
                .binary_search_by_key(&id, Subscriber::id)
                .ok()?
        };

        match self.state() {
            State::Notifying | State::Subscribing => unsafe {
                let subscriber = &(*subscribers)[index];
                subscriber.active.set(false);
                self.needs_retain.set(true);
            },
            _ => unsafe {
                (*subscribers).remove(index);
            },
        }

        Some(())
    }

    unsafe fn notify_all(&self, value: Erased) {
        let subscribers = self.subscribers.get();
        let mut i = 0;

        unsafe {
            while i < (*subscribers).len() {
                let subscriber = (*subscribers).as_mut_ptr().add(i);
                if (*subscriber).active() {
                    let mut notify = (*subscriber).notify;
                    (notify.as_mut())(value);
                }
                i += 1;
            }

            self.retain();
        }
    }
}

struct InnerSignal<T> {
    broadcast: Broadcast,
    value: Rc<UnsafeCell<MaybeUninit<T>>>,
}

impl<T> InnerSignal<T> {
    #[inline]
    fn value(&self) -> Erased {
        NonNull::new(self.value.get()).unwrap().cast()
    }

    #[inline]
    fn state(&self) -> State {
        self.broadcast.state()
    }

    #[inline]
    fn set_state(&self, state: State) {
        self.broadcast.state.set(state);
    }

    #[inline]
    fn is_initialized(&self) -> bool {
        self.broadcast.state() != State::Uninit
    }

    #[inline]
    fn unsubscribe(&self, id: SubscriberId) {
        self.broadcast.unsubscribe(id);
    }

    fn try_get(&self) -> Result<T>
    where
        T: Clone,
    {
        if matches!(self.state(), State::Notifying | State::Uninit) {
            return Err(SignalError);
        }

        let value = unsafe { self.value().cast::<T>().as_ref().clone() };
        Ok(value)
    }

    fn raw_for_each<F, G>(&self, make_notify: G) -> SubscriberId
    where
        F: FnMut(&T) + 'static,
        G: FnOnce(SubscriberId) -> F,
    {
        let id = self.broadcast.next_id();

        let notify = {
            let mut f = make_notify(id);
            let boxed = Box::new(move |value: Erased| {
                f(unsafe { value.cast().as_mut() });
            });
            NonNull::new(Box::into_raw(boxed)).unwrap()
        };

        self.broadcast.push_subscriber(id, notify, self.value());

        id
    }

    #[inline]
    fn try_set(&self, new_value: T) -> Result<()> {
        let value = self.value.get();

        match self.state() {
            State::Idling => unsafe {
                *(*value).assume_init_mut() = new_value;
            },
            State::Uninit => unsafe {
                (*value).write(new_value);
            },
            _ => return Err(SignalError),
        }

        self.set_state(State::Notifying);
        unsafe { self.broadcast.notify_all(self.value()) };
        self.set_state(State::Idling);

        Ok(())
    }

    #[inline]
    fn try_mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        if self.state() != State::Idling {
            return Err(SignalError);
        }

        self.set_state(State::Notifying);
        unsafe {
            let value = self.value();
            mutate(value.cast().as_mut());
            self.broadcast.notify_all(value);
        }
        self.set_state(State::Idling);

        Ok(())
    }
}

#[repr(transparent)]
pub struct Signal<T: 'static>(Rc<InnerSignal<T>>);

impl<T> Signal<T> {
    fn new(value: Rc::<UnsafeCell<MaybeUninit<T>>>, state: State) -> Self {
        Self(Rc::new(InnerSignal {
            broadcast: Broadcast::new(state),
            value,
        }))
    }

    #[inline]
    fn inner(&self) -> &Rc<InnerSignal<T>> {
        &self.0
    }

    #[inline]
    pub fn try_get(&self) -> Result<T>
    where
        T: Clone,
    {
        self.0.try_get()
    }

    pub fn for_each<F>(&self, notify: F) -> Unsubscriber<T> 
    where
        F: FnMut(&T) + 'static,
    {
        let id = self.inner().raw_for_each(|_| notify);
        Unsubscriber::new(Rc::downgrade(self.inner()), id)
    }

    pub fn for_each_inner<F>(&self, mut notify: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static,
    {
        let weak = Rc::downgrade(self.inner());
        self.inner().raw_for_each(|id| {
            let mut unsub = Unsubscriber::new(weak, id);
            move |data| notify(data, &mut unsub)
        });
    }

    #[inline]
    pub fn for_each_forever<F>(&self, notify: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.inner().raw_for_each(|_| notify);
    }
}

impl<T> Clone for Signal<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[repr(transparent)]
pub struct Mutable<T: 'static>(Signal<T>);

impl<T> Mutable<T> {
    #[inline]
    fn new_with_state(value: MaybeUninit<T>, state: State) -> Self {
        Self(Signal::new(Rc::new(UnsafeCell::new(value)), state))
    }

    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self::new_with_state(MaybeUninit::new(initial_value), State::Idling)
    }

    #[inline]
    pub fn uninit() -> Self {
        Self::new_with_state(MaybeUninit::uninit(), State::Uninit)
    }
}

impl<T> Clone for Mutable<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for Mutable<T> {
    type Target = Signal<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl<T> Signal<T> {
//     // #[inline]
//     // pub fn new(initial_value: T) -> Self {
//     //     let storage = UnsafeCell::new(MaybeUninit::new(initial_value));
//     //     Self::new_with_state(storage, State::Idling)
//     // }

//     // #[inline]
//     // pub fn uninit() -> Self {
//     //     let storage = UnsafeCell::new(MaybeUninit::uninit());
//     //     Self::new_with_state(storage, State::Uninit)
//     // }

//     pub fn try_set(&self, new_value: T) -> Result<()> {
//         let value = self.value.get();

//         match self.state() {
//             State::Idling => unsafe {
//                 *(*value).assume_init_mut() = new_value;
//             },
//             State::Uninit => unsafe {
//                 (*value).write(new_value);
//             },
//             _ => return Err(SignalError),
//         }

//         self.set_state(State::Mutating);
//         unsafe { self.inner.notify_all(self.value()) };
//         self.set_state(State::Idling);

//         Ok(())
//     }

//     #[inline]
//     pub fn try_mutate<F>(&self, mutate: F) -> Result<()>
//     where
//         F: FnOnce(&mut T),
//     {
//         if self.state() != State::Idling {
//             return Err(SignalError);
//         }

//         self.set_state(State::Mutating);
//         unsafe {
//             let value = self.value();
//             mutate(value.cast().as_mut());
//             self.inner.notify_all(value);
//         }
//         self.set_state(State::Idling);

//         Ok(())
//     }
// }

// impl<T> RawFilter<T> {
//     // pub fn from_mutable<F>(signal: &RawMutable<T>, filter: F) -> Self
//     // where
//     //     F: FnMut(&T) -> bool,
//     // {
//     //     let state = match signal.state() {
//     //         State::Mutating => State::Mutating,
//     //         _ => State::Idling,
//     //     };
//     //     let this = Self::new_with_state(signal.value().cast(), state);
//     //     // todo(benjamin): for_each_inner and filter, using this.notify_all()
//     //     todo!()
//     // }

//     // unsafe fn notify_all(&self) {
//     //     self.set_state(State::Mutating);
//     //     unsafe { self.inner.notify_all(self.value()) };
//     //     self.set_state(State::Idling);
//     // }
// }

// impl<T> Drop for Signal<T> {
//     #[inline]
//     fn drop(&mut self) {
//         if self.is_initialized() {
//             // SAFETY: the data is initialized and valid.
//             unsafe { self.storage.drop() };
//         }
//     }
// }

#[repr(transparent)]
pub struct Unsubscriber<T>(Option<(Weak<InnerSignal<T>>, SubscriberId)>);

impl<T> Unsubscriber<T> {
    #[inline]
    fn new(weak: Weak<InnerSignal<T>>, id: SubscriberId) -> Self {
        Self(Some((weak, id)))
    }

    pub fn unsubscribe(&mut self) {
        if let Some((weak, id)) = self.0.take() {
            if let Some(raw) = weak.upgrade() {
                raw.unsubscribe(id);
            }
        }
    }

    #[inline]
    pub fn has_effect(&self) -> bool {
        self.0.is_some()
    }
}

impl<T> Clone for Unsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
