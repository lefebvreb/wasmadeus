use alloc::rc::{Rc, Weak};

use super::raw::{RawMutable, RawMutableUnsubscriber, SubscriberId};
use super::{Result, Signal, Unsubscribe, Value};

#[repr(transparent)]
pub struct Mutable<T: 'static>(Rc<RawMutable<T>>);

impl<T> Mutable<T> {
    #[inline]
    fn new_from_raw(raw: RawMutable<T>) -> Self {
        Self(Rc::new(raw))
    }

    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self::new_from_raw(RawMutable::new(initial_value))
    }

    #[inline]
    pub fn uninit() -> Self {
        Self::new_from_raw(RawMutable::uninit())
    }

    #[inline]
    pub fn try_set(&self, new_value: T) -> Result<()> {
        self.0.try_set(new_value)
    }

    #[inline]
    pub fn set(&self, new_value: T) {
        self.try_set(new_value).unwrap()
    }

    #[inline]
    pub fn try_mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        self.0.try_mutate(mutate)
    }

    #[inline]
    pub fn mutate<F>(&self, mutate: F)
    where
        F: FnOnce(&mut T),
    {
        self.try_mutate(mutate).unwrap();
    }

    #[inline]
    pub fn try_update<F>(&self, update: F) -> Result<()>
    where
        F: FnOnce(&T) -> T,
    {
        self.try_mutate(|data| *data = update(data))
    }

    #[inline]
    pub fn update<F>(&self, update: F)
    where
        F: FnOnce(&T) -> T,
    {
        self.try_update(update).unwrap();
    }

    pub fn for_each<F>(&self, f: F) -> MutableUnsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        let id = self.0.raw_for_each(|_| f);
        MutableUnsubscriber::new(Rc::downgrade(&self.0), id)
    }

    pub fn for_each_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &mut MutableUnsubscriber<T>) + 'static,
    {
        let weak = Rc::downgrade(&self.0);
        self.0.raw_for_each(|id| {
            let mut unsub = MutableUnsubscriber::new(weak, id);
            move |data| f(data, &mut unsub)
        });
    }

    #[inline]
    pub fn for_each_forever<F>(&self, f: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.0.raw_for_each(|_| f);
    }
}

impl<T> Value<T> for &Mutable<T> {
    type Unsubscriber = MutableUnsubscriber<T>;

    #[inline]
    fn for_each<F>(self, f: F) -> Self::Unsubscriber
    where
        F: FnMut(&T) + 'static,
    {
        self.for_each(f)
    }

    #[inline]
    fn for_each_inner<F>(self, f: F)
    where
        F: FnMut(&T, &mut Self::Unsubscriber) + 'static,
    {
        self.for_each_inner(f)
    }

    #[inline]
    fn for_each_forever<F>(self, f: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.for_each_forever(f)
    }
}

impl<T> Signal for Mutable<T> {
    type Item = T;

    #[inline]
    fn try_get(&self) -> Result<Self::Item>
    where
        Self::Item: Clone,
    {
        self.0.try_get()
    }
}

impl<T> Clone for Mutable<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[must_use]
#[repr(transparent)]
pub struct MutableUnsubscriber<T>(RawMutableUnsubscriber<T>);

impl<T> MutableUnsubscriber<T> {
    #[inline]
    fn new(weak: Weak<RawMutable<T>>, id: SubscriberId) -> Self {
        Self(RawMutableUnsubscriber::new(weak, id))
    }

    #[inline]
    pub fn unsubscribe(&mut self) {
        self.0.unsubscribe()
    }

    #[inline]
    pub fn has_effect(&self) -> bool {
        self.0.has_effect()
    }
}

impl<T> Unsubscribe for MutableUnsubscriber<T> {
    #[inline]
    fn unsubscribe(&mut self) {
        self.unsubscribe()
    }

    #[inline]
    fn has_effect(&self) -> bool {
        self.has_effect()
    }
}

impl<T> Clone for MutableUnsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
