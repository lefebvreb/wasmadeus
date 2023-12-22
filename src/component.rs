use core::any::Any;
use core::cell::UnsafeCell;
use core::mem;

use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{CssStyleDeclaration, Element, HtmlElement, SvgElement};

use crate::attribute::Attributes;
use crate::signal::{Unsubscribe, Value};
use crate::utils::TryAsRef;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ElementNotFoundError;

#[derive(Debug)]
pub enum ElementKind {
    /// Standard html elements.
    Html(HtmlElement),
    /// Svg elements.
    Svg(SvgElement),
    /// MathML elements, future unknown elements.
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
    fn set_display<const B: bool>(&self) {
        if let Some(style) = self.style() {
            if B {
                style.set_property("display", "").ok();
            } else {
                style.set_property("display", "none").ok();
            }
        }
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
    pub fn style(&self) -> Option<&CssStyleDeclaration> {
        self.inner().style.as_ref()
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

    #[inline]
    pub fn text<T: Value>(self, text: T) -> Self
    where
        T::Item: TryAsRef<str>,
    {
        let node = web_sys::window().unwrap().document().unwrap().create_text_node("");
        self.as_element().append_child(&node).unwrap();
        let unsub = text.for_each(move |text| {
            node.set_text_content(text.try_as_ref());
        });
        self.push_dependency(unsub.droppable());
        self
    }

    #[inline]
    pub fn with(self, child: Component) -> Self {
        self.as_element().append_child(child.as_element()).unwrap();
        self.push_dependency(child);
        self
    }

    #[inline]
    pub fn with_if<C, F>(self, cond: C, if_true: F) -> Self
    where
        C: Value<Item = bool>,
        F: FnOnce() -> Component + 'static,
    {
        let weak = self.downgrade();
        let mut child = LazyChild::new(if_true);
        let unsub = cond.for_each(move |&cond| {
            child.display(cond, &weak);
        });
        self.push_dependency(unsub.droppable());
        self
    }

    #[inline]
    pub fn with_if_else<C, F, G>(self, cond: C, if_true: F, if_false: G) -> Self
    where
        C: Value<Item = bool>,
        F: FnOnce() -> Component + 'static,
        G: FnOnce() -> Component + 'static,
    {
        let weak = self.downgrade();
        let mut child1 = LazyChild::new(if_true);
        let mut child2 = LazyChild::new(if_false);
        let unsub = cond.for_each(move |&cond| {
            child1.display(cond, &weak);
            child2.display(!cond, &weak);
        });
        self.push_dependency(unsub.droppable());
        self
    }

    /// Adds a dependency to this component.
    ///
    /// The dependency will be dropped at the same time as the component. You most likely don't
    /// need to call this method directly.
    #[inline]
    pub fn push_dependency<T: Any>(&self, dep: T) {
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

struct LazyChild<F> {
    init: Option<F>,
    child: Option<Component>,
}

impl<F: FnOnce() -> Component> LazyChild<F> {
    #[inline]
    fn new(f: F) -> Self {
        Self {
            init: Some(f),
            child: None,
        }
    }

    #[inline]
    fn display(&mut self, display: bool, parent: &WeakComponent) {
        if display {
            match (parent.upgrade(), &self.child) {
                (Some(_), Some(child)) => child.set_display::<true>(),
                (Some(parent), _) => {
                    let new_child = (self.init.take().unwrap())();
                    parent.with(new_child.clone());
                    self.child = Some(new_child);
                }
                _ => (),
            }
        } else if let Some(child) = &self.child {
            child.set_display::<false>()
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
