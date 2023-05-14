use web_sys::HtmlElement;

use crate::utils::all_tuples;

pub trait Attribute {
    fn apply(self, element: &HtmlElement);
}

impl Attribute for () {
    #[inline]
    fn apply(self, _: &HtmlElement) {}
}

macro_rules! impl_attribute {
    ($($name: ident)*) => {
        impl<$($name: Attribute,)*> Attribute for ($($name,)*) {
            #[allow(non_snake_case)]
            fn apply(self, element: &HtmlElement) {
                let ($($name,)*) = self;
                $($name.apply(element);)*
            }
        }
    };
}

all_tuples!(impl_attribute);
