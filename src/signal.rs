use core::cell::{Cell, UnsafeCell};

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;

type Notifier = Box<dyn FnMut()>;

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

    fn mutate(&self, mutater: impl FnOnce()) {
        assert_eq!(self.state.get(), SignalState::Idling);
        self.state.set(SignalState::Mutating);

        mutater();

        // SAFETY: we set the state to Mutating, therefore preventing 
        // a second call to this function until this one terminates.
        unsafe { self.notify_all() };

        self.state.set(SignalState::Idling);
    }

    fn subscribe(&self, mut notify: Notifier) -> usize {
        // If the signal is not Mutating,
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

pub struct Signal<T>(Rc<(RawSignal, UnsafeCell<T>)>);

impl<T: 'static> Signal<T> {
    pub fn new(data: T) -> Self {
        let raw = RawSignal::default();
        let data = UnsafeCell::new(data);
        Self(Rc::new((raw, data)))
    }

    fn inner(&self) -> (&RawSignal, *mut T) {
        (&self.0.0, self.0.1.get())
    }

    pub fn mutate(&self, mutater: impl FnOnce(&mut T)) {
        let (raw, data) = self.inner();
        raw.mutate(|| mutater(unsafe { &mut *data }));
    }

    fn subscribe(&self, mut notify: impl FnMut(&T) + 'static) -> usize {
        let (raw, data) = self.inner();
        let notify = move || notify(unsafe { &*data });
        raw.subscribe(Box::new(notify))
    }

    fn unsubscribe(&self, id: usize) {
        let (raw, _) = self.inner();
        raw.unsubscribe(id);
    }
}

pub struct Unsubscriber;

pub trait Subscribable {
    
}
