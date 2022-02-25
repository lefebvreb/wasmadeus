use core::cell::UnsafeCell;

use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::HtmlElement;

use crate::prelude::*;
use crate::stores::StoreUnsubscriber;

struct InternalComponent {
    element: HtmlElement,
    unsubs: UnsafeCell<Vec<StoreUnsubscriber>>,
    children: UnsafeCell<Vec<Component>>,
}

impl InternalComponent {
    #[inline]
    fn new(element: HtmlElement) -> Self {
        Self {
            element,
            unsubs: Vec::new().into(),
            children: Vec::new().into(),
        }
    }
}

impl Drop for InternalComponent {
    #[inline]
    fn drop(&mut self) {
        for unsub in self.unsubs.get_mut().drain(..) {
            unsub.unsubscribe();
        }
    }
}

#[derive(Clone)]
pub struct Component(Rc<InternalComponent>);

impl Component {
    // Constructs a new component with an html element of the given tag name.
    // Precondition : tag_name is a valid HTML tag name.
    #[inline]
    fn new(tag_name: &'static str, style: Style) -> Self {
        let element = web_sys::window().unwrap()
            .document().unwrap()
            .create_element(tag_name).unwrap()
            .dyn_into().unwrap();

        let this = Component(Rc::new(InternalComponent::new(element)));
        this.set_style(style);
        this        
    }

    // Appends a child to the component.
    #[inline]
    fn append(&self, child: Component) {
        self.html().append_child(child.html()).unwrap();
        unsafe { (*self.0.children.get()).push(child); }
    }

    #[inline]
    fn push_unsub(&self, unsub: StoreUnsubscriber) {
        unsafe { (*self.0.unsubs.get()).push(unsub); }
    }

    #[inline]
    pub(crate) fn body(style: Style) -> Self {
        let body = web_sys::window().unwrap()
            .document().unwrap()
            .body().unwrap();

        let this = Component(Rc::new(InternalComponent::new(body)));
        this.set_style(style);
        this 
    }

    #[inline]
    pub fn html(&self) -> &HtmlElement {
        &self.0.element
    }

    #[inline]
    pub fn downgrade(&self) -> WeakComponent {
        WeakComponent(Rc::downgrade(&self.0))
    }

    #[inline]
    pub fn set_style(&self, style: Style) {
        self.html().set_class_name(style.0);
    }

    #[inline]
    pub fn with(self, component: Component) -> Self {
        self.append(component);
        self
    }

    #[inline]
    pub fn with_if(
        self, 
        condition: impl Subscribable<bool>, 
        if_true: impl Fn() -> Component + 'static,
    ) -> Self {
        let this = self.downgrade();
        let mut comp = Children::new(if_true);

        let unsub = condition.subscribe(move |&condition| {
            if condition {
                comp.activate(&this);
            } else {
                comp.deactivate();
            }
        });

        self.push_unsub(unsub);
        self
    }

    #[inline]
    pub fn with_if_else(
        self, 
        condition: impl Subscribable<bool>, 
        if_true: impl Fn() -> Component + 'static, 
        if_false: impl Fn() -> Component + 'static,
    ) -> Self {
        let this = self.downgrade();
        let mut comp1 = Children::new(if_true);
        let mut comp2 = Children::new(if_false);

        let unsub = condition.subscribe(move |&condition| {
            if condition {
                comp1.activate(&this);
                comp2.deactivate();
            } else {
                comp1.deactivate();
                comp2.activate(&this);
            }
        });

        self.push_unsub(unsub);
        self
    }
    
    #[inline]
    pub fn div(style: Style) -> Component {
        let this = Component::new("div", style);
        this
    }

    #[inline]
    pub fn text<S: AsRef<str>>(text: impl Subscribable<S>, style: Style) -> Component {
        let this = Component::new("p", style);
        let cloned = this.downgrade();

        let unsub = text.subscribe(move |text| {
            if let Some(comp) = cloned.upgrade() {
                comp.html().set_inner_html(text.as_ref());
            }
        });

        this.push_unsub(unsub);
        this
    }

    #[inline]
    pub fn button<S: AsRef<str>>(text: impl Subscribable<S>, on_click: impl FnMut() + 'static, style: Style) -> Component {
        let this = Component::new("button", style);
        let cloned = this.downgrade();

        let unsub = text.subscribe(move |text| {
            if let Some(comp) = cloned.upgrade() {
                comp.html().set_inner_html(text.as_ref());
            }
        });

        let on_click = Closure::wrap(Box::new(on_click) as Box<dyn FnMut()>);
        this.html().set_onclick(Some(on_click.as_ref().unchecked_ref()));
        on_click.forget();

        this.push_unsub(unsub);
        this
    }
}

#[derive(Clone)]
pub struct WeakComponent(Weak<InternalComponent>);

impl WeakComponent {
    #[inline]
    pub fn upgrade(&self) -> Option<Component> {
        self.0.upgrade().map(Component)
    }
}

enum Children<F: FnOnce() -> Component> {
    Uninit(Option<F>),
    Init(Component),
}

impl<F: FnOnce() -> Component> Children<F> {
    #[inline]
    fn new(init: F) -> Self {
        Self::Uninit(Some(init))
    }

    #[inline]
    fn activate(&mut self, parent: &WeakComponent) {
        match self {
            Self::Uninit(init) => {
                let comp = init.take().unwrap()();
                parent.upgrade().unwrap().append(comp.clone());
                *self = Self::Init(comp);
            },
            Self::Init(ref comp) => {
                comp.html().set_hidden(false);
            }
        }
    }

    #[inline]
    fn deactivate(&self) {
        if let Self::Init(comp) = self {
            comp.html().set_hidden(true);
        }
    }
}