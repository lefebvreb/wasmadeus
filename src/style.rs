//! Styles for components.
//! 
//! Styling is done with [tailwindcss](https://tailwindcss.com/) in mind.
//! Giving classes to components is done with the [`Style`] type.

/// A list of styles for a component.
/// 
/// This type is used to give a component a list of [tailwindcss](https://tailwindcss.com/)
/// utility classes.
/// 
/// # Examples
/// 
/// ```
/// # use wasmide::prelude::*;
/// const MY_BUTTON: Style = Style("bg-white rounded border-black border-2 py-3 px-5 hover:bg-gray-200");
/// ```
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Style(pub &'static str);

impl Style {
    /// An empty [`Style`] item.
    pub const NONE: Self = Self("");
}