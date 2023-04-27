use super::{Result, Unsubscriber, Computed};

pub trait Value<T> {
    fn for_each<F>(&self, f: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static;

    fn for_each_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static;
}

impl<T> Value<T> for T {
    #[inline]
    fn for_each<F>(&self, mut f: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        f(self);
        Unsubscriber::empty()
    }

    fn for_each_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static,
    {
        let mut unsub = Unsubscriber::empty();
        f(self, &mut unsub);
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

    fn map<B, F>(&self, f: F) -> Computed<B>
    where
        F: FnMut(&Self::Item) -> B;
}
