use super::{Unsubscriber, Signal, Mutable};

pub trait Value<T>: Sized {
    type Unsubscriber;

    fn for_each<F>(self, notify: F) -> Self::Unsubscriber
    where
        F: FnMut(&T) + 'static;

    fn for_each_inner<F>(self, notify: F)
    where
        F: FnMut(&T, &mut Self::Unsubscriber) + 'static;

    #[inline]
    fn for_each_forever<F>(self, notify: F)
    where
        F: FnMut(&T) + 'static,
    {
        let _ = self.for_each(notify);
    }
}

impl<T> Value<T> for &T {
    type Unsubscriber = ();

    #[inline]
    fn for_each<F>(self, notify: F) -> Self::Unsubscriber
    where
        F: FnOnce(&T),
    {
        notify(self);
    }

    #[inline]
    fn for_each_inner<F>(self, notify: F)
    where
        F: FnOnce(&T, &mut Self::Unsubscriber),
    {
        notify(self, &mut ());
    }

    #[inline]
    fn for_each_forever<F>(self, notify: F)
    where
        F: FnOnce(&T),
    {
        notify(self);
    }
}

impl<T: Copy> Value<T> for T {
    type Unsubscriber = ();

    #[inline]
    fn for_each<F>(self, notify: F) -> Self::Unsubscriber
    where
        F: FnOnce(&T),
    {
        notify(&self)
    }

    #[inline]
    fn for_each_inner<F>(self, notify: F)
    where
        F: FnOnce(&T, &mut Self::Unsubscriber),
    {
        notify(&self, &mut ())
    }

    #[inline]
    fn for_each_forever<F>(self, notify: F)
    where
        F: FnOnce(&T),
    {
        notify(&self);
    }
}

impl<T> Value<T> for &Signal<T> {
    type Unsubscriber = Unsubscriber<T>;

    #[inline]
    fn for_each<F>(self, notify: F) -> Self::Unsubscriber
    where
        F: FnMut(&T) + 'static,
    {
        self.for_each(notify)
    }

    #[inline]
    fn for_each_inner<F>(self, notify: F)
    where
        F: FnMut(&T, &mut Self::Unsubscriber) + 'static,
    {
        self.for_each_inner(notify);
    }

    #[inline]
    fn for_each_forever<F>(self, notify: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.for_each_forever(notify);
    }
}

impl<T> Value<T> for &Mutable<T> {
    type Unsubscriber = Unsubscriber<T>;

    #[inline]
    fn for_each<F>(self, notify: F) -> Self::Unsubscriber
    where
        F: FnMut(&T) + 'static,
    {
        self.0.for_each(notify)
    }

    #[inline]
    fn for_each_inner<F>(self, notify: F)
    where
        F: FnMut(&T, &mut Self::Unsubscriber) + 'static,
    {
        self.0.for_each_inner(notify);
    }

    #[inline]
    fn for_each_forever<F>(self, notify: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.0.for_each_forever(notify);
    }
}

// pub trait Signal
// where
//     for<'x> &'x Self: Value<Self::Item>,
// {
//     type Item;

//     fn try_get(&self) -> Result<Self::Item>
//     where
//         Self::Item: Clone;

//     #[inline]
//     fn get(&self) -> Self::Item
//     where
//         Self::Item: Clone,
//     {
//         self.try_get().unwrap()
//     }

//     // fn map<B, F>(&self, notify: F) -> Map<B>
//     // where
//     //     F: FnMut(&Self::Item) -> B + 'static;

//     // fn filter<P>(&self, predicate: P) -> Computed<Self::Item>
//     // where
//     //     P: FnMut(&Self::Item) -> bool;

//     // fn filter_map<B, F>(&self, notify: F) -> Computed<B>
//     // where
//     //     F: FnMut(&Self::Item) -> Option<B>;

//     // fn fold<B, F>(&self, init: B, notify: F) -> Computed<B>
//     // where
//     //     F: FnMut(&mut B, &Self::Item);

//     // fn map_while<B, P>(&self, predicate: P) -> Computed<B>
//     // where
//     //     P: FnMut(&Self::Item) -> Option<B>;

//     // fn skip(&self, n: usize) -> Computed<Self::Item>;

//     // fn skip_while<P>(&self, predicate: P) -> Computed<Self::Item>
//     // where
//     //     P: FnMut(&Self::Item) -> bool;
// }

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
