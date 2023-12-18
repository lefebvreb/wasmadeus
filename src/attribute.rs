use alloc::format;
use web_sys::Element;

use crate::signal::Value;
use crate::util::{for_all_tuples, TryAsRef};

pub trait Attribute: Sized {
    fn apply_to(&self, element: &Element);
}

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
                T::Item: TryAsRef<str>;

            #[allow(deprecated)]
            impl<T: Value> Attribute for $rust_name<T>
            where
                T::Item: TryAsRef<str>,
            {
                #[inline]
                fn apply_to(&self, element: &Element) {
                    let element = element.clone();
                    self.0.for_each_forever(move |value| {
                        // In both cases, it is safe to unwrap, provided the $html_name
                        // is a valid attribute name.
                        match value.try_as_ref() {
                            Some(value) => element.set_attribute($html_name, value).unwrap(),
                            None => element.remove_attribute($html_name).unwrap(),
                        }
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
    T::Item: TryAsRef<str>;

impl<N: AsRef<str>, T: Value> Attribute for CustomData<N, T>
where
    T::Item: TryAsRef<str>,
{
    #[inline]
    fn apply_to(&self, element: &Element) {
        let name = format!("data-{}", self.0.as_ref());
        let element = element.clone();
        self.1
            .for_each_forever(move |value| match value.try_as_ref() {
                Some(value) => element.set_attribute(&name, value).unwrap(),
                None => element.remove_attribute(&name).unwrap(),
            });
    }
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

macro_rules! impl_attributes {
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

for_all_tuples!(impl_attributes);
