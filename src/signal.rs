use core::cell::{Cell, UnsafeCell};
use core::num::NonZeroU32;
use core::ops::{Deref, DerefMut};

use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SignalMutatingError;

#[derive(Copy, Clone, Debug)]
enum NotifierState {
    Active(NonZeroU32),
    Deleted(NonZeroU32),
}

#[derive(Debug)]
struct Notifier {
    state: Cell<NotifierState>,
    notify: *mut dyn FnMut(),
}

impl Drop for Notifier {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: we have exclusive access to this pointer, that is
        // never copied.
        unsafe {
            let _ = Box::from_raw(self.notify);
        }
    }
}

impl Notifier {
    #[inline]
    fn id(&self) -> NonZeroU32 {
        match self.state.get() {
            NotifierState::Active(id) | NotifierState::Deleted(id) => id,
        }
    }

    #[inline]
    fn active(&self) -> bool {
        matches!(self.state.get(), NotifierState::Active(_))
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum SignalState {
    /// The signal is not currently in use.
    Idling,
    /// The signal's data is currently being updated and/or
    /// its subscribers are being notified.
    Mutating,
    /// The signal is currently notifying new susbcribers with
    /// a reference to the current data.
    Subscribing,
}

#[derive(Debug)]
struct RawSignal {
    state: Cell<SignalState>,
    next_id: Cell<NonZeroU32>,
    needs_delete: Cell<bool>,
    subscribers: UnsafeCell<Vec<Notifier>>,
}

impl RawSignal {
    #[inline]
    fn new() -> Self {
        Self {
            state: Cell::new(SignalState::Idling),
            subscribers: UnsafeCell::new(Vec::new()),
            next_id: Cell::new(1.try_into().unwrap()),
            needs_delete: Cell::new(false),
        }
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
                (*notifier.notify)();
            }

            i += 1;
        }

        if self.needs_delete.take() {
            (*subscribers).retain(Notifier::active);
        }
    }

    fn try_mutate<F>(&self, mutater: F) -> Result<(), SignalMutatingError>
    where
        F: FnOnce(),
    {
        if self.state.get() != SignalState::Idling {
            return Err(SignalMutatingError);
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

    fn append_notifier(&self, id: NonZeroU32, mut notifier: Box<dyn FnMut()>) {
        if self.state.get() != SignalState::Mutating {
            let old_state = self.state.replace(SignalState::Subscribing);
            notifier();
            self.state.set(old_state);
        }

        let subscribers = self.subscribers.get();

        // SAFETY: it is always safe to append to `self.subscribers` because
        // no one keeps a borrow of it between call boundaries.
        unsafe {
            (*subscribers).push(Notifier {
                state: Cell::new(NotifierState::Active(id)),
                notify: Box::into_raw(notifier),
            });
        }
    }

    fn for_each<F>(&self, make_notifier: F) -> NonZeroU32
    where
        F: FnOnce(NonZeroU32) -> Box<dyn FnMut()>,
    {
        let id = self.next_id.get();
        self.next_id.set(id.saturating_add(1));
        self.append_notifier(id, make_notifier(id));
        id
    }

    fn unsubscribe(&self, id: NonZeroU32) {
        let subscribers = self.subscribers.get();

        // SAFETY: if the signal is mutating, we simply borrow `self.subscribers` immutably.
        // If it is not, we remove a single element of it.
        // In both case, we don't retain the borrow through any other function calls.
        unsafe {
            if let Ok(index) = (*subscribers).binary_search_by_key(&id, Notifier::id) {
                if self.state.get() == SignalState::Mutating {
                    let notifier = (*subscribers).get(index).unwrap();
                    let id = notifier.id();
                    notifier.state.set(NotifierState::Deleted(id));
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
    fn get(&self) -> (&RawSignal, *mut T) {
        (&self.0, self.1.get())
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Signal<T: 'static>(Rc<InnerSignal<T>>);

impl<T> Signal<T> {
    pub fn new(value: T) -> Self {
        let raw = RawSignal::new();
        let data = UnsafeCell::new(value);
        Self(Rc::new(InnerSignal(raw, data)))
    }

    #[inline]
    pub fn try_mutate<F>(&self, f: F) -> Result<(), SignalMutatingError>
    where
        F: FnOnce(&mut T),
    {
        let (raw, data) = self.0.get();
        // SAFETY: `data` will live longer than this closure. `RawSignal::try_mutate`
        // will make sure the it is not called twice at the same time.
        raw.try_mutate(|| f(unsafe { &mut *data }))
    }

    #[inline]
    pub fn try_update<F>(&self, f: F) -> Result<(), SignalMutatingError>
    where
        F: FnOnce(&T) -> T,
    {
        self.try_mutate(|data| *data = f(data))
    }

    #[inline]
    pub fn try_set(&self, value: T) -> Result<(), SignalMutatingError> {
        self.try_mutate(|data| *data = value)
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
        let id = raw.for_each(|_| Box::new(move || f(unsafe { &*data })));

        let info = NotifierRef {
            signal: Rc::downgrade(&self.0),
            id,
        };

        Unsubscriber(Some(info))
    }

    pub fn for_each_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static,
    {
        let (raw, data) = self.0.get();

        raw.for_each(|id| {
            let info = NotifierRef {
                signal: Rc::downgrade(&self.0),
                id,
            };

            let mut unsub = Unsubscriber(Some(info));
            // SAFETY: when this closure gets called, there shall be no
            // other mutable borrow to data.
            Box::new(move || f(unsafe { &*data }, &mut unsub))
        });
    }
}

impl<T: Clone> Signal<T> {
    #[inline]
    pub fn get(&self) -> T {
        let (_, data) = self.0.get();
        unsafe { (*data).clone() }
    }
}

impl<T> Clone for Signal<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Computed<T: 'static>(Signal<T>);

#[derive(Debug)]
struct NotifierRef<T> {
    signal: Weak<InnerSignal<T>>,
    id: NonZeroU32,
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
    pub fn droppable(self) -> DroppableUnsubscriber<T> {
        DroppableUnsubscriber(self)
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
pub struct DroppableUnsubscriber<T>(pub Unsubscriber<T>);

impl<T> DroppableUnsubscriber<T> {
    #[inline]
    pub fn take(mut self) -> Unsubscriber<T> {
        Unsubscriber(self.0 .0.take())
    }
}

impl<T> Clone for DroppableUnsubscriber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for DroppableUnsubscriber<T> {
    type Target = Unsubscriber<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DroppableUnsubscriber<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for DroppableUnsubscriber<T> {
    #[inline]
    fn drop(&mut self) {
        self.0.unsubscribe();
    }
}

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
        Unsubscriber(None)
    }

    fn for_each_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static,
    {
        let mut unsub = Unsubscriber(None);
        f(self, &mut unsub);
    }
}

impl<T: 'static> Value<T> for Signal<T> {
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
