mod error;
mod raw;
mod unsub;
mod value;

use core::ops::Deref;

use alloc::rc::Rc;

use self::raw::RawSignal;

pub use error::*;
pub use unsub::*;
pub use value::*;

#[repr(transparent)]
pub struct Signal<T: 'static>(Rc<RawSignal<T>>);

impl<T> Signal<T> {
    #[inline]
    fn new_from_raw(raw: RawSignal<T>) -> Self {
        Self(Rc::new(raw))
    }

    #[inline]
    fn raw(&self) -> &Rc<RawSignal<T>> {
        &self.0
    }

    #[inline]
    pub fn try_get(&self) -> Result<T, SignalGetError>
    where
        T: Clone,
    {
        self.raw().try_get()
    }

    #[inline]
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.try_get().unwrap()
    }

    #[inline]
    fn compose<U, F>(&self, raw: RawSignal<U>, mut notify: F) -> Signal<U>
    where
        F: FnMut(&RawSignal<U>, &T, &mut SignalUnsubscriber<T>) + 'static,
    {
        let signal = Signal::new_from_raw(raw);
        let weak = Rc::downgrade(signal.raw());

        self.for_each_inner(move |value, unsub| match weak.upgrade() {
            Some(raw) => notify(&raw, value, unsub),
            _ => unsub.unsubscribe(),
        });

        signal
    }

    #[inline]
    pub fn map<U, F>(&self, mut map: F) -> Signal<U>
    where
        F: FnMut(&T) -> U + 'static,
    {
        self.compose(RawSignal::new(None), move |raw, value, _| {
            raw.try_set(map(value)).unwrap();
        })
    }

    #[inline]
    pub fn filter<P>(&self, mut predicate: P) -> Signal<T>
    where
        P: FnMut(&T) -> bool + 'static,
    {
        self.compose(self.raw().shared(), move |raw, value, _| {
            if predicate(value) {
                raw.notify_all();
            }
        })
    }

    #[inline]
    pub fn filter_map<U, F>(&self, mut filter_map: F) -> Signal<U>
    where
        F: FnMut(&T) -> Option<U> + 'static,
    {
        self.compose(RawSignal::new(None), move |raw, value, _| {
            if let Some(value) = filter_map(value) {
                raw.try_set(value).unwrap();
            }
        })
    }

    #[inline]
    pub fn fold<U, F>(&self, initial_value: U, mut fold: F) -> Signal<U>
    where
        F: FnMut(&mut U, &T) + 'static,
    {
        self.compose(RawSignal::new(Some(initial_value)), move |raw, value, _| {
            raw.try_mutate(|acc| fold(acc, value)).unwrap();
        })
    }

    #[inline]
    pub fn map_while<U, F>(&self, mut map_while: F) -> Signal<U>
    where
        F: FnMut(&T) -> Option<U> + 'static,
    {
        self.compose(RawSignal::new(None), move |raw, value, unsub| match map_while(value) {
            Some(value) => raw.try_set(value).unwrap(),
            _ => unsub.unsubscribe(),
        })
    }

    #[inline]
    pub fn skip(&self, n: usize) -> Signal<T> {
        let mut counter = 0;
        self.compose(self.raw().shared(), move |raw, _, _| {
            if counter < n {
                counter += 1;
            } else {
                raw.notify_all();
            }
        })
    }

    #[inline]
    pub fn skip_while<P>(&self, predicate: P) -> Signal<T>
    where
        P: FnMut(&T) -> bool + 'static,
    {
        let mut option = Some(predicate);
        self.compose(self.raw().shared(), move |raw, value, _| {
            if let Some(predicate) = &mut option {
                if predicate(value) {
                    return;
                } else {
                    option.take();
                }
            }
            raw.notify_all();
        })
    }

    #[inline]
    pub fn take(&self, n: usize) -> Signal<T> {
        let mut counter = 0;
        self.compose(self.raw().shared(), move |raw, _, _| {
            if counter < n {
                counter += 1;
                raw.notify_all();
            }
        })
    }

    #[inline]
    pub fn take_while<P>(&self, predicate: P) -> Signal<T>
    where
        P: FnMut(&T) -> bool + 'static,
    {
        let mut option = Some(predicate);
        self.compose(self.raw().shared(), move |raw, value, _| {
            if let Some(predicate) = &mut option {
                if predicate(value) {
                    raw.notify_all();
                } else {
                    option.take();
                }
            }
        })
    }

    #[inline]
    pub fn for_each<F>(&self, notify: F) -> SignalUnsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        let id = self.raw().raw_for_each(|_| notify);
        SignalUnsubscriber::new(Rc::downgrade(self.raw()), id)
    }

    #[inline]
    pub fn for_each_inner<F>(&self, mut notify: F)
    where
        F: FnMut(&T, &mut SignalUnsubscriber<T>) + 'static,
    {
        let weak = Rc::downgrade(self.raw());
        self.raw().raw_for_each(|id| {
            let mut unsub = SignalUnsubscriber::new(weak, id);
            move |data| notify(data, &mut unsub)
        });
    }

    #[inline]
    pub fn for_each_forever<F>(&self, notify: F)
    where
        F: FnMut(&T) + 'static,
    {
        self.raw().raw_for_each(|_| notify);
    }
}

impl<T> Clone for Signal<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[repr(transparent)]
pub struct SignalMut<T: 'static>(Signal<T>);

impl<T> SignalMut<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self(Signal::new_from_raw(RawSignal::new(Some(initial_value))))
    }

    #[inline]
    pub fn uninit() -> Self {
        Self(Signal::new_from_raw(RawSignal::new(None)))
    }

    #[inline]
    pub fn maybe_uninit(initial_value: Option<T>) -> Self {
        Self(Signal::new_from_raw(RawSignal::new(initial_value)))
    }

    #[inline]
    pub fn try_set(&self, new_value: T) -> Result<(), SignalUpdatingError> {
        self.raw().try_set(new_value)
    }

    #[inline]
    pub fn set(&self, new_value: T) {
        self.try_set(new_value).unwrap();
    }

    #[inline]
    pub fn try_mutate<F>(&self, mutate: F) -> Result<(), SignalUpdatingError>
    where
        F: FnOnce(&mut T),
    {
        self.raw().try_mutate(mutate)
    }

    #[inline]
    pub fn mutate<F>(&self, mutate: F)
    where
        F: FnOnce(&mut T),
    {
        self.try_mutate(mutate).unwrap();
    }

    #[inline]
    pub fn for_each<F>(&self, notify: F) -> SignalUnsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        Signal::for_each(self, notify)
    }

    #[inline]
    pub fn for_each_inner<F>(&self, notify: F)
    where
        F: FnMut(&T, &mut SignalUnsubscriber<T>) + 'static,
    {
        Signal::for_each_inner(self, notify);
    }

    #[inline]
    pub fn for_each_forever<F>(&self, notify: F)
    where
        F: FnMut(&T) + 'static,
    {
        Signal::for_each_forever(self, notify);
    }
}

impl<T> Clone for SignalMut<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for SignalMut<T> {
    type Target = Signal<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for SignalMut<T> {
    #[inline]
    fn from(initial_value: T) -> Self {
        Self::new(initial_value)
    }
}
