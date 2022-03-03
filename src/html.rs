//! Standard HTML components.
//! 
//! This module contains most of the elements of the HTML specification,
//! converted to wasmide [`Component`]s.

use alloc::string::ToString;

use crate::prelude::*;

#[inline]
pub fn button<S: ToString>(text: impl Subscribable<S>, on_click: impl FnMut() + 'static, style: Style) -> Component {
    let this = Component::new("button", style);
    this.set_inner_html(text);
    this.set_on_click(on_click);
    this
}

#[inline]
pub fn div(style: Style) -> Component {
    Component::new("div", style)
}

#[inline]
pub fn p<S: ToString>(text: impl Subscribable<S>, style: Style) -> Component {
    let this = Component::new("p", style);
    this.set_inner_html(text);
    this
}