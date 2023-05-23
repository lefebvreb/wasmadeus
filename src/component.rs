use web_sys::Element;

pub trait Component {
    fn append_to(self, parent: Element);
}
