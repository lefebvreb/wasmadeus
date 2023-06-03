use core::str::FromStr;

use alloc::borrow::Cow;
use web_sys::Element;

use crate::utils::for_all_tuples;

pub trait Attribute: Sized {
    const NAME: &'static str;

    type Value: FromStr;

    fn new(value: Self::Value) -> Self;

    fn value(&self) -> Cow<str>;

    fn apply_to(self, element: &Element) {
        // We can safely unwrap provided the NAME is valid,
        // i.e. it does not contain any character not valid in attributes name.
        element.set_attribute(Self::NAME, &self.value()).unwrap();
    }

    fn extract(element: &Element) -> Option<Self> {
        element.get_attribute(Self::NAME)
            .map(|attr| attr.parse().ok().expect("failed to parse attribute value"))
            .map(Self::new)
    }
}

pub trait ElementAttributes: Sized {
    fn apply_all(self, element: &Element);
}

macro_rules! impl_element_attributes {
    ($($name: ident)*) => {
        impl<$($name: Attribute,)*> ElementAttributes for ($($name,)*) {
            #[allow(non_snake_case, unused_variables)]
            fn apply_all(self, element: &Element) {
                let ($($name,)*) = self;
                $($name.apply_to(element);)*
            }
        }
    };
}

for_all_tuples!(impl_element_attributes);

// #[derive(Clone)]
// pub struct Class<T: Value>(pub T)
// where
//     T::Item: AsRef<str>;

// impl<T: Value> Attribute for Class<T>
// where
//     T::Item: AsRef<str>,
// {
//     #[inline]
//     fn apply_to(self, element: &Element) {
//         let element = element.clone();
//         self.0.for_each_forever(move |value| {
//             element.set_class_name(value.as_ref());
//         });
//     }
// }
