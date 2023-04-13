use core::cell::UnsafeCell;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;

type Notifier<T> = Box<dyn FnMut(&T)>;

#[inline(always)]
fn id_of<T>(f: &Notifier<T>) -> usize {
    f as *const dyn FnMut(&T) as *const () as usize
}

#[derive(Debug)]
pub struct SignalUpdatingError;

pub trait Subscribable<T> {
    fn subscribe(&self, notify: impl FnMut(&T) + 'static) -> Unsubscriber;
}

struct RawStore<T> {
    data: T,
    subscribers: Vec<Notifier<T>>,
    updating: bool,
}

impl<T> RawStore<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            subscribers: Vec::new(),
            updating: false,
        }
    }

    fn find(&self, id: usize) -> Result<usize, usize> {
        self.subscribers.binary_search_by_key(&id, id_of)
    }

    fn try_mutate(&mut self, mutater: impl FnOnce(&mut T)) -> Result<(), SignalUpdatingError> {
        if self.updating {
            return Err(SignalUpdatingError);
        }

        self.updating = true;

        mutater(&mut self.data);
        
        for subscriber in &mut self.subscribers {
            subscriber(&self.data);
        }

        self.updating = false;

        Ok(())
    }

    fn subscribe(&mut self, mut subscriber: Box<dyn FnMut(&T)>) -> usize {
        let id = id_of(&subscriber);
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
pub struct Signal<T>(Rc<UnsafeCell<RawStore<T>>>);

impl<T> Signal<T> {
    #[inline(always)]
    unsafe fn internal(&self) -> &mut RawStore<T> {
        &mut *self.0.get()
    }

    pub fn new(data: T) -> Self {
        let internal = RawStore::new(data);
        Self(Rc::new(UnsafeCell::new(internal)))
    }

    pub fn get(&self) -> T 
    where
        T: Clone 
    {
        let internal = unsafe { self.internal() };
        internal.data.clone()
    }

    #[inline(always)]
    pub fn try_mutate(&self, mutater: impl FnOnce(&mut T)) -> Result<(), SignalUpdatingError> {
        let internal = unsafe { self.internal() };
        internal.try_mutate(mutater)
    }

    #[inline(always)]
    pub fn mutate(&self, mutater: impl FnOnce(&mut T)) {
        self.try_mutate(mutater).unwrap();
    }

    #[inline(always)]
    pub fn try_update(&self, updater: impl FnOnce(&T) -> T) -> Result<(), SignalUpdatingError> {
        self.try_mutate(|data| *data = updater(data))
    }

    #[inline(always)]
    pub fn update(&self, updater: impl FnOnce(&T) -> T) {
        self.try_update(updater).unwrap()
    }

    #[inline(always)]
    pub fn try_set(&self, data: T) -> Result<(), SignalUpdatingError> {
        self.try_mutate(move |old| *old = data)
    }

    #[inline(always)]
    pub fn set(&self, data: T) {
        self.try_set(data).unwrap();
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

#[must_use]
#[repr(transparent)]
pub struct DropUnsubscriber(Unsubscriber);

impl Drop for DropUnsubscriber {
    fn drop(&mut self) {
        self.0.unsubscribe()
    }
}
