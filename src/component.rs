use core::any::Any;
use core::cell::UnsafeCell;
use core::fmt;
use core::mem;

use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{CssStyleDeclaration, Element, HtmlElement, SvgElement};

use crate::attribute::Attributes;
use crate::signal::{Unsubscribe, Value};
use crate::view::View;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ElementNotFoundError;

impl fmt::Display for ElementNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "element not found")
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OrphanElementError;

impl fmt::Display for OrphanElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "element has no parent")
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ElementKind {
    /// Standard html element.
    Html(HtmlElement),
    /// Svg element.
    Svg(SvgElement),
    /// MathML element or other unknown element.
    Other(Element),
}

#[derive(Debug)]
struct ComponentInner {
    element: ElementKind,
    style: Option<CssStyleDeclaration>,
    deps: UnsafeCell<Vec<Box<dyn Any>>>,
}

#[derive(Clone, Debug)]
pub struct Component(Rc<ComponentInner>);

impl Component {
    #[inline]
    fn inner(&self) -> &ComponentInner {
        &self.0
    }

    #[inline]
    pub fn new<A: Attributes>(tag: &str, attributes: A) -> Component {
        // Create element from DOM window.
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(tag)
            .unwrap();

        // Get element subclass.
        let element = Err(element)
            .or_else(|element| element.dyn_into::<HtmlElement>().map(ElementKind::Html))
            .or_else(|element| element.dyn_into::<SvgElement>().map(ElementKind::Svg))
            .unwrap_or_else(ElementKind::Other);

        // Get element style sheet.
        let style = match &element {
            ElementKind::Html(html) => Some(html.style()),
            ElementKind::Svg(svg) => Some(svg.style()),
            ElementKind::Other(_) => None,
        };

        // Create component.
        let this = Self(Rc::new(ComponentInner {
            element,
            style,
            deps: Default::default(),
        }));

        // Apply attributes and return.
        attributes.apply_to(&this);
        this
    }

    #[inline]
    pub fn downgrade(&self) -> WeakComponent {
        WeakComponent(Rc::downgrade(&self.0))
    }

    #[inline]
    pub fn element_kind(&self) -> &ElementKind {
        &self.inner().element
    }

    #[inline]
    pub fn as_element(&self) -> &Element {
        match &self.inner().element {
            ElementKind::Html(html) => html,
            ElementKind::Svg(svg) => svg,
            ElementKind::Other(other) => other,
        }
    }

    #[inline]
    pub fn as_html_element(&self) -> Option<&HtmlElement> {
        match &self.inner().element {
            ElementKind::Html(html) => Some(html),
            _ => None,
        }
    }

    #[inline]
    pub fn as_svg_element(&self) -> Option<&SvgElement> {
        match &self.inner().element {
            ElementKind::Svg(svg) => Some(svg),
            _ => None,
        }
    }

    #[inline]
    pub fn style(&self) -> Option<&CssStyleDeclaration> {
        self.inner().style.as_ref()
    }

    #[inline]
    pub fn set_visible<T: Value<Item = bool>>(&self, visible: T) {
        if let Some(style) = self.style().cloned() {
            let unsub = visible.for_each(move |&visible| {
                style.set_property("display", if visible { "" } else { "none" }).ok();
            });
            self.push_dependency(unsub.droppable());
        }
    }

    #[inline]
    pub fn has_parent(&self) -> bool {
        self.as_element().parent_node().is_some()
    }

    /// Attaches `self` to the result of the DOM function [`document.querySelector(selectors)`](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector).
    ///
    /// This method should be used as the entry point of your app, linking your components in the rust world to your HTML file.
    ///
    /// # Memory leaks
    ///
    /// Calling this method will [`forget`](core::mem::forget) `self`, to prevent it and its dependencies from
    /// being dropped. Coincidentally, this leaks memory.
    #[inline]
    pub fn attach_to(&self, selectors: &str) -> Result<(), ElementNotFoundError> {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector(selectors)
            .ok()
            .flatten()
            .ok_or(ElementNotFoundError)?
            .append_child(self.as_element())
            .unwrap();

        mem::forget(self.clone());
        Ok(())
    }

    /// Attaches `self` to the [`<body>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body) of the document.
    ///
    /// This method should be used as the entry point of your app, linking your components in the rust world to your HTML file.
    ///
    /// # Memory leaks
    ///
    /// Calling this method will [`forget`](core::mem::forget) `self`, to prevent it and its dependencies from
    /// being dropped. Coincidentally, this leaks memory.
    #[inline]
    pub fn attach_to_body(&self) -> Result<(), ElementNotFoundError> {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(self.as_element())
            .unwrap();

        mem::forget(self.clone());
        Ok(())
    }

    #[inline]
    pub fn with<V: View>(&self, view: V) -> &Self {
        view.update(self, &mut V::State::default());
        self
    }

    /// Adds a dependency to this component.
    ///
    /// The dependency will be dropped at the same time as the component. You most likely don't
    /// need to call this method directly.
    ///
    /// If `T` does not need to be dropped, calling this method is reduced to a noop.
    #[inline]
    pub fn push_dependency<T: 'static>(&self, dep: T) {
        if mem::needs_drop::<T>() {
            // SAFETY: deps is never borrowed ans Component is !Send.
            let deps = unsafe { &mut *self.inner().deps.get() };
            deps.push(Box::new(dep));
        }
    }
}

#[derive(Clone, Debug)]
pub struct WeakComponent(Weak<ComponentInner>);

impl WeakComponent {
    #[inline]
    pub fn upgrade(&self) -> Option<Component> {
        Weak::upgrade(&self.0).map(Component)
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
            pub fn $rust_name<A: $crate::attribute::Attributes>(attributes: A) -> $crate::component::Component {
                $crate::component::Component::new($html_name, attributes)
            }
        )*
    };
}

pub(crate) use elements;

#[test]
fn test() {
    Component::new("div", ()).child("My div is cool");
}
