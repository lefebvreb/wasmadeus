use super::{Signal, SignalMut, Unsubscriber};

pub trait Value: Sized {
    type Item;

    type Unsubscriber;

    fn for_each<F>(&self, notify: F) -> Self::Unsubscriber
    where
        F: FnMut(&Self::Item) + 'static;

    fn for_each_inner<F>(&self, notify: F)
    where
        F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static;

    #[inline]
    fn for_each_forever<F>(&self, notify: F)
    where
        F: FnMut(&Self::Item) + 'static,
    {
        _ = self.for_each(notify);
    }
}

#[cfg(not(feature = "nightly"))]
mod impls {
    use super::{Signal, SignalMut, Unsubscriber, Value};

    impl<T> Value for T
    where
        T: Copy,
    {
        type Item = T;

        type Unsubscriber = ();

        #[inline]
        fn for_each<F>(&self, notify: F) -> Self::Unsubscriber
        where
            F: FnOnce(&Self::Item),
        {
            notify(self);
        }

        #[inline]
        fn for_each_inner<F>(&self, notify: F)
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
        fn for_each<F>(&self, notify: F) -> Self::Unsubscriber
        where
            F: FnMut(&Self::Item) + 'static,
        {
            self.for_each(notify)
        }

        #[inline]
        fn for_each_inner<F>(&self, notify: F)
        where
            F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static,
        {
            self.for_each_inner(notify);
        }

        #[inline]
        fn for_each_forever<F>(&self, notify: F)
        where
            F: FnMut(&Self::Item) + 'static,
        {
            self.for_each_forever(notify);
        }
    }

    impl<T> Value for SignalMut<T> {
        type Item = T;

        type Unsubscriber = Unsubscriber<T>;

        #[inline]
        fn for_each<F>(&self, notify: F) -> Self::Unsubscriber
        where
            F: FnMut(&Self::Item) + 'static,
        {
            self.for_each(notify)
        }

        #[inline]
        fn for_each_inner<F>(&self, notify: F)
        where
            F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static,
        {
            self.for_each_inner(notify);
        }

        #[inline]
        fn for_each_forever<F>(&self, notify: F)
        where
            F: FnMut(&Self::Item) + 'static,
        {
            self.for_each_forever(notify);
        }
    }
}

#[cfg(feature = "nightly")]
mod impls {
    use super::{Signal, SignalMut, Unsubscriber, Value};

    auto trait NonSignal {}

    impl<T> !NonSignal for Signal<T> {}
    impl<T> !NonSignal for SignalMut<T> {}

    impl<T> Value for T
    where
        T: Copy + NonSignal,
    {
        type Item = T;

        type Unsubscriber = ();

        #[inline]
        fn for_each<F>(&self, notify: F) -> Self::Unsubscriber
        where
            F: FnOnce(&Self::Item),
        {
            notify(&self);
        }

        #[inline]
        fn for_each_inner<F>(&self, notify: F)
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
        fn for_each<F>(&self, notify: F) -> Self::Unsubscriber
        where
            F: FnMut(&Self::Item) + 'static,
        {
            Signal::for_each(self, notify)
        }

        #[inline]
        fn for_each_inner<F>(&self, notify: F)
        where
            F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static,
        {
            Signal::for_each_inner(self, notify);
        }

        #[inline]
        fn for_each_forever<F>(&self, notify: F)
        where
            F: FnMut(&Self::Item) + 'static,
        {
            Signal::for_each_forever(self, notify);
        }
    }

    impl<T> Value for &SignalMut<T> {
        type Item = T;

        type Unsubscriber = Unsubscriber<T>;

        #[inline]
        fn for_each<F>(&self, notify: F) -> Self::Unsubscriber
        where
            F: FnMut(&Self::Item) + 'static,
        {
            SignalMut::for_each(self, notify)
        }

        #[inline]
        fn for_each_inner<F>(&self, notify: F)
        where
            F: FnMut(&Self::Item, &mut Self::Unsubscriber) + 'static,
        {
            SignalMut::for_each_inner(self, notify);
        }

        #[inline]
        fn for_each_forever<F>(&self, notify: F)
        where
            F: FnMut(&Self::Item) + 'static,
        {
            SignalMut::for_each_forever(self, notify);
        }
    }
}
