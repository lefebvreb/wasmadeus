use core::cell::{Cell, UnsafeCell};

use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SignalUpdatingError;

type Notifier = Box<dyn FnMut()>;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
enum SignalState {
    #[default]
    /// The signal is not currently in use.
    Idling,
    /// The signal's data is currently being updated and/or
    /// its subscribers are being notified.
    Mutating,
    /// The signal is currently notifying new susbcribers with
    /// a reference to the current data.
    Subscribing,
}

#[derive(Default)]
struct RawSignal {
    state: Cell<SignalState>,
    subscribers: UnsafeCell<Vec<Option<Notifier>>>,
    garbage: UnsafeCell<Vec<usize>>,
}

impl RawSignal {
    unsafe fn notify_all(&self) {
        let subscribers = self.subscribers.get();
        let mut i = 0;

        while i < (*subscribers).len() {
            // SAFETY: for each element of the subscribers vec,
            // we get a mutable reference to the content of the box
            // containing the closure we need to call, without borrowing the father
            // vector. This is safe because `Box<T>` is always Unpin, and 
            // the only allowed modification of a vector during mutation of
            // a signal is appending to it.

            (*subscribers)
                .as_mut_ptr()
                .offset(i as isize)
                .as_mut()
                .unwrap()
                .as_mut()
                .map(|notifier| notifier());

            i += 1;
        }
    }

    fn try_mutate(&self, mutater: impl FnOnce()) -> Result<(), SignalUpdatingError> {
        if self.state.get() != SignalState::Idling {
            return Err(SignalUpdatingError);
        }
        self.state.set(SignalState::Mutating);

        mutater();

        // SAFETY: we set the state to Mutating, therefore preventing 
        // a second call to this function until this one terminates.
        unsafe { self.notify_all() };

        self.state.set(SignalState::Idling);
        Ok(())
    }

    fn subscribe(&self, mut notify: Notifier) -> usize {
        if self.state.get() != SignalState::Mutating {
            let old_state = self.state.replace(SignalState::Subscribing);
            notify();
            self.state.set(old_state);
        }

        let subscribers = unsafe { &mut *self.subscribers.get() };
        let garbage = unsafe { &mut *self.garbage.get() };
        
        if let Some(id) = garbage.pop() {
            subscribers[id] = Some(notify);
            id
        } else {
            let id = subscribers.len();
            subscribers.push(Some(notify));
            id
        }
    }

    fn unsubscribe(&self, id: usize) {
        let subscribers = unsafe { &mut *self.subscribers.get() };
        let garbage = unsafe { &mut *self.garbage.get() };
        
        subscribers[id] = None;
        garbage.push(id);
    }
}

struct InnerSignal<T>(RawSignal, UnsafeCell<T>);

impl<T> InnerSignal<T> {
    #[inline(always)]
    fn get(&self) -> (&RawSignal, *mut T) {
        (&self.0, self.1.get())
    }
}

#[repr(transparent)]
pub struct Signal<T>(Rc<InnerSignal<T>>);

impl<T: 'static> Signal<T> {
    pub fn new(value: T) -> Self {
        let raw = RawSignal::default();
        let data = UnsafeCell::new(value);
        Self(Rc::new(InnerSignal(raw, data)))
    }

    #[inline(always)]
    pub fn try_mutate<F>(&self, mutater: F) -> Result<(), SignalUpdatingError>
    where
        F: FnOnce(&mut T),
    {
        let (raw, data) = self.0.get();
        raw.try_mutate(|| mutater(unsafe { &mut *data }))
    }

    #[inline(always)]
    pub fn try_update<F>(&self, updater: F) -> Result<(), SignalUpdatingError>
    where
        F: FnOnce(&T) -> T,
    {
        let (raw, data) = self.0.get();
        raw.try_mutate(|| unsafe { *data = updater(&*data) })
    }

    #[inline(always)]
    pub fn try_set(&self, value: T) -> Result<(), SignalUpdatingError> {
        let (raw, data) = self.0.get();
        raw.try_mutate(|| unsafe { *data = value })
    }

    #[inline(always)]
    pub fn mutate<F>(&self, mutater: impl FnOnce(&mut T))
    where
        F: FnOnce(&mut T),
    {
        self.try_mutate(mutater).unwrap();
    }

    #[inline(always)]
    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&T) -> T,
    {
        self.try_update(updater).unwrap();
    }

    #[inline(always)]
    pub fn set(&self, value: T) {
        self.try_set(value).unwrap();
    }

    pub fn subscribe<F>(&self, mut notify: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        let (raw, data) = self.0.get();
        let notify = move || notify(unsafe { &*data });
        let id = raw.subscribe(Box::new(notify));

        Unsubscriber {
            signal: Some(Rc::downgrade(&self.0)), 
            id 
        }
    }
}

impl<T: Clone> Signal<T> {
    pub fn get(&self) -> T {
        let (_, data) = self.0.get();
        unsafe { (&*data).clone() }
    }
}

impl<T> Clone for Signal<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct Unsubscriber<T> {
    signal: Option<Weak<InnerSignal<T>>>,
    id: usize,
}

impl<T> Unsubscriber<T> {
    pub fn unsubscribe(&mut self) {
        if let Some(weak) = self.signal.take() {
            if let Some(inner) = weak.upgrade() {
                let (raw, _) = inner.get();
                raw.unsubscribe(self.id);
            }
        }
    }

    #[inline(always)]
    pub fn droppable(self) -> DroppableUnsubscriber<T> {
        DroppableUnsubscriber(self)
    }
}

pub struct DroppableUnsubscriber<T>(pub Unsubscriber<T>);

impl<T> Drop for DroppableUnsubscriber<T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.0.unsubscribe();
    }
}

pub trait Subscribable<T> {
    fn subscribe<F>(&self, notify: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static;
}

impl<T> Subscribable<T> for T {
    #[inline(always)]
    fn subscribe<F>(&self, mut notify: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        notify(self);

        Unsubscriber {
            signal: None, 
            id: 0,
        }
    }
}

impl<T: 'static> Subscribable<T> for Signal<T> {
    #[inline(always)]
    fn subscribe<F>(&self, notify: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        self.subscribe(notify)
    }
}
