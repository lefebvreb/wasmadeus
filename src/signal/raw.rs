use core::cell::{Cell, UnsafeCell};

use alloc::boxed::Box;
use alloc::rc::Rc;
use smallvec::SmallVec;

/// Permitted operation matrix:
/// 
/// |               | `mutate()` | `subscribe()` | `unsubscribe()` |
/// |---------------|------------|---------------|-----------------|
/// | `Idling`      | ✅         | ✅            | ✅              |
/// | `Mutating`    | ❌         | ❌            | ❌              |
/// | `Subscribing` | ❌         | ✅            | ✅              |
#[derive(Copy, Clone, PartialEq, Default, Debug)]
enum SignalState {
    #[default]
    Idling,
    Mutating,
    Subscribing,
}

type Subscriber = Box<dyn FnMut(*const ())>;

#[derive(Default)]
struct RawSignal {
    state: Cell<SignalState>,
    subscribers: UnsafeCell<SmallVec<[Option<Subscriber>; 1]>>,
    garbage: UnsafeCell<SmallVec<[usize; 1]>>,
}

impl RawSignal {
    fn notify_all(&self, data: *const ()) {
        let subscribers = self.subscribers.get();
        let mut i = 0;

        while let Some(maybe_subscriber) = unsafe { (*subscribers).get_mut(i) } {
            if let Some(subscriber) = maybe_subscriber {
                subscriber(data);
            }
            
            i += 1;
        }
    }

    fn mutate(&self, data: *mut (), mutater: impl FnOnce(*mut ())) {
        assert_eq!(self.state.get(), SignalState::Idling);
        self.state.set(SignalState::Mutating);

        mutater(data);
        self.notify_all(data);

        self.state.set(SignalState::Idling);
    }
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
