use core::mem;
use core::ops::{Deref, DerefMut};

use super::Result;

pub trait Value<T> {
    type Unsubscriber;

    fn for_each<F>(&self, f: F) -> Self::Unsubscriber
    where
        F: FnMut(&T) + 'static;

    fn for_each_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &mut Self::Unsubscriber) + 'static;

    #[inline]
    fn for_each_forever<F>(&self, f: F)
    where
        F: FnMut(&T) + 'static,
    {
        let _ = self.for_each(f);
    }
}

impl<T> Value<T> for T {
    type Unsubscriber = ();

    #[inline]
    fn for_each<F>(&self, f: F) -> Self::Unsubscriber
    where
        F: FnOnce(&T),
    {
        f(self);
    }

    #[inline]
    fn for_each_inner<F>(&self, f: F)
    where
        F: FnOnce(&T, &mut Self::Unsubscriber),
    {
        f(self, &mut ());
    }

    #[inline]
    fn for_each_forever<F>(&self, f: F)
    where
        F: FnOnce(&T),
    {
        f(self);
    }
}

pub trait Signal: Value<Self::Item> {
    type Item;

    fn try_get(&self) -> Result<Self::Item>
    where
        Self::Item: Clone;

    #[inline]
    fn get(&self) -> Self::Item
    where
        Self::Item: Clone,
    {
        self.try_get().unwrap()
    }

    // fn map<B, F>(&self, f: F) -> Computed<B>
    // where
    //     F: FnMut(&Self::Item) -> B + 'static;

    // fn filter<P>(&self, predicate: P) -> Computed<Self::Item>
    // where
    //     P: FnMut(&Self::Item) -> bool;

    // fn filter_map<B, F>(&self, f: F) -> Computed<B>
    // where
    //     F: FnMut(&Self::Item) -> Option<B>;

    // fn fold<B, F>(&self, init: B, f: F) -> Computed<B>
    // where
    //     F: FnMut(&mut B, &Self::Item);

    // fn map_while<B, P>(&self, predicate: P) -> Computed<B>
    // where
    //     P: FnMut(&Self::Item) -> Option<B>;

    // fn skip(&self, n: usize) -> Computed<Self::Item>;

    // fn skip_while<P>(&self, predicate: P) -> Computed<Self::Item>
    // where
    //     P: FnMut(&Self::Item) -> bool;
}

pub trait Unsubscribe {
    #[inline]
    fn unsubscribe(&mut self) {}

    #[inline]
    fn has_effect(&self) -> bool {
        false
    }

    #[inline]
    fn droppable(self) -> DropUnsubscriber<Self>
    where
        Self: Sized,
    {
        DropUnsubscriber(self)
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct DropUnsubscriber<U: Unsubscribe>(pub U);

impl<U: Unsubscribe> DropUnsubscriber<U> {
    #[inline]
    pub fn take(self) -> U {
        // SAFETY: `Self` and `U` have the same `repr`.
        let inner = unsafe { mem::transmute_copy(&self) };
        mem::forget(self);
        inner
    }
}

impl<U: Unsubscribe> Deref for DropUnsubscriber<U> {
    type Target = U;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<U: Unsubscribe> DerefMut for DropUnsubscriber<U> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<U: Unsubscribe> Drop for DropUnsubscriber<U> {
    #[inline]
    fn drop(&mut self) {
        self.unsubscribe()
    }
}

impl Unsubscribe for () {}
