use alloc::borrow::Cow;
use alloc::string::String;

use crate::component::Component;
use crate::signal::{DropUnsubscriber, Unsubscribe, Value};
use crate::utils::for_all_tuples;

// fn placeholder() -> Element {
//     let div = web_sys::window()
//         .unwrap()
//         .document()
//         .unwrap()
//         .create_element("div")
//         .unwrap();
//     div.set_attribute("display", "none").unwrap();
//     div
// }

pub trait View {
    type State: 'static;

    fn init(&self, parent: &Component) -> Self::State;
}

// Below: nonesense!
// impl<T> View for T
// where
//     T: Value + 'static,
//     T::Item: View,
// {
//     type State = Option<DropUnsubscriber<T::Unsubscriber>>;

//     #[inline]
//     fn update(&self, parent: &Component) -> Self::State {
//         let weak = parent.downgrade();
//         let mut child_state = <T::Item as View>::State::default();
//         let unsub = self.for_each(move |value| {
//             if let Some(parent) = weak.upgrade() {
//                 value.update(&parent, &mut child_state);
//             }
//         });
//         *state = Some(unsub.droppable());
//     }
// }

impl View for &str {
    type State = ();

    #[inline]
    fn init(&self, parent: &Component) -> Self::State {
        let node = web_sys::window().unwrap().document().unwrap().create_text_node(self);
        parent.as_element().append_child(&node).unwrap();
    }
}

impl View for String {
    type State = ();

    #[inline]
    fn init(&self, parent: &Component) -> Self::State {
        self.as_str().init(parent);
    }
}

impl View for Cow<'_, str> {
    type State = ();

    #[inline]
    fn init(&self, parent: &Component) -> Self::State {
        self.as_ref().init(parent);
    }
}

impl View for Option<&str> {
    type State = ();

    #[inline]
    fn init(&self, parent: &Component) -> Self::State {
        self.unwrap_or_default().init(parent);
    }
}

impl View for Option<String> {
    type State = ();

    #[inline]
    fn init(&self, parent: &Component) -> Self::State {
        self.as_deref().init(parent);
    }
}

impl View for Option<Cow<'_, str>> {
    type State = ();

    #[inline]
    fn init(&self, parent: &Component) -> Self::State {
        self.as_deref().init(parent);
    }
}

impl View for Component {
    type State = Component;

    #[inline]
    fn init(&self, parent: &Component) -> Self::State {
        parent.as_element().append_child(&self.as_element()).unwrap();
        self.clone()
    }
}

pub struct If<C, F>(pub C, pub F);

impl<C, F> View for If<C, F>
where
    C: Value<Item = bool>,
    F: FnOnce() -> Component,
{
    type State = DropUnsubscriber<C::Unsubscriber>;

    fn init(&self, parent: &Component) -> Self::State {
        let unsub = self.0.for_each(|cond| {});
        unsub.droppable()
    }
}

macro_rules! impl_view {
    ($($name: ident)*) => {
        impl<$($name: View,)*> View for ($($name,)*) {
            type State = ($($name::State,)*);

            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn init(&self, parent: &Component) -> Self::State {
                let ($($name,)*) = self;
                ($($name.init(parent),)*)
            }
        }
    };
}

for_all_tuples!(impl_view);
