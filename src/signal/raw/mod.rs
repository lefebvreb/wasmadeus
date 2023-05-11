mod broadcast;
//mod data;

use core::cell::RefCell;
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::rc::Rc;

use super::{Result, SignalError};

use self::broadcast::Broadcast;

type Data<T> = Rc<Option<RefCell<T>>>;

/// The ID of a subscription to a signal, can be used to unsubscribe from
/// this signal.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriberId(usize);

pub struct RawSignal<T> {
    broadcast: Broadcast<T>,
    data: Rc<Option<RefCell<T>>>,
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
        let data = Rc::new(Some(RefCell::new(initial_value)));
        Self::new_with_data(data)
    }

    #[inline]
    pub fn uninit() -> Self {
        let data = Rc::new(None);
        Self::new_with_data(data)
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
        if let Some(refcell) = self.data.as_ref() {
            if let Ok(value) = refcell.try_borrow() {
                return Ok(value.clone());
            }
        }

        Err(SignalError)
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

        // unsafe {
        //     self.broadcast
        //         .push_subscriber(id, notify, Some(self.value()));
        // }

        id
    }

    pub fn try_set(&self, new_value: T) -> Result<()> {
        todo!()
    }

    pub fn try_mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        todo!()
    }
}
