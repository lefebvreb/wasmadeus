use core::cell::{Cell, UnsafeCell};
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{Erased, SubscriberId};

/// A dynamic closure that reacts to a new value, passed by reference.
///
/// Must not mutate the reference.
type NotifyFn = dyn FnMut(Erased);

/// A subscriber to a signal, with it's ID, notify closure
/// and wether it is still active or is awaiting being dropped.
pub struct Subscriber {
    id: SubscriberId,
    active: Cell<bool>,
    notify: NonNull<NotifyFn>,
}

impl Subscriber {
    /// Returns the ID of this subscriber.
    #[inline]
    pub fn id(&self) -> SubscriberId {
        self.id
    }

    /// Returns true iff this subscriber is still willing
    /// to receive more values, and false if it needs to be
    /// dropped.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.get()
    }
}

impl Drop for Subscriber {
    #[inline]
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.notify.as_mut()) };
    }
}

#[derive(Default)]
pub struct Broadcast {
    next_id: Cell<usize>,
    notifying: Cell<bool>,
    needs_retain: Cell<bool>,
    subscribers: UnsafeCell<Vec<Subscriber>>,
}

impl Broadcast {
    pub fn next_id(&self) -> SubscriberId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        SubscriberId(id)
    }

    pub unsafe fn retain(&self) {
        if self.needs_retain.replace(false) {
            let subscribers = self.subscribers.get();
            (*subscribers).retain(Subscriber::active)
        }
    }

    pub unsafe fn push_subscriber(
        &self,
        id: SubscriberId,
        mut notify: NonNull<NotifyFn>,
        value: Option<Erased>,
    ) {
        let subscribers = self.subscribers.get();

        (*subscribers).push(Subscriber {
            id,
            active: Cell::new(true),
            notify,
        });

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

    pub unsafe fn notify(&self, value: Erased) {
        if self.notifying.replace(true) {
            return;
        }

        let subscribers = self.subscribers.get();

        let mut i = 0;
        while i < (*subscribers).len() {
            let subscriber = (*subscribers).as_mut_ptr().add(i);

            if (*subscriber).active() {
                let mut notify = (*subscriber).notify;
                notify.as_mut()(value);
            }

            i += 1;
        }

        self.notifying.set(false);
        self.retain();
    }
}

