use core::any::Any;
use core::cell::UnsafeCell;
use core::mem;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::attribute::Attributes;
use crate::signal::Value;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ElementNotFoundError;

#[derive(Debug)]
struct ComponentInner {
    element: Element,
    deps: UnsafeCell<Vec<Box<dyn Any>>>,
}

#[derive(Clone, Debug)]
pub struct Component(Rc<ComponentInner>);

impl Component {
    fn inner(&self) -> &ComponentInner {
        &self.0
    }

    pub fn new<A: Attributes>(tag: &str, attributes: A) -> Component {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(tag)
            .unwrap();

        let this = Self(Rc::new(ComponentInner {
            element,
            deps: Default::default(),
        }));

        attributes.apply_to(&this);
        this
    }

    pub fn as_element(&self) -> &Element {
        &self.inner().element
    }

    pub fn as_html_element(&self) -> Option<&HtmlElement> {
        self.inner().element.dyn_ref::<HtmlElement>()
    }

    /// Attaches `self` to the result of the JS function [`document.querySelector`](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector).
    ///
    /// This method should be used as the entry point of your app, linking your components in the rust world to your HTML file.
    ///
    /// # Memory leaks
    ///
    /// Calling this method will [`forget`](core::mem::forget) `self`, to prevent it and its dependencies from
    /// being dropped. Coincidentally, this leaks memory.
    pub fn attach_to(self, selectors: &str) -> Result<(), ElementNotFoundError> {
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
        mem::forget(self);
        Ok(())
    }

    pub fn with(self, child: Component) -> Self {
        self.as_element().append_child(child.as_element()).unwrap();
        self.push_dependency(child);
        self
    }

    pub fn with_if<C, F>(self, _cond: C, _if_true: F) -> Self
    where
        C: Value<Item = bool>,
        F: FnOnce() -> Component,
    {
        todo!()
    }

    pub fn with_if_else<C, F, G>(self, _cond: C, _if_true: F, _if_false: G) -> Self
    where
        C: Value<Item = bool>,
        F: FnOnce() -> Component,
        G: FnOnce() -> Component,
    {
        todo!()
    }

    /// Adds a dependency to this component.
    ///
    /// The dependency will be dropped at the same time as the component. You most likely don't
    /// need to call this method directly.
    pub fn push_dependency<T: Any>(&self, dep: T) {
        if mem::needs_drop::<T>() {
            // SAFETY: deps is never borrowed ans Component is !Send.
            let deps = unsafe { &mut *self.inner().deps.get() };
            deps.push(Box::new(dep));
        }
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
