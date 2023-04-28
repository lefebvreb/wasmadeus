use super::{Mutable, Result, Signal, Unsubscriber, Value};

#[derive(Debug)]
#[repr(transparent)]
pub struct Computed<T: 'static>(Mutable<T>);

impl<T> Computed<T> {
    #[inline]
    pub fn uninit() -> Self {
        Self(Mutable::uninit())
    }

    #[inline]
    pub fn as_mutable(&self) -> &Mutable<T> {
        &self.0
    }
}

impl<T> Clone for Computed<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Signal for Computed<T> {
    type Item = T;

    #[inline]
    fn try_get(&self) -> Result<Self::Item>
    where
        Self::Item: Clone,
    {
        self.0.try_get()
    }
}

impl<T> Value<T> for Computed<T> {
    #[inline]
    fn for_each<F>(&self, f: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        self.0.for_each(f)
    }

    #[inline]
    fn for_each_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static,
    {
        self.0.for_each_inner(f)
    }
}
