use crate::prelude::*;

static mut BODY: Option<Component> = None;

#[inline]
pub fn app(style: Style) -> Component {
    unsafe {
        assert!(BODY.is_none());
        BODY.replace(Component::body(style));
        BODY.clone().unwrap()
    }
}