use core::cell::{Cell, UnsafeCell};

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;

type Notifier = Box<dyn FnMut(*const ())>;

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
    subscribers: UnsafeCell<Vec<Notifier>>,
    garbage: UnsafeCell<Vec<usize>>,
}

impl RawSignal {
    /// Calls each of the subscribers with a reference to the new data.
    /// 
    /// # Safety
    /// 
    /// The data should not be mutated during execution of this function.
    unsafe fn notify_all(&self, data: *const ()) {
        let subscribers = self.subscribers.get();
        let mut i = 0;

        while i < (*subscribers).len() {
            // SAFETY: for each element of the subscribers vec,
            // we get a mutable reference to the content of the box
            // containing the closure we need to call, without borrowing the father
            // vector. This is safe because `Box<T>` is always Unpin, and 
            // the only allowed modification of a vector during mutation of
            // a signal is appending to it.

            let notifier = (*subscribers)
                .as_mut_ptr()
                .offset(i as isize)
                .as_mut()
                .unwrap()
                .as_mut();

            notifier(data);

            i += 1;
        }

        self.cleanup_garbage();
    }

    unsafe fn cleanup_garbage(&self) {
        let subscribers = &mut *self.subscribers.get();
        let garbage = &mut *self.garbage.get();

        while let Some(id) = garbage.pop() {
            let _ = (*subscribers).swap_remove(id);
        }
    }

    fn mutate(&self, data: *mut (), mutater: impl FnOnce(*mut ())) {
        assert_eq!(self.state.get(), SignalState::Idling);
        self.state.set(SignalState::Mutating);

        mutater(data);

        // SAFETY: we set the state to Mutating, therefore preventing 
        // a second call to this function until this one terminates.
        unsafe { self.notify_all(data) };

        self.state.set(SignalState::Idling);
    }

    fn subscribe(&self, data: *const (), mut notify: Notifier) -> usize {
        let old_state = self.state.get();

        // If the signal is not Mutating,
        if old_state != SignalState::Mutating {
            self.state.set(SignalState::Subscribing);
            notify(data);
            self.state.set(old_state);
        }

        let subscribers = unsafe { &mut *self.subscribers.get() };
        let id = subscribers.len();
        subscribers.push(notify);

        if old_state == SignalState::Idling {
            unsafe { self.cleanup_garbage(); }
        }

        id
    }

    fn unsubscribe(&self, id: usize) {
        if self.state.get() == SignalState::Idling {
            let subscribers = unsafe { &mut *self.subscribers.get() };
            let _ = subscribers.swap_remove(id);
        } else {
            let garbage = unsafe { &mut *self.garbage.get() };
            garbage.push(id);
        }
    }
}

pub struct Signal<T>(Rc<(RawSignal, UnsafeCell<T>)>);

impl<T> Signal<T> {
    pub fn new(data: T) -> Self {
        let raw = RawSignal::default();
        let data = UnsafeCell::new(data);
        Self(Rc::new((raw, data)))
    }

    fn inner(&self) -> (&RawSignal, *mut ()) {
        (&self.0.0, self.0.1.get() as *mut ())
    }

    pub fn mutate(&self, mutater: impl FnOnce(&mut T)) {
        let (raw, data) = self.inner();
        let mutater = |data| mutater(unsafe { &mut *(data as *mut T) });
        raw.mutate(data, mutater);
    }

    fn subscribe(&self, mut notify: impl FnMut(&T) + 'static) -> usize {
        let (raw, data) = self.inner();
        let notify = move |data| notify(unsafe { &*(data as *const T) });
        raw.subscribe(data, Box::new(notify))
    }

    fn unsubscribe(&self, index: usize) {

    }
}
