use core::cell::UnsafeCell;
use core::mem;

use alloc::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::prelude::Subscribable;
use crate::styles::Style;

enum InternalLazyComponent<F: FnOnce() -> Component> {
    Uninit(F),
    Init(Component),
}

struct LazyComponent<F: FnOnce() -> Component>(UnsafeCell<InternalLazyComponent<F>>);

impl<F: FnOnce() -> Component> LazyComponent<F> {
    #[inline]
    fn new(init: F) -> Self {
        Self(InternalLazyComponent::Uninit(init).into())
    }

    #[inline]
    fn internal(&self) -> &mut InternalLazyComponent<F> {
        unsafe { &mut *self.0.get() }
    }

    #[inline]
    fn activate(&self, parent: &Component) {
        let internal = self.internal();

        match internal {
            InternalLazyComponent::Uninit(init) => {
                mem::replace(internal, InternalLazyComponent::Init(init()));
            }
            InternalLazyComponent::Init(component) => {
                component.set_hidden(false);
            }
        }
    }

    #[inline]
    fn deactivate(&self) {
        if let InternalLazyComponent::Init(component) = self.internal() {
            component.set_hidden(true);
        }
    }
}

#[derive(Clone)]
pub struct Component(Rc<HtmlElement>);

impl Component {
    // Constructs a new component with an html element of the given tag name.
    // Precondition : tag_name is a valid HTML tag name.
    #[inline]
    fn new(tag_name: &'static str) -> Self {
        let element = web_sys::window().unwrap()
            .document().unwrap()
            .create_element(tag_name).unwrap()
            .dyn_into::<HtmlElement>().unwrap();

        Component(Rc::new(element))
    }

    // Appends a child to the component.
    #[inline]
    fn append(&self, child: &Component) {
        self.0.append_child(&child.0).unwrap();
    }

    #[inline]
    pub fn set_hidden(&self, hidden: bool) {
        self.0.set_hidden(hidden);
    }

    #[inline]
    pub fn set_style(&self, style: Style) {
        self.0.set_class_name(style.class_name());
    }

    #[inline]
    pub fn with(self, component: Component) -> Self {
        self.append(&component);
        self
    }

    #[inline]
    pub fn with_if(
        self, 
        condition: impl Subscribable<bool>, 
        if_true: impl Fn() -> Component + 'static,
    ) -> Self {
        /*let this = self.clone();
        let comp = LazyComponent::default();

        condition.subscribe(move |&condition| {
            if condition {
                comp.activate(&this, &if_true);
            } else {
                comp.deactivate();
            }
        });*/

        self
    }

    #[inline]
    pub fn with_if_else(
        self, 
        condition: impl Subscribable<bool>, 
        if_true: impl Fn() -> Component + 'static, 
        if_false: impl Fn() -> Component + 'static,
    ) -> Self {
        /*let this = self.clone();
        let comp1 = LazyComponent::default();
        let comp2 = LazyComponent::default();

        condition.subscribe(move |&condition| {
            if condition {
                comp1.activate(&this, &if_true);
                comp2.deactivate();
            } else {
                comp1.deactivate();
                comp2.activate(&this, &if_false);
            }
        });*/

        self
    }
    
    #[inline]
    pub fn div(style: Style) -> Component {
        let this = Component::new("div");
        this.set_style(style);
        this
    }

    #[inline]
    pub fn text<S: AsRef<str>>(text: impl Subscribable<S>, style: Style) -> Component {
        todo!()
    }
}