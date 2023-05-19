use web_sys::HtmlElement;

use crate::signal::Value;
use crate::utils::for_all_tuples;

pub trait Attribute {
    fn apply_to(self, element: &HtmlElement);
}

impl Attribute for () {
    #[inline]
    fn apply_to(self, _: &HtmlElement) {}
}

macro_rules! impl_tuple_attribute {
    ($($name: ident)*) => {
        impl<$($name: Attribute,)*> Attribute for ($($name,)*) {
            #[allow(non_snake_case)]
            fn apply_to(self, element: &HtmlElement) {
                let ($($name,)*) = self;
                $($name.apply_to(element);)*
            }
        }
    };
}

for_all_tuples!(impl_tuple_attribute);

#[derive(Clone)]
pub struct Class<T: Value>(pub T)
where
    T::Item: AsRef<str>;

impl<T: Value> Attribute for Class<T>
where
    T::Item: AsRef<str>,
{
    #[inline]
    fn apply_to(self, element: &HtmlElement) {
        let element = element.clone();
        self.0.for_each_forever(move |value| {
            element.set_class_name(value.as_ref());
        });
    }
}
