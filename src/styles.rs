#[derive(Copy, Clone)]
pub struct Style(&'static str);

impl Style {
    pub const NONE: Self = Self("");

    #[doc(hidden)]
    pub const fn __new(style: &'static str) -> Self {
        Style(style)
    }

    #[inline]
    pub(crate) fn class_name(&self) -> &str {
        self.0
    }
}