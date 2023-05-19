use web_sys::HtmlElement;

pub trait Component {
    fn append_to(self, parent: HtmlElement);
}
