use core::future::Future;

use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;

macro_rules! for_all_tuples {
    ($mac: ident) => {
        $mac! {}
        $mac! { A }
        $mac! { A B }
        $mac! { A B C }
        $mac! { A B C D }
        $mac! { A B C D E }
        $mac! { A B C D E F }
        $mac! { A B C D E F G }
        $mac! { A B C D E F G H }
        $mac! { A B C D E F G H I }
        $mac! { A B C D E F G H I J }
        $mac! { A B C D E F G H I J K }
        $mac! { A B C D E F G H I J K L }
        // $mac! { A B C D E F G H I J K L M }
        // $mac! { A B C D E F G H I J K L M N }
        // $mac! { A B C D E F G H I J K L M N O }
        // $mac! { A B C D E F G H I J K L M N O P }
        // $mac! { A B C D E F G H I J K L M N O P Q }
        // $mac! { A B C D E F G H I J K L M N O P Q R }
        // $mac! { A B C D E F G H I J K L M N O P Q R S }
        // $mac! { A B C D E F G H I J K L M N O P Q R S T }
    };
}

pub(crate) use for_all_tuples;

pub trait TryAsRef<T>
where
    T: ?Sized,
{
    fn try_as_ref(&self) -> Option<&T>;
}

impl TryAsRef<str> for str {
    #[inline]
    fn try_as_ref(&self) -> Option<&str> {
        Some(self)
    }
}

impl TryAsRef<str> for String {
    #[inline]
    fn try_as_ref(&self) -> Option<&str> {
        Some(self)
    }
}

impl TryAsRef<str> for Option<&str> {
    #[inline]
    fn try_as_ref(&self) -> Option<&str> {
        *self
    }
}

impl TryAsRef<str> for Option<String> {
    #[inline]
    fn try_as_ref(&self) -> Option<&str> {
        self.as_ref().map(String::as_ref)
    }
}

impl<T> TryAsRef<T> for Option<T> {
    fn try_as_ref(&self) -> Option<&T> {
        self.as_ref()
    }
}

impl<T, E> TryAsRef<T> for Result<T, E> {
    fn try_as_ref(&self) -> Option<&T> {
        self.as_ref().ok()
    }
}

impl<T, U> TryAsRef<U> for &T
where
    T: TryAsRef<U> + ?Sized,
    U: ?Sized,
{
    #[inline]
    fn try_as_ref(&self) -> Option<&U> {
        T::try_as_ref(self)
    }
}

impl<T, U> TryAsRef<U> for &mut T
where
    T: TryAsRef<U> + ?Sized,
    U: ?Sized,
{
    #[inline]
    fn try_as_ref(&self) -> Option<&U> {
        T::try_as_ref(self)
    }
}

impl<T> TryAsRef<T> for Box<T>
where
    T: ?Sized,
{
    #[inline]
    fn try_as_ref(&self) -> Option<&T> {
        Some(self)
    }
}

impl<T> TryAsRef<T> for Rc<T>
where
    T: ?Sized,
{
    #[inline]
    fn try_as_ref(&self) -> Option<&T> {
        Some(self)
    }
}

impl<T> TryAsRef<T> for Arc<T>
where
    T: ?Sized,
{
    #[inline]
    fn try_as_ref(&self) -> Option<&T> {
        Some(self)
    }
}

impl<T> TryAsRef<T> for Cow<'_, T>
where
    T: ToOwned + ?Sized,
{
    #[inline]
    fn try_as_ref(&self) -> Option<&T> {
        Some(self)
    }
}

#[inline]
pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}
