use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::ptr::NonNull;

use alloc::boxed::Box;
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
        unsafe {
            let _ = Box::from_raw(self.notify.as_mut());
        }
    }
}

/// The internal state of a RawSignal.
#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    Idling,
    Mutating,
    Subscribing,
    Uninit,
}

struct InnerRawSignal {
    state: Cell<State>,
    next_id: Cell<usize>,
    needs_retain: Cell<bool>,
    subscribers: UnsafeCell<Vec<Subscriber>>,
}

impl InnerRawSignal {
    fn new_with_state(state: State) -> Self {
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
    /// The state must neither be `Subscribing` or `Mutating`, so
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
            State::Mutating | State::Subscribing => unsafe {
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

    // TODO: check states
    unsafe fn notify_all(&self, value: Erased) {
        let subscribers = self.subscribers.get();
        let mut i = 0;

        unsafe {
            while i < (*subscribers).len() {
                let subscriber = (*subscribers).as_mut_ptr().add(i).as_ref().unwrap();
                if subscriber.active() {
                    let mut notify = subscriber.notify;
                    (notify.as_mut())(value);
                }
                i += 1;
            }

            self.retain();
        }
    }
}

pub trait SignalStorage: 'static {
    type Data;

    fn get(&self) -> NonNull<()>;

    /// # Safety
    /// 
    /// `self` must be initialized and valid to call this method.
    #[inline]
    unsafe fn drop(&mut self) {}
}

impl<T: 'static> SignalStorage for UnsafeCell<MaybeUninit<T>> {
    type Data = T;

    #[inline]
    fn get(&self) -> NonNull<()> {
        NonNull::new(self.get()).unwrap().cast()
    }

    #[inline]
    unsafe fn drop(&mut self) {
        (*self.get()).assume_init_drop();
    }
}

impl<T: 'static> SignalStorage for NonNull<T> {
    type Data = T;

    #[inline]
    fn get(&self) -> NonNull<()> {
        self.cast()
    }
}

pub struct RawSignal<S: SignalStorage> {
    storage: S,
    inner: InnerRawSignal,
}

impl<S: SignalStorage> RawSignal<S> {
    #[inline]
    fn new_with_state(storage: S, state: State) -> Self {
        Self {
            storage,
            inner: InnerRawSignal::new_with_state(state),
        }
    }

    #[inline]
    fn value(&self) -> Erased {
        self.storage.get()
    }

    #[inline]
    fn state(&self) -> State {
        self.inner.state()
    }

    #[inline]
    fn set_state(&self, state: State) {
        self.inner.state.set(state);
    }

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.inner.state() != State::Uninit
    }

    pub unsafe fn for_each<F, G>(&self, make_notify: G) -> SubscriberId
    where
        F: FnMut(&S::Data) + 'static,
        G: FnOnce(SubscriberId) -> F,
    {
        let id = self.inner.next_id();

        let notify = {
            let mut f = make_notify(id);
            let boxed = Box::new(move |data: Erased| {
                f(data.cast().as_mut());
            });
            NonNull::new(Box::into_raw(boxed)).unwrap()
        };

        self.inner.push_subscriber(id, notify, self.storage.get());

        id
    }

    #[inline]
    pub fn unsubscribe(&self, id: SubscriberId) {
        self.inner.unsubscribe(id);
    }
}

impl<T: 'static> RawSignal<UnsafeCell<MaybeUninit<T>>> {
    #[inline]
    pub fn new(value: T) -> Self {
        let storage = UnsafeCell::new(MaybeUninit::new(value));
        Self::new_with_state(storage, State::Idling)
    }

    #[inline]
    pub fn uninit() -> Self {
        let storage = UnsafeCell::new(MaybeUninit::uninit());
        Self::new_with_state(storage, State::Uninit)
    }

    pub unsafe fn set(&self, new_value: T) -> Result<()> {
        let storage = self.storage.get();

        match self.state() {
            State::Idling => {
                *(*storage).assume_init_mut() = new_value;
            },
            State::Uninit => {
                (*storage).write(new_value);
            },
            _ => return Err(SignalError),
        }

        self.set_state(State::Mutating);
        self.inner.notify_all(self.value());
        self.set_state(State::Idling);

        Ok(())
    }

    #[inline]
    pub unsafe fn mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        if self.state() != State::Idling {
            return Err(SignalError);
        }

        self.set_state(State::Mutating);
        mutate(self.value().cast().as_mut());
        self.inner.notify_all(self.value());
        self.set_state(State::Idling);

        Ok(())
    }
}

impl<T: 'static> RawSignal<NonNull<T>> {
    #[inline]
    pub fn new(value: NonNull<T>) -> Self {
        Self::new_with_state(value, State::Idling)
    }

    #[inline]
    pub fn uninit(value: NonNull<T>) -> Self {
        Self::new_with_state(value, State::Uninit)
    }

    pub unsafe fn notify_all(&self) {
        self.set_state(State::Mutating);
        self.inner.notify_all(self.value());
        self.set_state(State::Idling);
    }
}

impl<S: SignalStorage> Drop for RawSignal<S> {
    #[inline]
    fn drop(&mut self) {
        if self.is_initialized() {
            // SAFETY: the data is initialized and valid.
            unsafe {
                self.storage.drop();
            }
        }
    }
}

// pub type RawMutable<T> = RawSignal<UnsafeCell<MaybeUninit<T>>>;

// pub type RawFilter<T> = RawSignal<NonNull<T>>;
