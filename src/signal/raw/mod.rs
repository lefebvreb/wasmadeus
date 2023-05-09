mod broadcast;
mod data;

use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::rc::Rc;

use super::Result;

use self::broadcast::Broadcast;
use self::data::SignalData;

/// A pointer to a ([`Sized`]) value whose type was erased.
type Erased = NonNull<()>;

/// The ID of a subscription to a signal, can be used to unsubscribe from
/// this signal.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriberId(usize);

pub struct RawSignal<T> {
    broadcast: Broadcast,
    data: Rc<SignalData<T>>,
}

impl<T> RawSignal<T> {
    fn new_with_data(data: Rc<SignalData<T>>) -> Self {
        Self {
            broadcast: Broadcast::default(),
            data,
        }
    }

    #[inline]
    pub fn new(initial_value: T) -> Self {
        let data = Rc::new(SignalData::new(initial_value));
        Self::new_with_data(data)
    }

    #[inline]
    pub fn uninit() -> Self {
        let data = Rc::new(SignalData::uninit());
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
        self.data.try_get()
    }

    pub fn raw_for_each<F, G>(&self, make_notify: G) -> SubscriberId
    where
        F: FnMut(&T) + 'static,
        G: FnOnce(SubscriberId) -> F,
    {
        let id = self.broadcast.next_id();

        let notify = {
            let mut f = make_notify(id);
            let boxed = Box::new(move |value: Erased| {
                f(unsafe { value.cast().as_mut() });
            });
            NonNull::new(Box::into_raw(boxed)).unwrap()
        };

        self.data.borrow_then(|data| {
            
        }).ok();

        // unsafe {
        //     self.broadcast
        //         .push_subscriber(id, notify, Some(self.value()));
        // }

        id
    }

    pub fn try_set(&self, new_value: T) -> Result<()> {
        self.data.try_set(new_value);
        todo!()
    }

    pub fn try_mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        todo!()
    }
}
