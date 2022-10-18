//! Standard HTML components.
//! 
//! This module contains most of the elements of the HTML specification,
//! converted to wasmide [`Component`]s.

use alloc::string::ToString;

use crate::prelude::*;

/// A button component, will display it's `text` and call `on_click` when clicked.
/// 
/// Corresponds to an HTML [`<button>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Button) element.
/// 
/// # Examples
/// 
/// See the [counter example](https://github.com/L-Benjamin/wasmide/blob/main/examples/counter/src/main.rs) for a concrete use of this component.
pub fn button<S: ToString, F: FnMut() + 'static, C: Into<Option<F>>>(text: impl Subscribable<S>, on_click: C, style: Style) -> Component {
    let this = Component::new("button", style);
    this.set_inner_html(text);
    if let Some(callback) = on_click.into() {
        this.set_on_click(callback);
    }
    this
}

/// A division component, an invisible box that other components can be 
/// attached to, and styled as need be.
/// 
/// Corresponds to an HTML [`<div>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Div) element.
/// 
/// # Examples
/// 
/// See the [cards example](https://github.com/L-Benjamin/wasmide/tree/main/examples/cards) for a concrete use of this component.
pub fn div(style: Style) -> Component {
    Component::new("div", style)
}

/// A text component, displays the provided `text` with the given `style`.
/// 
/// Corresponds to an HTML [`<p>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p) element.
/// 
/// # Examples
/// 
/// See the ["Hello, world!" example](https://github.com/L-Benjamin/wasmide/blob/main/examples/hello-world/src/main.rs) for a concrete use of this component.
pub fn text<S: ToString>(text: impl Subscribable<S>, style: Style) -> Component {
    let this = Component::new("p", style);
    this.set_inner_html(text);
    this
}