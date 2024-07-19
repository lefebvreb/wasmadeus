use alloc::string::String;
use web_sys::{Element, Text};

use crate::component::Component;
use crate::signal::{Unsubscribe, Value};
use crate::utils::for_all_tuples;

pub trait View {
    type State: Default;

    fn update(&self, parent: &Component, state: &mut Self::State);
}

impl<T> View for T
where
    T: Value + 'static,
    T::Item: View,
{
    type State = ();

    #[inline]
    fn update(&self, parent: &Component, _: &mut ()) {
        let weak = parent.downgrade();
        let mut state = <T::Item as View>::State::default();
        let unsub = self.for_each(move |value| {
            if let Some(parent) = weak.upgrade() {
                value.update(&parent, &mut state);
            }
        });
        parent.push_dependency(unsub.droppable());
    }
}

impl View for &str {
    type State = Option<Text>;

    #[inline]
    fn update(&self, parent: &Component, state: &mut Option<Text>) {
        match state {
            Some(node) => node.set_text_content(Some(self)),
            None => {
                let node = web_sys::window().unwrap().document().unwrap().create_text_node(self);
                parent.as_element().append_child(&node).unwrap();
                *state = Some(node);
            }
        }
    }
}

impl View for String {
    type State = Option<Text>;

    #[inline]
    fn update(&self, parent: &Component, state: &mut Option<Text>) {
        self.as_str().update(parent, state);
    }
}

impl View for Option<&str> {
    type State = Option<Text>;

    #[inline]
    fn update(&self, parent: &Component, state: &mut Option<Text>) {
        match state {
            Some(node) => node.set_text_content(*self),
            None => {
                let node = web_sys::window().unwrap().document().unwrap().create_text_node(self);
                parent.as_element().append_child(&node).unwrap();
                *state = Some(node);
            }
        }
    }
}

impl View for Option<String> {
    type State = Option<Text>;

    #[inline]
    fn update(&self, parent: &Component, state: &mut Option<Text>) {
        self.as_deref().update(parent, state);
    }
}

impl View for Component {
    type State = Option<Element>;

    #[inline]
    fn update(&self, parent: &Component, state: &mut Option<Element>) {
        todo!()
    }
}

macro_rules! impl_view {
    ($($name: ident)*) => {
        impl<$($name: View,)*> View for ($($name,)*) {
            type State = ();

            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn update(&self, component: &Component, _: &mut ()) {
                let ($($name,)*) = self;
                $($name.update(component, &mut $name::State::default());)*
            }
        }
    };
}

for_all_tuples!(impl_view);
