use core::cell::UnsafeCell;

use alloc::string::String;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::prelude::Subscribable;
use crate::styles::Style;

pub struct Component {
    html_element: HtmlElement,
}

impl Component {
    // Constructs a new component with an html element of the given tag name.
    // Precondition : tag_name is a valid HTML tag name.
    #[inline]
    fn new(tag_name: &'static str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.create_element(tag_name).unwrap();

        Component {
            html_element: element.dyn_into::<HtmlElement>().unwrap(),
        }
    }

    // Appends a child to the component.
    #[inline]
    fn append(&self, child: &Component) {
        self.html_element.append_child(&child.html_element).unwrap();
    }

    #[inline]
    pub fn set_hidden(&self, hidden: bool) {
        self.html_element.set_hidden(hidden);
    }

    #[inline]
    pub fn with(&self, component: Component) {
        self.append(&component);
    }

    #[inline]
    pub fn with_if(
        &'static self, 
        condition: impl Subscribable<bool>, 
        if_true: impl Fn() -> Component + 'static,
    ) {
        let comp: UnsafeCell<Option<Component>> = UnsafeCell::new(None);

        let unsub = condition.subscribe(move |&condition| unsafe {
            if condition {
                if let Some(component) = &*comp.get() {
                    component.set_hidden(false);
                } else {
                    let component = if_true();
                    self.append(&component);
                    *comp.get() = Some(component);
                }
            } else {
                if let Some(component) = &*comp.get() {
                    component.set_hidden(true);
                }
            }
        });
        
        todo!();
    }

    #[inline]
    pub fn with_if_else(
        &self, 
        condition: impl Subscribable<bool>, 
        if_true: impl Fn() -> Component + 'static, 
        if_false: impl Fn() -> Component + 'static,
    ) {
        todo!()
    }
}

#[inline]
pub fn text(text: impl Subscribable<String>, style: &Style) -> Component {
    todo!()
}

#[inline]
pub fn div(style: &Style) -> Component {
    todo!()
}