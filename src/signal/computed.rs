use super::{Mutable, Unsubscriber, Value, Signal, Result};

#[repr(transparent)]
pub struct Computed<T: 'static>(Mutable<T>);

impl<T> Computed<T> {
    /// Returns an uninitialized [`Computed`] signal.
    ///
    /// The observant reader may object that a free, uninitialized computed
    /// signal is useless on its own, as nothing will come to provide it with
    /// a value (unless [`Computed::as_mutable`] is used...), and they would
    /// be right.
    ///
    /// This method is only provided for advanced use cases, for example
    /// when implementing your own monadic operations on signals.
    #[inline]
    pub fn uninit() -> Self {
        Self(Mutable::uninit())
    }

    /// Gives a reference to the underlying [`Mutable`] signal behind
    /// this computed signal.
    ///
    /// * *Wait, it's all mutable ?*
    /// * *Always has been...*
    ///
    /// You should not need this method for normal use cases, however
    /// it can come in handy when fiddling with signals.
    ///
    /// Note that there is nothing unsafe in mutating a computed signal.
    #[inline]
    pub fn as_mutable(&self) -> &Mutable<T> {
        &self.0
    }

    #[inline]
    pub fn for_each<F>(&self, f: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        self.0.for_each(f)
    }

    #[inline]
    pub fn for_each_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static,
    {
        self.0.for_each_inner(f)
    }

    #[inline]
    pub fn for_each_forever<F>(&self, f: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.0.for_each_forever(f)
    }
}

impl<T> Clone for Computed<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Value<T> for &Computed<T> {
    type Unsubscriber = Unsubscriber<T>;

    #[inline]
    fn for_each<F>(self, f: F) -> Self::Unsubscriber
    where
        F: FnMut(&T) + 'static,
    {
        self.0.for_each(f)
    }

    #[inline]
    fn for_each_inner<F>(self, f: F)
    where
        F: FnMut(&T, &mut Self::Unsubscriber) + 'static, 
    {
        self.0.for_each_inner(f)
    }

    #[inline]
    fn for_each_forever<F>(self, f: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.0.for_each_forever(f)
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
