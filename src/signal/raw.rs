use core::cell::{Cell, UnsafeCell};
use core::marker::PhantomData;
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::Result;

/// A pointer to a value whose type was erased.
type Erased = NonNull<()>;

type NotifyFn = dyn FnMut(Erased);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListenerId(usize);

struct Listener {
    id: ListenerId,
    active: Cell<bool>,
    notify: NonNull<NotifyFn>,
}

impl Listener {
    #[inline]
    fn id(&self) -> ListenerId {
        self.id
    }

    #[inline]
    fn active(&self) -> bool {
        self.active.get()
    }
}

impl Drop for Listener {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.notify.as_mut());
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    Idling,
    Mutating,
    Subscribing,
    Uninit,
}

struct InnerRawSignal {
    value: Erased,
    state: Cell<State>,
    next_id: Cell<usize>,
    needs_retain: Cell<bool>,
    listeners: UnsafeCell<Vec<Listener>>,
}

impl InnerRawSignal {
    fn new_with_state(value: Erased, state: State) -> Self {
        Self {
            value,
            state: Cell::new(state),
            next_id: Cell::new(0),
            needs_retain: Cell::new(false),
            listeners: UnsafeCell::new(Vec::new()),
        }
    }

    #[inline]
    fn state(&self) -> State {
        self.state.get()
    }

    fn next_id(&self) -> ListenerId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        ListenerId(id)
    }

    /// # Safety
    /// 
    /// The state must neither be `Subscribing` or `Mutating`, so
    /// that none of the listeners are currently borrowed.
    unsafe fn retain(&self) {
        if self.needs_retain.replace(false) {
            let listeners = self.listeners.get();
            (*listeners).retain(Listener::active)
        }
    }

    fn push_listener(&self, id: ListenerId, mut notify: NonNull<NotifyFn>) {
        let listeners = self.listeners.get();

        unsafe {
            (*listeners).push(Listener {
                id,
                active: Cell::new(true),
                notify,
            });
        }

        match self.state() {
            State::Idling => unsafe {
                self.state.set(State::Subscribing);
                notify.as_mut()(self.value);
                self.state.set(State::Idling);
                self.retain();
            },
            State::Subscribing => unsafe {
                notify.as_mut()(self.value);
            },
            _ => (),
        }
    }

    fn unsubscribe(&self, id: ListenerId) -> Option<()> {
        let listeners = self.listeners.get();

        let index = unsafe { (*listeners).binary_search_by_key(&id, Listener::id).ok()? };

        match self.state() {
            State::Mutating | State::Subscribing => unsafe {
                let listener = &(*listeners)[index];
                listener.active.set(false);
                self.needs_retain.set(true);
            },
            _ => unsafe {
                (*listeners).remove(index);
            },
        }

        Some(())
    }

    // TODO: check states
    unsafe fn notify_all(&self) {
        let listeners = self.listeners.get();
        let mut i = 0;

        unsafe {
            while i < (*listeners).len() {
                let listener = (*listeners).as_mut_ptr().add(i).as_ref().unwrap();
                if listener.active() {
                    let mut notify = listener.notify;
                    (notify.as_mut())(self.value);
                }
                i += 1;
            }

            self.retain();
        }
    }
}

pub struct RawSignal<T> {
    _phantom: PhantomData<T>,
    inner: InnerRawSignal,
}

impl<T> RawSignal<T> {
    fn new_with_state(value: NonNull<T>, state: State) -> Self {
        Self {
            _phantom: PhantomData,
            inner: InnerRawSignal::new_with_state(value.cast(), state),
        }
    }

    #[inline]
    pub fn new(value: NonNull<T>) -> Self {
        Self::new_with_state(value, State::Idling)
    }

    #[inline]
    pub fn uninit(value: NonNull<T>) -> Self {
        Self::new_with_state(value, State::Uninit)
    }

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.inner.state() != State::Uninit
    }

    pub unsafe fn for_each<F, G>(&self, make_notify: G) -> ListenerId
    where
        F: FnMut(&T) + 'static,
        G: FnOnce(ListenerId) -> F,
    {
        let id = self.inner.next_id();

        let notify = {
            let mut f = make_notify(id);
            let boxed = Box::new(move |data: Erased| {
                let data = unsafe { data.cast().as_mut() };
                f(data);
            });
            NonNull::new(Box::into_raw(boxed)).unwrap()
        };

        self.inner.push_listener(id, notify);

        id
    }

    #[inline]
    pub fn unsubscribe(&self, id: ListenerId) {
        self.inner.unsubscribe(id);
    }

    #[inline]
    pub unsafe fn notify_all(&self) -> Result<()> {
        todo!()
    }

    #[inline]
    pub unsafe fn set(&self, new_value: T) -> Result<()> {
        todo!()
    }

    #[inline]
    pub unsafe fn mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        todo!()
    }
}
