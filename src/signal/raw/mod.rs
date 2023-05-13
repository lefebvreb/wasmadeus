mod broadcast;

use core::cell::RefCell;

use alloc::boxed::Box;
use alloc::rc::Rc;

use super::{Result, SignalError};

use self::broadcast::Broadcast;

type Data<T> = Rc<RefCell<Option<T>>>;

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
    fn new_from_value(value: Option<T>) -> Self {
        Self {
            broadcast: Broadcast::default(),
            data: Rc::new(RefCell::new(value)),
        }
    }

    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self::new_from_value(Some(initial_value))
    }

    #[inline]
    pub fn uninit() -> Self {
        Self::new_from_value(None)
    }

    pub fn raw_for_each<F, G>(&self, make_notify: G) -> SubscriberId
    where
        F: FnMut(&T) + 'static,
        G: FnOnce(SubscriberId) -> F,
    {
        let id = self.broadcast.next_id();
        let notify = Box::new(make_notify(id));
        let data = self.data.try_borrow().ok();
        let value = data.as_ref().and_then(|value| value.as_ref());
        self.broadcast.push_subscriber(id, notify, value);
        id
    }

    #[inline]
    fn notify_all(&self) {
        let data = self.data.borrow();
        self.broadcast.notify(data.as_ref().unwrap());
    }

    pub fn try_set(&self, new_value: T) -> Result<()> {
        let mut data = self.data.try_borrow_mut()?;
        *data = Some(new_value);
        drop(data);
        self.notify_all();
        Ok(())
    }

    pub fn try_mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        let mut data = self.data.try_borrow_mut()?;
        mutate(data.as_mut().ok_or(SignalError)?);
        drop(data);
        self.notify_all();
        Ok(())
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
        let data = self.data.try_borrow().map_err(|_| SignalError)?;
        data.as_ref().map(T::clone).ok_or(SignalError)
    }
}
