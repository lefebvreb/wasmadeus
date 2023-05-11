mod broadcast;

use core::cell::{Ref, RefCell, OnceCell};
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::rc::Rc;

use super::{Result, SignalError};

use self::broadcast::Broadcast;

type Data<T> = Rc<OnceCell<RefCell<T>>>;

/// The ID of a subscription to a signal, can be used to unsubscribe from
/// this signal.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriberId(usize);

pub struct RawSignal<T> {
    broadcast: Broadcast<T>,
    data: Data<T>,
}

impl<T> RawSignal<T> {
    fn new_with_data(data: Data<T>) -> Self {
        Self {
            broadcast: Broadcast::default(),
            data,
        }
    }

    #[inline]
    pub fn new(initial_value: T) -> Self {
        let data = Rc::new(OnceCell::from(RefCell::new(initial_value)));
        Self::new_with_data(data)
    }

    #[inline]
    pub fn uninit() -> Self {
        let data = Rc::new(OnceCell::new());
        Self::new_with_data(data)
    }

    #[inline]
    fn try_borrow(&self) -> Result<Ref<T>> {
        self.data.get()
            .and_then(|refcell| refcell.try_borrow().ok())
            .ok_or(SignalError)
    }

    pub fn raw_for_each<F, G>(&self, make_notify: G) -> SubscriberId
    where
        F: FnMut(&T) + 'static,
        G: FnOnce(SubscriberId) -> F,
    {
        let id = self.broadcast.next_id();

        let notify = {
            let boxed = Box::new(make_notify(id));
            NonNull::new(Box::into_raw(boxed)).unwrap()
        };

        todo!()
    }

    pub fn try_set(&self, new_value: T) -> Result<()> {
        self.data.set(RefCell::new(new_value)).map_err(|_| SignalError)?;
        let value = self.try_borrow().unwrap();
        todo!()
    }

    pub fn try_mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        todo!()
    }

    #[inline]
    pub fn unsubscribe(&self, id: SubscriberId) {
        self.broadcast.unsubscribe(id);
    }

    #[inline]
    pub fn try_get(&self) -> Result<T>
    where
        T: Clone,
    {
        todo!()
    }
}
