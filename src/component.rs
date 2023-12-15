use web_sys::Element;

use crate::attribute::Attributes;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ElementNotFoundError;

#[derive(Clone)]
pub struct Component {
    element: Element,
}

impl Component {
    pub fn new<A: Attributes>(tag: &str, attributes: A) -> Component {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(tag)
            .unwrap();

        attributes.apply_to(&element);

        Component { element }
    }

    pub fn as_element(&self) -> &Element {
        &self.element
    }

    pub fn attach_to(self, selectors: &str) -> Result<(), ElementNotFoundError> {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector(selectors)
            .ok()
            .flatten()
            .ok_or_else(|| ElementNotFoundError)?
            .append_child(&self.as_element())
            .unwrap();

        Ok(())
    }

    pub fn with(self, component: Component) -> Self {
        self.element.append_child(&component.element).unwrap();
        self
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
