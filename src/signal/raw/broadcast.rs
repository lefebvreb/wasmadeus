//! A [`Broadcast`] is used to register subscribers (closures to be invoked on state change),
//! and notify them, whilst allowing subscription/unsubscription at any time safely.

// todo: erase typings of broadcast to save wasm binary size.

use core::cell::{Cell, UnsafeCell};
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::signal::Unsubscriber;

use super::SubscriberId;

/// A closure that reacts to a new value, passed by reference.
type NotifyFn<T> = dyn FnMut(&T);

/// A subscriber to a signal, with it's ID, notify closure
/// and wether it is still active or is awaiting being dropped.
struct Subscriber<T> {
    id: SubscriberId,
    active: Cell<bool>,
    notify: NonNull<NotifyFn<T>>,
}

impl<T> Subscriber<T> {
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

    /// Returns a pointer to the notify function of this
    /// subscriber.
    #[inline]
    fn notify(&self) -> NonNull<NotifyFn<T>> {
        self.notify
    }
}

impl<T> Drop for Subscriber<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.notify.as_mut());
        }
    }
}

/// The state of a broadcast.
#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    Idling,
    Notifying,
    Subscribing,
}

/// A [`Broadcast`] is used to register subscribers (closures to be invoked on state change),
/// and notify them, whilst allowing subscription/unsubscription at any time safely.
pub struct Broadcast<T> {
    state: Cell<State>,
    next_id: Cell<usize>,
    needs_retain: Cell<bool>,
    subscribers: UnsafeCell<Vec<Subscriber<T>>>,
}

impl<T> Broadcast<T> {
    /// Retains the subscribers that still want to be notified, the other
    /// being dropped.
    ///
    /// # Safety
    ///
    /// The state of the broadcast must be `State::Idling`, so that
    /// no subscribers is borrowed while ratain takes place.
    unsafe fn retain(&self) {
        if self.needs_retain.replace(false) {
            let subscribers = self.subscribers.get();
            (*subscribers).retain(Subscriber::active)
        }
    }

    /// Returns the next ID to be attributed to a subscriber.
    pub fn next_id(&self) -> SubscriberId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        SubscriberId(id)
    }

    /// Push a new subscriber at the end of the subscriber list.
    ///
    /// The `id` given must be the last one in the list (e.g. provided by the latest call
    /// to [`Broadcast::next_id`]). Else, expect funny (but safe) stuff to happen.
    ///
    /// if `Some` `value` is provided, the subscriber is immediately
    /// notified (given the broadcast is not already notifying).
    pub fn push_subscriber(&self, id: SubscriberId, notify: Box<NotifyFn<T>>, value: Option<&T>) {
        let subscriber = Subscriber {
            id,
            active: Cell::new(true),
            notify: NonNull::new(Box::into_raw(notify)).unwrap(),
        };

        let mut notify = subscriber.notify();
        let subscribers = self.subscribers.get();

        unsafe {
            (*subscribers).push(subscriber);

            if let Some(value) = value {
                match self.state.get() {
                    State::Idling => {
                        self.state.set(State::Subscribing);
                        notify.as_mut()(value);
                        self.state.set(State::Idling);
                    }
                    State::Notifying => (),
                    State::Subscribing => {
                        notify.as_mut()(value);
                    }
                }
            }
        }
    }

    /// Notify all subscribers of a `value` change.
    ///
    /// If the state was already notifying (all subscribers or just a single new one), this
    /// function does nothing.
    pub fn notify(&self, value: &T) {
        if self.state.get() != State::Idling {
            return;
        }

        self.state.set(State::Notifying);
        let subscribers = self.subscribers.get();

        unsafe {
            let mut i = 0;

            while i < (*subscribers).len() {
                let subscriber = (*subscribers).as_mut_ptr().add(i);
                if (*subscriber).active() {
                    let mut notify = (*subscriber).notify();
                    notify.as_mut()(value);
                }
                i += 1;
            }

            self.retain()
        }

        self.state.set(State::Idling);
    }

    /// Unsubscribes the subscriber with the given `id`.
    ///
    /// If the subscriber is already unsubscribed, this function does nothing.
    ///
    /// The subscriber might not get dropped right away, but won't be called again.
    pub fn unsubscribe(&self, id: SubscriberId) {
        let subscribers = self.subscribers.get();

        unsafe {
            if let Ok(index) = (*subscribers).binary_search_by_key(&id, Subscriber::id) {
                match self.state.get() {
                    State::Idling => {
                        (*subscribers).remove(index);
                    }
                    _ => {
                        let subscriber = &(*subscribers)[index];
                        subscriber.active.set(false);
                        self.needs_retain.set(true);
                    }
                }
            }
        }
    }
}

impl<T> Default for Broadcast<T> {
    #[inline]
    fn default() -> Self {
        Self {
            state: Cell::new(State::Idling),
            next_id: Cell::new(0),
            needs_retain: Cell::new(false),
            subscribers: UnsafeCell::new(Vec::new()),
        }
    }
}
