use core::cell::{Cell, UnsafeCell};

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;

type Subscriber = Box<dyn FnMut(*const ())>;

#[derive(Default)]
struct RawSignal {
    notifying: Cell<bool>,
    subscribers: UnsafeCell<Vec<Subscriber>>,
    garbage: UnsafeCell<Vec<usize>>,
}

impl RawSignal {
    /// Calls each of the subscribers with a reference to the new data.
    /// 
    /// # Safety
    /// 
    /// The data should not be mutated by one of the subscriber's callback.
    unsafe fn notify_all(&self, data: *const ()) {
        let mut i = 0;

        while let Some(subscriber) = (*self.subscribers.get()).get_mut(i) {
            subscriber(data);
            i += 1;
        }
    }

    fn mutate(&self, data: *mut (), mutater: impl FnOnce(*mut ())) {
        assert!(!self.notifying.get());
        self.notifying.set(true);

        mutater(data);

        // SAFETY: we set the state to notifying, preventing a second call to this
        // function until this one terminates.
        unsafe { self.notify_all(data) };

        self.notifying.set(false);
    }

    fn subscribe(&self, data: *const (), mut notify: Subscriber) -> usize {
        // If the signal is not already notifying, call the new closure.
        if !self.notifying.get() {
            notify(data);
        }

        let subscribers = unsafe { &mut *self.subscribers.get() };
        let id = subscribers.len();
        subscribers.push(notify);

        id
    }

    // fn unsubscribe(&self, id: usize) {
    //     if self.mutating.get() == SignalState::Mutating {
    //         // If the state is Mutating, the remove operation is delayed until
    //         // after the mutation.

    //         // SAFETY: a borrow to self.garbage is never kept between calls.
    //         let garbage = unsafe { &mut *self.garbage.get() };
    //         garbage.push(id);
    //     } else {

    //     }
    //     todo!()
    // }
}

pub struct Signal<T>(Rc<(RawSignal, UnsafeCell<T>)>);

impl<T> Signal<T> {
    pub fn new(data: T) -> Self {
        let raw = RawSignal::default();
        let data = UnsafeCell::new(data);
        Self(Rc::new((raw, data)))
    }

    pub fn mutate(&self, mutater: impl FnOnce(&mut T)) {
        let mutater = |data: *mut ()| {
            let data = unsafe { &mut *(data as *mut T) };
            mutater(data);
        };
    }

    fn subscribe(&self, mut notify: impl FnMut(&T)) -> usize {
        let notify = |data: *const ()| {
            let data = unsafe { &*(data as *const T) };
            notify(data);
        };

        todo!()
    }

    fn unsubscribe(&self, index: usize) {}
}
