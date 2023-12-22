use alloc::format;

use crate::component::Component;
use crate::signal::Value;
use crate::utils::{for_all_tuples, TryAsRef};

pub trait Attribute: Sized {
    fn apply_to(&self, component: &Component);
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
            pub struct $rust_name<T: $crate::signal::Value>(pub T)
            where
                T::Item: $crate::utils::TryAsRef<str>;

            #[allow(deprecated)]
            impl<T: $crate::signal::Value> $crate::attribute::Attribute for $rust_name<T>
            where
                T::Item: $crate::utils::TryAsRef<str>,
            {
                #[inline]
                fn apply_to(&self, component: &$crate::component::Component) {
                    use $crate::utils::TryAsRef;
                    let element = component.as_element().clone();
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
    fn apply_to(&self, component: &Component) {
        let name = format!("data-{}", self.0.as_ref());
        let element = component.as_element().clone();
        self.1.for_each_forever(move |value| match value.try_as_ref() {
            Some(value) => element.set_attribute(&name, value).unwrap(),
            None => element.remove_attribute(&name).unwrap(),
        });
    }
}

pub trait Attributes: Sized {
    fn apply_to(&self, component: &Component);
}

impl<T: Attribute> Attributes for T {
    #[inline]
    fn apply_to(&self, component: &Component) {
        self.apply_to(component);
    }
}

macro_rules! impl_attributes {
    ($($name: ident)*) => {
        impl<$($name: Attributes,)*> Attributes for ($($name,)*) {
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn apply_to(&self, component: &Component) {
                let ($($name,)*) = self;
                $($name.apply_to(component);)*
            }
        }
    };
}

for_all_tuples!(impl_attributes);
