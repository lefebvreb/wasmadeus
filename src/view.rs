use core::future::Future;

use crate::component::Component;
use crate::signal::Value;
use crate::utils::{for_all_tuples, TryAsRef};

pub trait View: Sized {
    fn apply_to(self, parent: &Component);
}

impl View for Component {
    fn apply_to(self, parent: &Component) {
        todo!()
    }
}

// impl<T: View, F: FnOnce() -> T> View for F {
//     #[inline]
//     fn apply_to(self, parent: &Component) {
//         self().apply_to(parent);
//     }
// }

macro_rules! impl_view {
    ($($name: ident)*) => {
        impl<$($name: View,)*> View for ($($name,)*) {
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn apply_to(self, parent: &Component) {
                let ($($name,)*) = self;
                $($name.apply_to(parent);)*
            }
        }
    };
}

for_all_tuples!(impl_view);

#[derive(Debug)]
pub struct Text<V: Value>(pub V)
where
    V::Item: TryAsRef<str>;

impl<V: Value> View for Text<V>
where
    V::Item: TryAsRef<str>,
{
    fn apply_to(self, parent: &Component) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Await<F: Future>(pub F)
where
    F::Output: View;

#[derive(Debug)]
pub struct If<C, V: View>(pub C, pub V)
where
    C: Value<Item = bool>;

#[derive(Debug)]
pub struct IfElse<C, V1: View, V2: View>(pub C, pub V1, pub V2)
where
    C: Value<Item = bool>;
