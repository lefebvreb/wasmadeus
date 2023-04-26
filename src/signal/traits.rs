use super::{SignalMutatingError, Unsubscriber};

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

    fn try_get(&self) -> Result<Self::Item, SignalMutatingError>
    where
        Self::Item: Clone;

    fn get(&self) -> Self::Item
    where
        Self::Item: Clone,
    {
        self.try_get().unwrap()
    }

    fn try_take(&self) -> Result<Self::Item, SignalMutatingError>
    where
        Self::Item: Default;

    fn take(&self) -> Self::Item
    where
        Self::Item: Default,
    {
        self.try_take().unwrap()
    }

    fn map<T, F>(&self, f: F)
    where
        F: FnMut(&Self::Item) -> T;
}
