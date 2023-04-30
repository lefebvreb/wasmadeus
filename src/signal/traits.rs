use super::{Computed, Result, Unsubscriber};

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
        F: FnMut(&Self::Item) -> B + 'static;

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
