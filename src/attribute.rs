use alloc::format;
use web_sys::Element;

use crate::signal::Value;
use crate::utils::for_all_tuples;

pub trait Attribute: Sized {
    fn apply_to(&self, element: &Element);
}

pub trait ElementAttributes: Sized {
    fn apply_to(&self, element: &Element);
}

macro_rules! impl_element_attributes {
    ($($name: ident)*) => {
        impl<$($name: Attribute,)*> ElementAttributes for ($($name,)*) {
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
    { $($name: ident => $code: expr,)* } => {
        $(
            #[derive(Clone)]
            pub struct $name<T: Value>(pub T)
            where
                T::Item: AsRef<str>;

            impl<T: Value> Attribute for $name<T>
            where
                T::Item: AsRef<str>,
            {
                #[inline]
                fn apply_to(&self, element: &Element) {
                    let element = element.clone();
                    self.0.for_each_forever(move |value| {
                        $code(&element, value.as_ref());
                        //element.set_class_name(value.as_ref());
                    });
                }
            }
        )*
    };
}

attributes! {
    Class => Element::set_class_name,
}

#[derive(Clone)]
pub struct Data<S: AsRef<str>, T: Value>(pub S, pub T)
where
    T::Item: AsRef<str>;

impl<S: AsRef<str>, T: Value> Attribute for Data<S, T>
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
