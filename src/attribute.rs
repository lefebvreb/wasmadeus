use alloc::format;
use web_sys::Element;

use crate::signal::Value;
use crate::utils::for_all_tuples;

pub trait Attribute: Sized {
    fn apply_to(&self, element: &Element);
}

pub trait Attributes: Sized {
    fn apply_to(&self, element: &Element);
}

impl<T: Attribute> Attributes for T {
    #[inline]
    fn apply_to(&self, element: &Element) {
        self.apply_to(element);
    }
}

macro_rules! impl_element_attributes {
    ($($name: ident)*) => {
        impl<$($name: Attribute,)*> Attributes for ($($name,)*) {
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn apply_to(&self, element: &Element) {
                let ($($name,)*) = self;
                $($name.apply_to(element);)*
            }
        }
    };
}

for_all_tuples!(impl_element_attributes);

macro_rules! attributes {
    {
        $(
            $(#[$attr:meta])*
            $rust_name: ident => $html_name: expr,
        )*
    } => {
        $(
            $(#[$attr])*
            #[derive(Clone)]
            pub struct $rust_name<T: Value>(pub T)
            where
                T::Item: AsRef<str>;

            #[allow(deprecated)]
            impl<T: Value> Attribute for $rust_name<T>
            where
                T::Item: AsRef<str>,
            {
                #[inline]
                fn apply_to(&self, element: &Element) {
                    let element = element.clone();
                    self.0.for_each_forever(move |value| {
                        // It is safe to unwrap, provided the $html_name is valid.
                        element.set_attribute($html_name, value.as_ref()).unwrap();
                    });
                }
            }
        )*
    };
}

pub(crate) use attributes;

#[derive(Clone)]
pub struct CustomData<N: AsRef<str>, T: Value>(pub N, pub T)
where
    T::Item: AsRef<str>;

impl<N: AsRef<str>, T: Value> Attribute for CustomData<N, T>
where
    T::Item: AsRef<str>,
{
    #[inline]
    fn apply_to(&self, element: &Element) {
        let name = format!("data-{}", self.0.as_ref());
        let element = element.clone();
        self.1.for_each_forever(move |value| {
            element.set_attribute(&name, value.as_ref()).ok();
        });
    }
}
