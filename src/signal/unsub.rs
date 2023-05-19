use core::ops::{Deref, DerefMut};

use alloc::rc::Weak;

use super::raw::{RawSignal, SubscriberId};

#[must_use]
#[repr(transparent)]
pub struct Unsubscriber<T>(Option<(Weak<RawSignal<T>>, SubscriberId)>);

impl<T> Unsubscriber<T> {
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
    pub fn droppable(self) -> DropUnsubscriber<T> {
        DropUnsubscriber(self)
    }
}

impl<T> Clone for Unsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[repr(transparent)]
pub struct DropUnsubscriber<T>(pub Unsubscriber<T>);

impl<T> DropUnsubscriber<T> {
    #[inline]
    pub fn take(mut self) -> Unsubscriber<T> {
        Unsubscriber(self.0 .0.take())
    }
}

impl<T> Clone for DropUnsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for DropUnsubscriber<T> {
    type Target = Unsubscriber<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DropUnsubscriber<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for DropUnsubscriber<T> {
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
}

impl Unsubscribe for () {}

impl<T> Unsubscribe for Unsubscriber<T> {
    #[inline]
    fn unsubscribe(&mut self) {
        self.unsubscribe();
    }

    #[inline]
    fn has_effect(&self) -> bool {
        self.has_effect()
    }
}
