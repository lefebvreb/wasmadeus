use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

use alloc::rc::Rc;

use super::Result;
use super::raw::RawSignal;

#[derive(Debug)]
#[repr(transparent)]
pub struct Mutable<T: 'static>(Rc<RawSignal<UnsafeCell<MaybeUninit<T>>>>);

impl<T> Mutable<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self(Rc::new(RawSignal::new(value)))
    }

    #[inline]
    pub fn uninit() -> Self {
        Self(Rc::new(RawSignal::uninit()))
    }

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.0.is_initialized()
    }

    #[inline]
    pub fn try_mutate<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        let (raw, mut data) = self.0.get();
        if raw.state() != SignalState::Idling {
            return Err(SignalError);
        }
        // SAFETY: `data` will live longer than this closure. `RawSignal::try_mutate`
        // will make sure the it is not called twice at the same time.
        unsafe { raw.try_mutate(|| f(data.as_mut().assume_init_mut())) }
    }

    #[inline]
    pub fn try_update<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&T) -> T,
    {
        self.try_mutate(|data| *data = f(data))
    }

    #[inline]
    pub fn mutate<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        self.try_mutate(f).unwrap();
    }

    #[inline]
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&T) -> T,
    {
        self.try_update(f).unwrap();
    }

    #[inline]
    pub fn try_set(&self, value: T) -> Result<()> {
        let (raw, mut data) = self.0.get();
        match raw.state() {
            SignalState::Idling => self.try_mutate(|data| *data = value),
            SignalState::Uninit => unsafe {
                raw.try_mutate(|| {
                    data.as_mut().write(value);
                })
            },
            _ => Err(SignalError),
        }
    }

    #[inline]
    pub fn set(&self, value: T) {
        self.try_set(value).unwrap();
    }

    pub fn for_each<F>(&self, mut f: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        let (raw, data) = self.0.get();
        let id = raw.for_each(|_| {
            // SAFETY: when this closure gets called, there shall be no
            // other mutable borrow to data.
            Box::new(move || unsafe {
                f(data.as_ref().assume_init_ref());
            })
        });
        Unsubscriber::new(self, id)
    }

    pub fn for_each_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static,
    {
        let (raw, data) = self.0.get();
        raw.for_each(|id| {
            let mut unsub = Unsubscriber::new(self, id);
            // SAFETY: when this closure gets called, there shall be no
            // other mutable borrow to data.
            Box::new(move || unsafe {
                f(data.as_ref().assume_init_ref(), &mut unsub);
            })
        });
    }

    pub(super) fn compose<B, F, G>(&self, g: G) -> Computed<B>
    where
        F: FnMut(&T) + 'static,
        G: FnOnce(Mutable<B>) -> F,
    {
        let computed = Computed::uninit();
        let mutable = computed.as_mutable().clone();
        let _ = self.for_each(g(mutable));
        computed
    }
}

impl<T> Clone for Mutable<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Signal for Mutable<T> {
    type Item = T;

    fn try_get(&self) -> Result<Self::Item>
    where
        Self::Item: Clone,
    {
        let (raw, data) = self.0.get();
        if matches!(raw.state(), SignalState::Mutating | SignalState::Uninit) {
            return Err(SignalError);
        }
        // SAFETY: the data is not currently getting mutated, therefore it is safe
        // to borrow it immutably.
        Ok(unsafe { data.as_ref().assume_init_ref() }.clone())
    }

    fn map<B, F>(&self, mut f: F) -> Computed<B>
    where
        F: FnMut(&Self::Item) -> B + 'static,
    {
        self.compose(|mutable| move |data| mutable.set(f(data)))
    }

    fn filter<P>(&self, predicate: P) -> Computed<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.compose(|mutable| move |data| todo!())
    }
}

impl<T> Value<T> for Mutable<T> {
    #[inline]
    fn for_each<F>(&self, f: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static,
    {
        self.for_each(f)
    }

    #[inline]
    fn for_each_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static,
    {
        self.for_each_inner(f);
    }
}

#[derive(Debug)]
struct NotifierRef<T> {
    signal: Weak<InnerSignal<T>>,
    id: u32,
}

impl<T> Clone for NotifierRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            signal: self.signal.clone(),
            id: self.id,
        }
    }
}

#[must_use]
#[derive(Debug)]
#[repr(transparent)]
pub struct Unsubscriber<T>(Option<NotifierRef<T>>);

impl<T> Unsubscriber<T> {
    #[inline]
    fn new(mutable: &Mutable<T>, id: u32) -> Self {
        Self(Some(NotifierRef {
            signal: Rc::downgrade(&mutable.0),
            id,
        }))
    }

    #[inline]
    pub(super) fn empty() -> Self {
        Self(None)
    }

    #[inline]
    pub fn needed(&self) -> bool {
        self.0.is_some()
    }

    pub fn unsubscribe(&mut self) {
        if let Some(info) = self.0.take() {
            if let Some(inner) = info.signal.upgrade() {
                let (raw, _) = inner.get();
                raw.unsubscribe(info.id);
            }
        }
    }

    #[inline]
    pub fn droppable(self) -> DropUnsubscriber<T> {
        DropUnsubscriber(self)
    }
}

impl<T> Clone for Unsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[must_use]
#[derive(Debug)]
#[repr(transparent)]
pub struct DropUnsubscriber<T>(pub Unsubscriber<T>);

impl<T> DropUnsubscriber<T> {
    #[inline]
    pub fn take(mut self) -> Unsubscriber<T> {
        let inner = &mut self.0;
        Unsubscriber(inner.0.take())
    }
}

impl<T> Clone for DropUnsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for DropUnsubscriber<T> {
    type Target = Unsubscriber<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DropUnsubscriber<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for DropUnsubscriber<T> {
    #[inline]
    fn drop(&mut self) {
        self.0.unsubscribe();
    }
}
