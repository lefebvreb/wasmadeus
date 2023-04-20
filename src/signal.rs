use core::cell::{Cell, UnsafeCell};

use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;

type Notifier = Box<dyn FnMut()>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SignalMutatingError;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
enum SignalState {
    #[default]
    Idling,
    Mutating,
    Subscribing,
}

#[derive(Default)]
struct RawSignal {
    state: Cell<SignalState>,
    subscribers: UnsafeCell<Vec<Option<Notifier>>>,
    garbage: UnsafeCell<Vec<usize>>,
}

impl RawSignal {
    /// Calls each of the subscribers with a reference to the new data.
    /// 
    /// # Safety
    /// 
    /// The data should not be mutated during execution of this function.
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

    fn try_mutate(&self, mutater: impl FnOnce()) -> Result<(), SignalMutatingError> {
        if self.state.get() != SignalState::Idling {
            return Err(SignalMutatingError);
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

pub struct InnerSignal<T>(RawSignal, UnsafeCell<T>);

impl<T> InnerSignal<T> {
    #[inline(always)]
    fn get(&self) -> (&RawSignal, *mut T) {
        (&self.0, self.1.get())
    }
}

pub struct Signal<T>(Rc<InnerSignal<T>>);

impl<T: 'static> Signal<T> {
    pub fn new(data: T) -> Self {
        let raw = RawSignal::default();
        let data = UnsafeCell::new(data);
        Self(Rc::new(InnerSignal(raw, data)))
    }

    fn try_mutate(&self, mutater: impl FnOnce(&mut T)) -> Result<(), SignalMutatingError> {
        let (raw, data) = self.0.get();
        raw.try_mutate(|| mutater(unsafe { &mut *data }))
    }

    #[inline(always)]
    pub fn mutate(&self, mutater: impl FnOnce(&mut T)) {
        self.try_mutate(mutater).unwrap();
    }

    pub fn try_replace(&self, replacer: impl FnOnce(T) -> T) -> Result<(), SignalMutatingError> {
        let (raw, data) = self.0.get();
        raw.try_mutate(|| unsafe { data.write(replacer(data.read())) })
    }

    #[inline(always)]
    pub fn replace(&self, replacer: impl FnOnce(T) -> T) {
        self.try_replace(replacer).unwrap();
    }

    fn subscribe(&self, mut notify: impl FnMut(&T) + 'static) -> usize {
        let (raw, data) = self.0.get();
        let notify = move || notify(unsafe { &*data });
        raw.subscribe(Box::new(notify))
    }

    #[inline(always)]
    fn unsubscribe(&self, id: usize) {
        let (raw, _) = self.0.get();
        raw.unsubscribe(id);
    }
}

impl<T> Clone for Signal<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct SignalUnsubscriber<T>(Option<Weak<InnerSignal<T>>>);

impl<T> SignalUnsubscriber<T> {
    
}

pub trait Subscribable<T> {
    fn subscribe(&self, notify: impl FnMut(&T)) -> SignalUnsubscriber<T>;
}

impl<T> Subscribable<T> for T {
    fn subscribe(&self, mut notify: impl FnMut(&T)) -> SignalUnsubscriber<T> {
        notify(self);
        SignalUnsubscriber(None)
    }
}

impl<T> Subscribable<T> for Signal<T> {
    fn subscribe(&self, notify: impl FnMut(&T)) -> SignalUnsubscriber<T> {
        todo!()
    }
}
