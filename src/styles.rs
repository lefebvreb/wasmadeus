use web_sys::HtmlElement;

#[derive(Clone, Copy)]
pub struct Style(pub &'static str);

impl Style {
    pub const NONE: Self = Self("");
}