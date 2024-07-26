use alloc::borrow::Cow;
use alloc::string::String;
use web_sys::{Element, Text};

use crate::component::Component;
use crate::signal::{Unsubscribe, Value};
use crate::utils::for_all_tuples;

mod utils {
    use web_sys::{Element, Text};

    use crate::component::Component;

    #[inline]
    pub fn text_node(value: &str, parent: &Component) -> Text {
        let node = web_sys::window().unwrap().document().unwrap().create_text_node(value);
        parent.as_element().append_child(&node).unwrap();
        node
    }

    #[inline]
    pub fn placeholder_div(parent: &Component) -> Element {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_attribute("display", "none").unwrap();
        parent.as_element().append_child(&div).unwrap();
        div
    }

    #[inline]
    pub fn swap_elements(parent: &Element, old: &Element, new: &Element) {
        parent.insert_before(new, Some(old)).unwrap();
        parent.remove_child(old).unwrap();
    }
}

pub trait View {
    fn init(&self, parent: &Component);
}

pub trait UpdateableView {
    type State;

    fn init(parent: &Component) -> Self::State;

    fn update(&self, parent: &Component, state: &mut Self::State);
}

impl<T> View for T
where
    T: Value + 'static,
    T::Item: UpdateableView,
{
    #[inline]
    fn init(&self, parent: &Component) {
        let weak = parent.downgrade();
        let mut child_state = <T::Item as UpdateableView>::init(parent);
        let unsub = self.for_each(move |value| {
            if let Some(parent) = weak.upgrade() {
                value.update(&parent, &mut child_state);
            }
        });
        parent.push_dependency(unsub.droppable());
    }
}

impl View for &str {
    #[inline]
    fn init(&self, parent: &Component) {
        utils::text_node(self, parent);
    }
}

impl UpdateableView for &str {
    type State = Text;

    #[inline]
    fn init(parent: &Component) -> Self::State {
        utils::text_node("", parent)
    }

    #[inline]
    fn update(&self, _: &Component, state: &mut Self::State) {
        state.set_text_content(Some(self));
    }
}

impl View for String {
    #[inline]
    fn init(&self, parent: &Component) {
        <&str as View>::init(&self.as_str(), parent);
    }
}

impl UpdateableView for String {
    type State = Text;

    #[inline]
    fn init(parent: &Component) -> Self::State {
        <&str as UpdateableView>::init(parent)
    }

    #[inline]
    fn update(&self, parent: &Component, state: &mut Self::State) {
        <&str as UpdateableView>::update(&self.as_str(), parent, state);
    }
}

impl View for Cow<'_, str> {
    #[inline]
    fn init(&self, parent: &Component) {
        <&str as View>::init(&self.as_ref(), parent);
    }
}

impl UpdateableView for Cow<'_, str> {
    type State = Text;

    #[inline]
    fn init(parent: &Component) -> Self::State {
        <&str as UpdateableView>::init(parent)
    }

    #[inline]
    fn update(&self, parent: &Component, state: &mut Self::State) {
        <&str as UpdateableView>::update(&self.as_ref(), parent, state);
    }
}

impl View for Option<&str> {
    #[inline]
    fn init(&self, parent: &Component) {
        <&str as View>::init(&self.unwrap_or_default(), parent);
    }
}

impl UpdateableView for Option<&str> {
    type State = Text;

    #[inline]
    fn init(parent: &Component) -> Self::State {
        <&str as UpdateableView>::init(parent)
    }

    #[inline]
    fn update(&self, parent: &Component, state: &mut Self::State) {
        <&str as UpdateableView>::update(&self.unwrap_or_default(), parent, state);
    }
}

impl View for Option<String> {
    #[inline]
    fn init(&self, parent: &Component) {
        <Option<&str> as View>::init(&self.as_deref(), parent);
    }
}

impl UpdateableView for Option<String> {
    type State = Text;

    #[inline]
    fn init(parent: &Component) -> Self::State {
        <Option<&str> as UpdateableView>::init(parent)
    }

    #[inline]
    fn update(&self, parent: &Component, state: &mut Self::State) {
        <Option<&str> as UpdateableView>::update(&self.as_deref(), parent, state);
    }
}

impl View for Option<Cow<'_, str>> {
    #[inline]
    fn init(&self, parent: &Component) {
        <Option<&str> as View>::init(&self.as_deref(), parent);
    }
}

impl UpdateableView for Option<Cow<'_, str>> {
    type State = Text;

    #[inline]
    fn init(parent: &Component) -> Self::State {
        <Option<&str> as UpdateableView>::init(parent)
    }

    #[inline]
    fn update(&self, parent: &Component, state: &mut Self::State) {
        <Option<&str> as UpdateableView>::update(&self.as_deref(), parent, state);
    }
}

impl View for Component {
    #[inline]
    fn init(&self, parent: &Component) {
        parent.as_element().append_child(&self.as_element()).unwrap();
        parent.push_dependency(self.clone());
    }
}

impl UpdateableView for Component {
    type State = Element;

    #[inline]
    fn init(parent: &Component) -> Self::State {
        utils::placeholder_div(parent)
    }

    #[inline]
    fn update(&self, parent: &Component, state: &mut Self::State) {
        utils::swap_elements(parent.as_element(), self.as_element(), state);
        *state = self.as_element().clone();
    }
}

pub struct If<C, F>(pub C, pub F);

impl<C, F> View for If<C, F>
where
    C: Value<Item = bool>,
    F: FnOnce() -> Component,
{
    fn init(&self, parent: &Component) {
        let weak = parent.downgrade();
        let placeholder = utils::placeholder_div(parent);
        let unsub = self.0.for_each(move |&cond| {
            if let Some(parent) = weak.upgrade() {
                if cond {
                    utils::swap_elements(parent.as_element(), todo!(), todo!());
                }
            }
        });
        parent.push_dependency(unsub.droppable());
    }
}

macro_rules! impl_view {
    ($($name: ident)*) => {
        impl<$($name: View,)*> View for ($($name,)*) {
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn init(&self, parent: &Component) {
                let ($($name,)*) = self;
                $($name.init(parent);)*
            }
        }

        // #[allow(non_snake_case, unused_variables)]
        // impl<$($name: UpdateableView,)*> UpdateableView for ($($name,)*) {
        //     type State = ($($name::State,)*);

        //     #[inline]
        //     fn init(parent: &Component) -> Self::State {
        //         ($($name::init(parent),)*)
        //     }

        //     #[inline]
        //     fn update(&self, parent: &Component, state: &mut Self::State) {
        //         todo!()
        //     }
        // }
    };
}

for_all_tuples!(impl_view);
