use core::cell::{Cell, UnsafeCell};
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;

use super::{Computed, Result, Signal, SignalError, Value};

#[derive(Debug)]
struct Notifier {
    id: u32,
    active: Cell<bool>,
    notify: NonNull<dyn FnMut()>,
}

impl Drop for Notifier {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: we have exclusive access to this pointer, that is
        // never copied.
        unsafe {
            let _ = Box::from_raw(self.notify.as_ptr());
        }
    }
}

impl Notifier {
    #[inline]
    fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    fn active(&self) -> bool {
        self.active.get()
    }
}

/// The state of a `RawSignal`, used to prevent reentrant calls from 
/// breaking the aliasing laws of Rust.
/// 
/// It is set by functions that need exclusive access to part of the `RawSignal`,
/// and are unset at the end of their executions.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
enum SignalState {
    /// The signal is not currently in use.
    #[default]
    Idling,
    /// The signal's data is currently being updated. 
    Mutating,
    /// The signal is currently notifying new susbcribers.
    Subscribing,
    /// The signal's data is currently uninitialized.
    Uninitialized,
}

#[derive(Default, Debug)]
struct RawSignal {
    state: Cell<SignalState>,
    next_id: Cell<u32>,
    needs_delete: Cell<bool>,
    subscribers: UnsafeCell<Vec<Notifier>>,
}

impl RawSignal {
    #[inline]
    fn state(&self) -> SignalState {
        self.state.get()
    }

    /// # Safety
    ///
    /// The caller must ensure that:
    /// * No element of the `self.subscribers` vector gets dropped.
    /// * No closure contained in the `self.subscribers` vector gets borrowed.
    /// * The parent signal's data is not mutated.
    unsafe fn notify_all(&self) {
        let subscribers = self.subscribers.get();
        let mut i = 0;

        while i < (*subscribers).len() {
            let notifier = (*subscribers).as_mut_ptr().add(i).as_ref().unwrap();
            if notifier.active() {
                (*notifier.notify.as_ptr())();
            }
            i += 1;
        }

        if self.needs_delete.take() {
            (*subscribers).retain(Notifier::active);
        }
    }

    fn try_mutate<F>(&self, mutater: F) -> Result<()>
    where
        F: FnOnce(),
    {
        if matches!(self.state(), SignalState::Mutating | SignalState::Subscribing) {
            return Err(SignalError);
        }

        self.state.set(SignalState::Mutating);
        mutater();
        // SAFETY: we set the state to Mutating, therefore preventing
        // others from removing elements from `self.subscribers` or
        // borrowing it's closures.
        unsafe { self.notify_all() };

        self.state.set(SignalState::Idling);
        Ok(())
    }

    fn append_notifier(&self, id: u32, mut notifier: Box<dyn FnMut()>) {
        if matches!(self.state(), SignalState::Idling | SignalState::Subscribing) {
            let old_state = self.state.replace(SignalState::Subscribing);
            notifier();
            self.state.set(old_state);
        }

        let subscribers = self.subscribers.get();

        // SAFETY: it is always safe to append to `self.subscribers` because
        // no one keeps a borrow of it between call boundaries.
        unsafe {
            (*subscribers).push(Notifier {
                id,
                active: Cell::new(true),
                notify: NonNull::new(Box::into_raw(notifier)).unwrap(),
            });
        }
    }

    fn for_each<F>(&self, make_notifier: F) -> u32
    where
        F: FnOnce(u32) -> Box<dyn FnMut()>,
    {
        let id = self.next_id.get();
        self.next_id.set(id.saturating_add(1));
        self.append_notifier(id, make_notifier(id));
        id
    }

    fn unsubscribe(&self, id: u32) {
        let subscribers = self.subscribers.get();

        // SAFETY: if the signal is mutating, we simply borrow `self.subscribers` immutably.
        // If it is not, we remove a single element of it.
        // In both case, we don't retain the borrow through any other function calls.
        unsafe {
            if let Ok(index) = (*subscribers).binary_search_by_key(&id, Notifier::id) {
                if self.state() == SignalState::Mutating {
                    let notifier = (*subscribers).get(index).unwrap();
                    notifier.active.set(false);
                    self.needs_delete.set(true);
                } else {
                    (*subscribers).remove(index);
                }
            }
        }
    }
}

#[derive(Debug)]
struct InnerSignal<T>(RawSignal, UnsafeCell<T>);

impl<T> InnerSignal<T> {
    #[inline]
    fn get(&self) -> (&RawSignal, NonNull<T>) {
        (&self.0, NonNull::new(self.1.get()).unwrap())
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Mutable<T: 'static>(Rc<InnerSignal<T>>);

impl<T> Mutable<T> {
    // TODO: prevent mutate()/update(), make set() correct.
    pub fn new_uninit() -> Self {
        todo!()
    }

    pub fn new(value: T) -> Self {
        let raw = RawSignal::default();
        let data = UnsafeCell::new(value);
        Self(Rc::new(InnerSignal(raw, data)))
    }

    #[inline]
    pub fn initialized(&self) -> bool {
        let (raw, _) = self.0.get();
        raw.state() != SignalState::Uninitialized
    }

    #[inline]
    pub fn try_mutate<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        if !self.initialized() {
            return Err(SignalError);
        }

        let (raw, mut data) = self.0.get();
        // SAFETY: `data` will live longer than this closure. `RawSignal::try_mutate`
        // will make sure the it is not called twice at the same time.
        raw.try_mutate(|| f(unsafe { data.as_mut() }))
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
        if self.initialized() {
            return self.try_mutate(|data| *data = value)
        }

        let (raw, mut data) = self.0.get();
        raw.try_mutate(|| unsafe {
            todo!()
        })
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
        // SAFETY: when the innermost closure gets called, there shall be no
        // other mutable borrow to data.
        let id = raw.for_each(|_| Box::new(move || f(unsafe { data.as_ref() })));
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
            Box::new(move || f(unsafe { data.as_ref() }, &mut unsub))
        });
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
        if matches!(raw.state(), SignalState::Mutating | SignalState::Uninitialized) {
            return Err(SignalError);
        }
        // SAFETY: the data is not currently getting mutated, therefore it is safe
        // to borrow it immutably.
        Ok(unsafe { data.as_ref() }.clone())
    }

    fn map<B, F>(&self, _f: F) -> Computed<B>
    where
        F: FnMut(&Self::Item) -> B,
    {
        let (raw, data) = self.0.get();
        todo!()
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
