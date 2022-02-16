pub struct Style(&'static str);

impl Style {
    #[doc(hidden)]
    pub const fn __new(style: &'static str) -> Self {
        Style(style)
    }

    #[inline]
    pub(crate) fn as_str(&self) -> &str {
        self.0
    }
}