use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::rc::Rc;
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
pub struct SubscriberId(usize);

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

/// The internal state of a signal.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum State {
    Idling,
    Notifying,
    Subscribing,
    Uninit,
}

// broadcast state:
// * idling
// * notifying

#[derive(Default)]
struct Broadcast {
    next_id: Cell<usize>,
    needs_retain: Cell<bool>,
    subscribers: UnsafeCell<Vec<Subscriber>>,
}

impl Broadcast {
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

    unsafe fn push_subscriber(&self, id: SubscriberId, mut notify: NonNull<NotifyFn>, erased: ErasedData) {
        let subscribers = self.subscribers.get();

        unsafe {
            (*subscribers).push(Subscriber {
                id,
                active: Cell::new(true),
                notify,
            });
        }

        match erased.state.get() {
            State::Idling => unsafe {
                erased.state.set(State::Subscribing);
                notify.as_mut()(erased.value);
                erased.state.set(State::Idling);
                self.retain();
            },
            State::Subscribing => unsafe {
                notify.as_mut()(erased.value);
            },
            _ => (),
        }
    }

    unsafe fn unsubscribe(&self, id: SubscriberId, state: State) -> Option<()> {
        let subscribers = self.subscribers.get();

        let index = unsafe {
            (*subscribers)
                .binary_search_by_key(&id, Subscriber::id)
                .ok()?
        };

        match state {
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

    unsafe fn notify(&self, data: ErasedData) {
        data.state.set(State::Notifying);

        let subscribers = self.subscribers.get();
        let mut i = 0;

        unsafe {
            while i < (*subscribers).len() {
                let subscriber = (*subscribers).as_mut_ptr().add(i);
                if (*subscriber).active() {
                    let mut notify = (*subscriber).notify;
                    (notify.as_mut())(data.value);
                }
                i += 1;
            }

            self.retain();
        }

        data.state.set(State::Idling);
    }

    // unsafe fn mutate<F>(&self, make_value: F) 
    // where
    //     F: FnOnce() -> Erased,
    // {
    //     self.state.set(State::Mutating);
    //     let value = make_value();
    //     self.notify(value);
    //     self.state.set(State::Idling);
    // }
}

struct SignalData<T> {
    state: Cell<State>,
    value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> SignalData<T> {
    #[inline]
    fn new(initial_value: T) -> Self {
        Self {
            state: Cell::new(State::Idling),
            value: UnsafeCell::new(MaybeUninit::new(initial_value)),
        }
    }

    #[inline]
    fn uninit() -> Self {
        Self {
            state: Cell::new(State::Uninit),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    #[inline]
    fn value(&self) -> Erased {
        NonNull::new(self.value.get()).unwrap().cast()
    }

    #[inline]
    fn erased(&self) -> ErasedData {
        ErasedData {
            state: &self.state,
            value: self.value(),
        }
    }
}

// data state:
// * idling
// * uninit
// * mutating
// * borrowed

struct ErasedData<'a> {
    state: &'a Cell<State>,
    value: Erased,
}

pub struct RawSignal<T> {
    broadcast: Broadcast,
    data: Rc<SignalData<T>>,
}

impl<T> RawSignal<T> {
    fn new_with_data(data: Rc<SignalData<T>>) -> Self {
        Self {
            broadcast: Broadcast::default(),
            data,
        }
    }

    #[inline]
    pub fn new(initial_value: T) -> Self {
        let data = Rc::new(SignalData::new(initial_value));
        Self::new_with_data(data)
    }

    #[inline]
    pub fn uninit() -> Self {
        let data = Rc::new(SignalData::uninit());
        Self::new_with_data(data)
    }

    #[inline]
    pub fn new_derived(&self) -> Self {
        Self::new_with_data(self.data.clone())
    }

    #[inline]
    fn value(&self) -> Erased {
        self.data.value()
    }

    #[inline]
    fn state(&self) -> &Cell<State> {
        &self.data.state
    }

    #[inline]
    pub fn unsubscribe(&self, id: SubscriberId) {
        unsafe {
            self.broadcast.unsubscribe(id, self.state().get());
        }
    }

    pub fn try_get(&self) -> Result<T>
    where
        T: Clone,
    {
        if matches!(self.state().get(), State::Notifying | State::Uninit) {
            return Err(SignalError);
        }

        let value = unsafe { self.value().cast::<T>().as_ref().clone() };
        Ok(value)
    }

    pub fn raw_for_each<F, G>(&self, make_notify: G) -> SubscriberId
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

        unsafe {
            self.broadcast.push_subscriber(id, notify, self.data.erased());
        }

        id
    }

    pub fn try_notify(&self) -> Result<()> {
        todo!()
    }

    pub fn try_set(&self, new_value: T) -> Result<()> {
        let value = self.data.value.get();

        match self.state().get() {
            State::Idling => unsafe {
                *(*value).assume_init_mut() = new_value;
            },
            State::Uninit => unsafe {
                (*value).write(new_value);
            },
            _ => return Err(SignalError),
        }

        unsafe {
            self.broadcast.notify(self.data.erased());
        }

        Ok(())
    }

    pub fn try_mutate<F>(&self, mutate: F) -> Result<()> 
    where
        F: FnOnce(&mut T),
    {
        todo!()
    }
}
