use super::{Signal, SignalMut, SignalUnsubscriber, Unsubscribe};

pub trait Value {
    type Item;

    type Unsubscriber: Unsubscribe;

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

impl<T> Value for Signal<T> {
    type Item = T;

    type Unsubscriber = SignalUnsubscriber<T>;

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

    type Unsubscriber = SignalUnsubscriber<T>;

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

impl<T> Value for &T {
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
