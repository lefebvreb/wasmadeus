use core::cell::{Cell, UnsafeCell};
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::vec::Vec;

/// A value who's type was erased.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Erased(NonNull<()>);

impl Erased {
    /// Erase the type of this reference.
    #[inline]
    pub fn new<T>(value: &T) -> Self {
        Self(NonNull::from(value).cast())
    }

    /// Casts this erased value into an immutable reference to a `T`.
    ///
    /// # Safety
    ///
    /// The erased pointer must point to a valid value of type `T`,
    /// that is not currently borrowed mutably.
    #[inline]
    pub unsafe fn cast<T>(&self) -> &T {
        self.0.cast().as_ref()
    }
}

type Notify = dyn FnMut(Erased);

struct Subscriber {
    id: SubscriberId,
    active: Cell<bool>,
    notify: NonNull<Notify>,
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.notify.as_mut());
        }
    }
}

impl Subscriber {
    #[inline]
    fn id(&self) -> SubscriberId {
        self.id
    }

    #[inline]
    fn active(&self) -> bool {
        self.active.get()
    }
}

#[derive(Default)]
pub struct Broadcast {
    next_id: Cell<u32>,
    needs_retain: Cell<bool>,
    subscribers: UnsafeCell<Vec<Subscriber>>,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriberId(u32);

impl Broadcast {
    /// Returns the next id
    fn next_id(&self) -> SubscriberId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        SubscriberId(id)
    }

    #[inline]
    pub fn subscribe<T, F, G>(&self, make_notifier: G) -> (SubscriberId, NonNull<Notify>)
    where
        F: FnMut(&T) + 'static,
        G: FnOnce(SubscriberId) -> F,
    {
        let id = self.next_id();
        let notify = {
            let mut f = make_notifier(id);
            let boxed = Box::new(move |data: Erased| f(unsafe { data.cast() }));
            NonNull::new(Box::into_raw(boxed)).unwrap()
        };
        {
            let subscribers = self.subscribers.get();
            unsafe {
                (*subscribers).push(Subscriber {
                    id,
                    active: Cell::new(true),
                    notify,
                })
            }
        }
        (id, notify)
    }

    fn find(&self, id: SubscriberId) -> Option<usize> {
        let subscribers = self.subscribers.get();
        unsafe {
            (*subscribers)
                .binary_search_by_key(&id, Subscriber::id)
                .ok()
        }
    }

    pub fn lazy_unsubscribe(&self, id: SubscriberId) {
        if let Some(index) = self.find(id) {
            let subscribers = self.subscribers.get();
            unsafe {
                let subscriber = (*subscribers).get(index).unwrap();
                subscriber.active.set(false);
                self.needs_retain.set(true);
            }
        }
    }

    /// Removes the
    pub unsafe fn unsubscribe(&self, id: SubscriberId) {
        if let Some(index) = self.find(id) {
            let subscribers = self.subscribers.get();
            (*subscribers).remove(index);
        }
    }

    unsafe fn notify_all_erased(&self, data: Erased) {
        let subscribers = self.subscribers.get();
        let mut i = 0;
        while i < (*subscribers).len() {
            let notifier = (*subscribers).as_mut_ptr().add(i).as_ref().unwrap();
            if notifier.active() {
                let mut notify = notifier.notify;
                (notify.as_mut())(data);
            }
            i += 1;
        }
        if self.needs_retain.replace(false) {
            (*subscribers).retain(Subscriber::active)
        }
    }

    #[inline]
    pub unsafe fn notify_all<T>(&self, value: &T) {
        self.notify_all_erased(Erased::new(value));
    }
}
