use core::ops::Deref;

use alloc::rc::{Rc, Weak};

use super::raw::{RawSignal, SubscriberId};
use super::Result;

#[repr(transparent)]
pub struct Signal<T: 'static>(Rc<RawSignal<T>>);

impl<T> Signal<T> {
    #[inline]
    fn new_from_raw(raw: RawSignal<T>) -> Self {
        Self(Rc::new(raw))
    }

    #[inline]
    fn new_derived(&self) -> Self {
        Self::new_from_raw(RawSignal::new_derived(&self.0))
    }

    #[inline]
    fn inner(&self) -> &Rc<RawSignal<T>> {
        &self.0
    }

    #[inline]
    pub fn try_get(&self) -> Result<T>
    where
        T: Clone,
    {
        self.0.try_get()
    }

    pub fn for_each<F>(&self, notify: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        let id = self.inner().raw_for_each(|_| notify);
        Unsubscriber::new(Rc::downgrade(self.inner()), id)
    }

    pub fn for_each_inner<F>(&self, mut notify: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static,
    {
        let weak = Rc::downgrade(self.inner());
        self.inner().raw_for_each(|id| {
            let mut unsub = Unsubscriber::new(weak, id);
            move |data| notify(data, &mut unsub)
        });
    }

    #[inline]
    pub fn for_each_forever<F>(&self, notify: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.inner().raw_for_each(|_| notify);
    }

    pub fn map<B, F>(&self, map: F) -> Signal<B>
    where
        F: FnMut(&T) -> B + 'static,
    {
        todo!()
    }

    pub fn filter<P>(&self, predicate: P) -> Signal<T>
    where
        P: FnMut(&T) -> bool,
    {
        todo!()
    }
}

impl<T> Clone for Signal<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[repr(transparent)]
pub struct Mutable<T: 'static>(Signal<T>);

impl<T> Mutable<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self(Signal::new_from_raw(RawSignal::new(initial_value)))
    }

    #[inline]
    pub fn uninit() -> Self {
        Self(Signal::new_from_raw(RawSignal::uninit()))
    }

    #[inline]
    pub fn try_set(&self, new_value: T) -> Result<()> {
        self.inner().try_set(new_value)
    }

    #[inline]
    pub fn try_mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        self.inner().try_mutate(mutate)
    }

    // #[inline]
    // pub fn try_set(&self, new_value: T) -> Result<()> {
    //     let value = self.inner().value.get();

    //     match self.inner().state() {
    //         State::Idling => unsafe {
    //             *(*value).assume_init_mut().get() = new_value;
    //         },
    //         State::Uninit => unsafe {
    //             let rc = Rc::new(UnsafeCell::new(new_value));
    //             *value = MaybeUninit::new(rc);
    //         },
    //         _ => return Err(SignalError),
    //     }

    //     unsafe {
    //         let value = self.inner().value();
    //         self.inner().broadcast.notify_with(|| value);
    //     }

    //     Ok(())
    // }

    // #[inline]
    // fn try_mutate<F>(&self, mutate: F) -> Result<()>
    // where
    //     F: FnOnce(&mut T),
    // {
    //     if self.state() != State::Idling {
    //         return Err(SignalError);
    //     }

    //     unsafe {
    //         let value = self.value();
    //         self.broadcast.notify_with(|| {
    //             mutate(value.cast().as_mut());
    //             value
    //         });
    //     }

    //     Ok(())
    // }
}

impl<T> Clone for Mutable<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for Mutable<T> {
    type Target = Signal<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl<T> Signal<T> {
//     // #[inline]
//     // pub fn new(initial_value: T) -> Self {
//     //     let storage = UnsafeCell::new(MaybeUninit::new(initial_value));
//     //     Self::new_with_state(storage, State::Idling)
//     // }

//     // #[inline]
//     // pub fn uninit() -> Self {
//     //     let storage = UnsafeCell::new(MaybeUninit::uninit());
//     //     Self::new_with_state(storage, State::Uninit)
//     // }

//     pub fn try_set(&self, new_value: T) -> Result<()> {
//         let value = self.value.get();

//         match self.state() {
//             State::Idling => unsafe {
//                 *(*value).assume_init_mut() = new_value;
//             },
//             State::Uninit => unsafe {
//                 (*value).write(new_value);
//             },
//             _ => return Err(SignalError),
//         }

//         self.set_state(State::Mutating);
//         unsafe { self.inner.notify_all(self.value()) };
//         self.set_state(State::Idling);

//         Ok(())
//     }

//     #[inline]
//     pub fn try_mutate<F>(&self, mutate: F) -> Result<()>
//     where
//         F: FnOnce(&mut T),
//     {
//         if self.state() != State::Idling {
//             return Err(SignalError);
//         }

//         self.set_state(State::Mutating);
//         unsafe {
//             let value = self.value();
//             mutate(value.cast().as_mut());
//             self.inner.notify_all(value);
//         }
//         self.set_state(State::Idling);

//         Ok(())
//     }
// }

// impl<T> RawFilter<T> {
//     // pub fn from_mutable<F>(signal: &RawMutable<T>, filter: F) -> Self
//     // where
//     //     F: FnMut(&T) -> bool,
//     // {
//     //     let state = match signal.state() {
//     //         State::Mutating => State::Mutating,
//     //         _ => State::Idling,
//     //     };
//     //     let this = Self::new_with_state(signal.value().cast(), state);
//     //     // todo(benjamin): for_each_inner and filter, using this.notify_all()
//     //     todo!()
//     // }

//     // unsafe fn notify_all(&self) {
//     //     self.set_state(State::Mutating);
//     //     unsafe { self.inner.notify_all(self.value()) };
//     //     self.set_state(State::Idling);
//     // }
// }

// impl<T> Drop for Signal<T> {
//     #[inline]
//     fn drop(&mut self) {
//         if self.is_initialized() {
//             // SAFETY: the data is initialized and valid.
//             unsafe { self.storage.drop() };
//         }
//     }
// }

#[repr(transparent)]
pub struct Unsubscriber<T>(Option<(Weak<RawSignal<T>>, SubscriberId)>);

impl<T> Unsubscriber<T> {
    #[inline]
    fn new(weak: Weak<RawSignal<T>>, id: SubscriberId) -> Self {
        Self(Some((weak, id)))
    }

    pub fn unsubscribe(&mut self) {
        if let Some((weak, id)) = self.0.take() {
            if let Some(raw) = weak.upgrade() {
                raw.unsubscribe(id);
            }
        }
    }

    #[inline]
    pub fn has_effect(&self) -> bool {
        self.0.is_some()
    }
}

impl<T> Clone for Unsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
