use super::{Mutable, Signal, Unsubscriber};

pub trait Value: Sized {
    type Item;

    type Unsubscriber;

    fn for_each<F>(self, notify: F) -> Self::Unsubscriber
    where
        F: FnMut(&Self::Item) + 'static;

    fn for_each_inner<F>(self, notify: F)
    where
        F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static;

    #[inline]
    fn for_each_forever<F>(self, notify: F)
    where
        F: FnMut(&Self::Item) + 'static,
    {
        let _ = self.for_each(notify);
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

#[cfg(not(feature = "nightly"))]
mod impls {
    use super::{Mutable, Signal, Unsubscriber, Value};

    impl<T> Value for &T {
        type Item = T;

        type Unsubscriber = ();

        #[inline]
        fn for_each<F>(self, notify: F) -> Self::Unsubscriber
        where
            F: FnOnce(&Self::Item),
        {
            notify(self);
        }

        #[inline]
        fn for_each_inner<F>(self, notify: F)
        where
            F: FnOnce(&Self::Item, &mut Self::Unsubscriber),
        {
            notify(self, &mut ());
        }
    }

    impl<T> Value for Signal<T> {
        type Item = T;

        type Unsubscriber = Unsubscriber<T>;

        #[inline]
        fn for_each<F>(self, notify: F) -> Self::Unsubscriber
        where
            F: FnMut(&Self::Item) + 'static,
        {
            Self::for_each(&self, notify)
        }

        #[inline]
        fn for_each_inner<F>(self, notify: F)
        where
            F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static,
        {
            Self::for_each_inner(&self, notify);
        }

        #[inline]
        fn for_each_forever<F>(self, notify: F)
        where
            F: FnMut(&Self::Item) + 'static,
        {
            Self::for_each_forever(&self, notify);
        }
    }

    impl<T> Value for Mutable<T> {
        type Item = T;

        type Unsubscriber = Unsubscriber<T>;

        #[inline]
        fn for_each<F>(self, notify: F) -> Self::Unsubscriber
        where
            F: FnMut(&Self::Item) + 'static,
        {
            Self::for_each(&self, notify)
        }

        #[inline]
        fn for_each_inner<F>(self, notify: F)
        where
            F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static,
        {
            Self::for_each_inner(&self, notify);
        }

        #[inline]
        fn for_each_forever<F>(self, notify: F)
        where
            F: FnMut(&Self::Item) + 'static,
        {
            Self::for_each_forever(&self, notify);
        }
    }
}

#[cfg(feature = "nightly")]
mod impls {
    use core::ops::Deref;

    use super::{Mutable, Signal, Unsubscriber, Value};

    auto trait SignalValue {}

    impl<T> !SignalValue for &Signal<T> {}
    impl<T> !SignalValue for &Mutable<T> {}

    impl<T: Deref> Value for T
    where
        T: SignalValue,
        T::Target: Sized,
    {
        type Item = T::Target;

        type Unsubscriber = ();

        #[inline]
        fn for_each<F>(self, notify: F) -> Self::Unsubscriber
        where
            F: FnOnce(&Self::Item),
        {
            notify(&self);
        }

        #[inline]
        fn for_each_inner<F>(self, notify: F)
        where
            F: FnOnce(&Self::Item, &mut Self::Unsubscriber),
        {
            notify(&self, &mut ());
        }
    }

    impl<T> Value for &Signal<T> {
        type Item = T;

        type Unsubscriber = Unsubscriber<T>;

        #[inline]
        fn for_each<F>(self, notify: F) -> Self::Unsubscriber
        where
            F: FnMut(&Self::Item) + 'static,
        {
            self.for_each(notify)
        }

        #[inline]
        fn for_each_inner<F>(self, notify: F)
        where
            F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static,
        {
            self.for_each_inner(notify);
        }

        #[inline]
        fn for_each_forever<F>(self, notify: F)
        where
            F: FnMut(&Self::Item) + 'static,
        {
            self.for_each_forever(notify);
        }
    }

    impl<T> Value for &Mutable<T> {
        type Item = T;

        type Unsubscriber = Unsubscriber<T>;

        #[inline]
        fn for_each<F>(self, notify: F) -> Self::Unsubscriber
        where
            F: FnMut(&Self::Item) + 'static,
        {
            self.for_each(notify)
        }

        #[inline]
        fn for_each_inner<F>(self, notify: F)
        where
            F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static,
        {
            self.for_each_inner(notify);
        }

        #[inline]
        fn for_each_forever<F>(self, notify: F)
        where
            F: FnMut(&Self::Item) + 'static,
        {
            self.for_each_forever(notify);
        }
    }
}
