use alloc::rc::Rc;

use super::raw::RawFiltered;

#[repr(transparent)]
pub struct Filtered<T: 'static>(Rc<RawFiltered<T>>);

impl<T> Clone for Filtered<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// impl<T> Value<T> for &Filtered<T> {
//     type Unsubscriber;

//     fn for_each<F>(self, f: F) -> Self::Unsubscriber
//     where
//         F: FnMut(&T) + 'static {
//         todo!()
//     }

//     fn for_each_inner<F>(self, f: F)
//     where
//         F: FnMut(&T, &mut Self::Unsubscriber) + 'static {
//         todo!()
//     }
// }
