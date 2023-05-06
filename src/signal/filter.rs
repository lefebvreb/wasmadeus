use alloc::rc::{Rc, Weak};

use super::raw::{RawFilter, RawFilteredUnsubscriber, SubscriberId};
use super::{DropUnsubscriber, Map, Result, Signal, Unsubscribe, Value};

#[repr(transparent)]
pub struct Filter<T: 'static>(Rc<RawFilter<T>>);

impl<T> Filter<T> {
    fn for_each<F>(&self, f: F) -> FilterUnsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        let id = self.0.raw_for_each(|_| f);
        FilterUnsubscriber::new(Rc::downgrade(&self.0), id)
    }

    fn for_each_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &mut FilterUnsubscriber<T>) + 'static,
    {
        let weak = Rc::downgrade(&self.0);
        self.0.raw_for_each(|id| {
            let mut unsub = FilterUnsubscriber::new(weak, id);
            move |data| f(data, &mut unsub)
        });
    }

    #[inline]
    fn for_each_forever<F>(&self, f: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.0.raw_for_each(|_| f);
    }
}

impl<T> Clone for Filter<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Value<T> for &Filter<T> {
    type Unsubscriber = FilterUnsubscriber<T>;

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
        self.for_each_forever(f);
    }
}

impl<T> Signal for Filter<T> {
    type Item = T;

    #[inline]
    fn try_get(&self) -> Result<Self::Item>
    where
        Self::Item: Clone,
    {
        self.0.try_get()
    }

    fn map<B, F>(&self, f: F) -> Map<B>
    where
        F: FnMut(&Self::Item) -> B + 'static,
    {
        todo!()
    }
}

#[must_use]
#[repr(transparent)]
pub struct FilterUnsubscriber<T>(RawFilteredUnsubscriber<T>);

impl<T> FilterUnsubscriber<T> {
    #[inline]
    fn new(weak: Weak<RawFilter<T>>, id: SubscriberId) -> Self {
        Self(RawFilteredUnsubscriber::new(weak, id))
    }

    #[inline]
    pub fn unsubscribe(&mut self) {
        self.0.unsubscribe()
    }

    #[inline]
    pub fn has_effect(&self) -> bool {
        self.0.has_effect()
    }

    #[inline]
    pub fn droppable(self) -> DropUnsubscriber<Self> {
        DropUnsubscriber(self)
    }
}

impl<T> Unsubscribe for FilterUnsubscriber<T> {
    #[inline]
    fn unsubscribe(&mut self) {
        self.unsubscribe()
    }

    #[inline]
    fn has_effect(&self) -> bool {
        self.has_effect()
    }
}

impl<T> Clone for FilterUnsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
