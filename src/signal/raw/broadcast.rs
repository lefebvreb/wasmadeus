use core::cell::{Cell, UnsafeCell};
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::SubscriberId;

/// A dynamic closure that reacts to a new value, passed by reference.
///
/// Must not mutate the reference.
type NotifyFn<T> = dyn FnMut(&T);

/// A subscriber to a signal, with it's ID, notify closure
/// and wether it is still active or is awaiting being dropped.
struct Subscriber<T> {
    id: SubscriberId,
    active: Cell<bool>,
    notify: Box<NotifyFn<T>>,
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
        NonNull::from(&*self.notify)
    }
}

pub struct Broadcast<T> {
    next_id: Cell<usize>,
    notifying: Cell<bool>,
    needs_retain: Cell<bool>,
    subscribers: UnsafeCell<Vec<Subscriber<T>>>,
}

impl<T> Broadcast<T> {
    pub fn next_id(&self) -> SubscriberId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        SubscriberId(id)
    }

    unsafe fn retain(&self) {
        if self.needs_retain.replace(false) {
            let subscribers = self.subscribers.get();
            (*subscribers).retain(Subscriber::active)
        }
    }

    pub fn push_subscriber(
        &self,
        id: SubscriberId,
        notify: Box<NotifyFn<T>>,
        value: Option<&T>,
    ) {
        let subscriber = Subscriber {
            id,
            active: Cell::new(true),
            notify,
        };
        
        let mut notify = subscriber.notify();
        let subscribers = self.subscribers.get();

        unsafe {
            (*subscribers).push(subscriber);

            if let Some(value) = value {
                if self.notifying.replace(true) {
                    notify.as_mut()(value);
                } else {
                    notify.as_mut()(value);
                    self.notifying.set(false);
                    self.retain();
                }
            }
        }

    }

    pub fn unsubscribe(&self, id: SubscriberId) {
        let subscribers = self.subscribers.get();

        unsafe {
            if let Some(index) = (*subscribers)
                .binary_search_by_key(&id, Subscriber::id)
                .ok()
            {
                if self.notifying.get() {
                    let subscriber = &(*subscribers)[index];
                    subscriber.active.set(false);
                    self.needs_retain.set(true);
                } else {
                    (*subscribers).remove(index);
                }
            }
        }
    }

    pub fn notify(&self, value: &T) {
        if self.notifying.replace(true) {
            return;
        }

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
        }

        self.notifying.set(false);

        unsafe {
            self.retain();
        }
    }
}

impl<T> Default for Broadcast<T> {
    #[inline]
    fn default() -> Self {
        Self {
            next_id: Cell::new(0),
            notifying: Cell::new(false),
            needs_retain: Cell::new(false),
            subscribers: UnsafeCell::new(Vec::new()),
        }    
    }
}
