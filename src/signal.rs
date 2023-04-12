use core::cell::UnsafeCell;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;

fn fn_id<T>(f: &Box<dyn FnMut(&T)>) -> usize {
    f as *const dyn FnMut(&T) as *const () as usize
}

pub trait Subscribable<T> {
    fn subscribe(&self, notify: impl FnMut(&T) + 'static) -> Unsubscriber;
}

struct SignalInternal<T> {
    data: T,
    subscribers: Vec<Box<dyn FnMut(&T)>>,
}

impl<T> SignalInternal<T> {
    fn find(&self, id: usize) -> Result<usize, usize> {
        self.subscribers.binary_search_by_key(&id, |f| fn_id(f))
    }

    fn mutate(&mut self, mutater: impl FnOnce(&mut T)) {
        mutater(&mut self.data);
        
        for subscriber in &mut self.subscribers {
            subscriber(&self.data);
        }
    }

    fn subscribe(&mut self, mut subscriber: Box<dyn FnMut(&T)>) -> usize {
        let id = fn_id(&subscriber);
        let index = self.find(id).unwrap_err();
        subscriber(&self.data);
        self.subscribers.insert(index, subscriber);
        id
    }

    fn unsubscribe(&mut self, id: usize) {
        let index = self.find(id).unwrap();
        let _ = self.subscribers.remove(index);
    }
}

#[repr(transparent)]
pub struct Signal<T>(Rc<UnsafeCell<SignalInternal<T>>>);

impl<T> Signal<T> {
    #[inline(always)]
    unsafe fn internal(&self) -> &mut SignalInternal<T> {
        &mut *self.0.get()
    }

    pub fn new(data: T) -> Self {
        let internal = SignalInternal {
            data,
            subscribers: Vec::new(),
        };

        Self(Rc::new(UnsafeCell::new(internal)))
    }

    pub fn mutate(&self, mutater: impl FnOnce(&mut T)) {
        let internal = unsafe { self.internal() };
        internal.mutate(mutater);
    }

    #[inline(always)]
    pub fn update(&self, updater: impl FnOnce(&T) -> T) {
        self.mutate(|data| *data = updater(data))
    }

    #[inline(always)]
    pub fn set(&self, data: T) {
        self.mutate(move |old| *old = data)
    }

    /* map, filter, reduce, ... */
}

impl<T: 'static> Subscribable<T> for Signal<T> {
    fn subscribe(&self, notify: impl FnMut(&T) + 'static) -> Unsubscriber {
        let internal = unsafe { self.internal() };
        let id = internal.subscribe(Box::new(notify));
        let weak = Rc::downgrade(&self.0);

        Unsubscriber::new(move || {
            if let Some(signal) = weak.upgrade().map(Self) {
                let internal = unsafe { signal.internal() };
                internal.unsubscribe(id);
            }
        })
    }
}

#[must_use]
#[repr(transparent)]
pub struct Unsubscriber(Option<Box<dyn FnOnce()>>);

impl Unsubscriber {
    fn new(action: impl FnOnce() + 'static) -> Self {
        Self(Some(Box::new(action)))
    }

    pub fn unsubscribe(&mut self) {
        if let Some(action) = self.0.take() {
            action();
        }
    }

    #[inline(always)]
    pub fn droppable(self) -> DropUnsubscriber {
        DropUnsubscriber(self)
    }
}

#[repr(transparent)]
pub struct DropUnsubscriber(Unsubscriber);

impl Drop for DropUnsubscriber {
    fn drop(&mut self) {
        self.0.unsubscribe()
    }
}
