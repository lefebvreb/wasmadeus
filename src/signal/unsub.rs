use core::mem;
use core::ops::{Deref, DerefMut};

use alloc::rc::Weak;

use super::raw::{RawSignal, SubscriberId};

#[must_use]
#[repr(transparent)]
pub struct SignalUnsubscriber<T>(Option<(Weak<RawSignal<T>>, SubscriberId)>);

impl<T> SignalUnsubscriber<T> {
    #[inline]
    pub(super) fn new(weak: Weak<RawSignal<T>>, id: SubscriberId) -> Self {
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

    #[inline]
    pub fn droppable(self) -> DropUnsubscriber<Self> {
        DropUnsubscriber(self)
    }
}

impl<T> Clone for SignalUnsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct DropUnsubscriber<U: Unsubscribe>(pub U);

impl<U: Unsubscribe> DropUnsubscriber<U> {
    #[inline]
    pub fn into_inner(self) -> U {
        // SAFETY: Self has the same layout as U because it is marked as #[repr(transparent)].
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

impl Unsubscribe for () {}

impl<T> Unsubscribe for SignalUnsubscriber<T> {
    #[inline]
    fn unsubscribe(&mut self) {
        self.unsubscribe();
    }

    #[inline]
    fn has_effect(&self) -> bool {
        self.has_effect()
    }
}
