use web_sys::Element;

use crate::attribute::Attributes;

#[derive(Clone)]
pub struct Component {
    element: Element,
}

impl Component {
    pub(crate) fn new<A: Attributes>(tag: &str, attributes: A) -> Component {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(tag)
            .unwrap();

        attributes.apply_to(&element);

        Component { element }
    }

    pub fn element(&self) -> &Element {
        &self.element
    }
}

macro_rules! elements {
    {
        $(
            $(#[$attr:meta])*
            $rust_name: ident => $html_name: expr,
        )*
    } => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $rust_name<A: Attributes>(attributes: A) -> Component {
                Component::new($html_name, attributes)
            }
        )*
    };
}

pub(crate) use elements;
