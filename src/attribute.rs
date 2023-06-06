use alloc::format;
use web_sys::Element;

use crate::signal::Value;
use crate::utils::for_all_tuples;

pub trait Attribute: Sized {
    fn apply_to(&self, element: &Element);
}

pub trait Attributes: Sized {
    fn apply_to(&self, element: &Element);
}

macro_rules! impl_element_attributes {
    ($($name: ident)*) => {
        impl<$($name: Attribute,)*> Attributes for ($($name,)*) {
            #[allow(non_snake_case, unused_variables)]
            fn apply_to(&self, element: &Element) {
                let ($($name,)*) = self;
                $($name.apply_to(element);)*
            }
        }
    };
}

for_all_tuples!(impl_element_attributes);

macro_rules! attributes {
    {
        $(
            $(#[$attr:meta])*
            $rust_name: ident => $html_name: expr,
        )*
    } => {
        $(
            $(#[$attr])*
            #[derive(Clone)]
            pub struct $rust_name<T: Value>(pub T)
            where
                T::Item: AsRef<str>;

            impl<T: Value> Attribute for $rust_name<T>
            where
                T::Item: AsRef<str>,
            {
                #[inline]
                fn apply_to(&self, element: &Element) {
                    let element = element.clone();
                    self.0.for_each_forever(move |value| {
                        // It is safe to unwrap, provided the $html_name is valid.
                        element.set_attribute($html_name, value.as_ref()).unwrap();
                    });
                }
            }
        )*
    };
}

#[derive(Clone)]
pub struct CustomData<N: AsRef<str>, T: Value>(pub N, pub T)
where
    T::Item: AsRef<str>;

impl<N: AsRef<str>, T: Value> Attribute for CustomData<N, T>
where
    T::Item: AsRef<str>,
{
    #[inline]
    fn apply_to(&self, element: &Element) {
        let name = format!("data-{}", self.0.as_ref());
        let element = element.clone();
        self.1.for_each_forever(move |value| {
            element.set_attribute(&name, value.as_ref()).ok();
        });
    }
}

// Programmatically gathered from https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes.
attributes! {
    /// List of types the server accepts, typically a file type.
    ///
    /// Can be applied to the following HTML elements: `<form>`, `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/accept)
    Accept => "accept",
    /// List of supported charsets.
    ///
    /// Can be applied to the following HTML elements: `<form>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#accept-charset)
    AcceptCharset => "accept-charset",
    /// Keyboard shortcut to activate or add focus to the element.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/accesskey)
    AccessKey => "accesskey",
    /// The URI of a program that processes the information submitted via the
    /// form.
    ///
    /// Can be applied to the following HTML elements: `<form>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/action)
    Action => "action",
    /// Specifies the horizontal alignment of the element.
    ///
    /// Can be applied to the following HTML elements: `<caption>`, `<col>`, `<colgroup>`, `<hr>`, `<iframe>`, `<img>`, `<table>`, `<tbody>`, `<td>`, `<tfoot>`, `<th>`, `<thead>`, `<tr>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/align)
    Align => "align",
    /// Specifies a feature-policy for the iframe.
    ///
    /// Can be applied to the following HTML elements: `<iframe>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#allow)
    Allow => "allow",
    /// Alternative text in case an image can't be displayed.
    ///
    /// Can be applied to the following HTML elements: `<area>`, `<img>`, `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/alt)
    Alt => "alt",
    /// Executes the script asynchronously.
    ///
    /// Can be applied to the following HTML elements: `<script>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script#async)
    Async => "async",
    /// Sets whether input is automatically capitalized when entered by user
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/autocapitalize)
    AutoCapitalize => "autocapitalize",
    /// Indicates whether controls in this form can by default have their values
    /// automatically completed by the browser.
    ///
    /// Can be applied to the following HTML elements: `<form>`, `<input>`, `<select>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete)
    AutoComplete => "autocomplete",
    /// The element should be automatically focused after the page loaded.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<input>`, `<select>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autofocus)
    AutoFocus => "autofocus",
    /// The audio or video should play as soon as possible.
    ///
    /// Can be applied to the following HTML elements: `<audio>`, `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autoplay)
    AutoPlay => "autoplay",
    /// Specifies the URL of an image file.
    /// Note: Although browsers and email clients may still
    /// support this attribute, it is obsolete. Use CSS
    /// background-image instead.
    ///
    /// Can be applied to the following HTML elements: `<body>`, `<table>`, `<td>`, `<th>`.
    Background => "background",
    /// Background color of the element.
    /// Note: This is a legacy attribute. Please use the
    /// CSS background-color property instead.
    ///
    /// Can be applied to the following HTML elements: `<body>`, `<col>`, `<colgroup>`, `<marquee>`, `<table>`, `<tbody>`, `<tfoot>`, `<td>`, `<th>`, `<tr>`.
    BgColor => "bgcolor",
    /// The border width.
    /// Note: This is a legacy attribute. Please use the
    /// CSS border property instead.
    ///
    /// Can be applied to the following HTML elements: `<img>`, `<object>`, `<table>`.
    Border => "border",
    /// Contains the time range of already buffered media.
    ///
    /// Can be applied to the following HTML elements: `<audio>`, `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/buffered)
    Buffered => "buffered",
    /// From the Media Capture specification,
    /// specifies a new file can be captured.
    ///
    /// Can be applied to the following HTML elements: `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/capture)
    Capture => "capture",
    /// Declares the character encoding of the page or script.
    ///
    /// Can be applied to the following HTML elements: `<meta>`, `<script>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/charset)
    Charset => "charset",
    /// Indicates whether the element should be checked on page load.
    ///
    /// Can be applied to the following HTML elements: `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#checked)
    Checked => "checked",
    /// Contains a URI which points to the source of the quote or change.
    ///
    /// Can be applied to the following HTML elements: `<blockquote>`, `<del>`, `<ins>`, `<q>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/cite)
    Cite => "cite",
    /// Often used with CSS to style elements with common properties.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class)
    Class => "class",
    /// This attribute sets the text color using either a named color or a
    /// color specified in the hexadecimal #RRGGBB format.
    /// Note: This is a legacy attribute. Please use the
    /// CSS color property instead.
    ///
    /// Can be applied to the following HTML elements: `<font>`, `<hr>`.
    Color => "color",
    /// Defines the number of columns in a textarea.
    ///
    /// Can be applied to the following HTML elements: `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea#cols)
    Cols => "cols",
    /// The colspan attribute defines the number of columns a cell should span.
    ///
    /// Can be applied to the following HTML elements: `<td>`, `<th>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/colspan)
    Colspan => "colspan",
    /// A value associated with http-equiv or
    /// name depending on the context.
    ///
    /// Can be applied to the following HTML elements: `<meta>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta#content)
    Content => "content",
    /// Indicates whether the element's content is editable.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable)
    ContentEditable => "contenteditable",
    /// Defines the ID of a `<menu>` element which will
    /// serve as the element's context menu.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/contextmenu)
    ContextMenu => "contextmenu",
    /// Indicates whether the browser should show playback controls to the user.
    ///
    /// Can be applied to the following HTML elements: `<audio>`, `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/controls)
    Controls => "controls",
    /// A set of values specifying the coordinates of the hot-spot region.
    ///
    /// Can be applied to the following HTML elements: `<area>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#coords)
    Coords => "coords",
    /// How the element handles cross-origin requests
    ///
    /// Can be applied to the following HTML elements: `<audio>`, `<img>`, `<link>`, `<script>`, `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/crossorigin)
    CrossOrigin => "crossorigin",
    /// Specifies the Content Security Policy that an embedded document must
    /// agree to enforce upon itself.
    ///
    /// Experimental.
    ///
    /// Can be applied to the following HTML elements: `<iframe>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/csp)
    Csp => "csp",
    /// Specifies the URL of the resource.
    ///
    /// Can be applied to the following HTML elements: `<object>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/object#data)
    Data => "data",
    /// Indicates the date and time associated with the element.
    ///
    /// Can be applied to the following HTML elements: `<del>`, `<ins>`, `<time>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/datetime)
    DateTime => "datetime",
    /// Indicates the preferred method to decode the image.
    ///
    /// Can be applied to the following HTML elements: `<img>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#decoding)
    Decoding => "decoding",
    /// Indicates that the track should be enabled unless the user's preferences
    /// indicate something different.
    ///
    /// Can be applied to the following HTML elements: `<track>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track#default)
    Default => "default",
    /// Indicates that the script should be executed after the page has been
    /// parsed.
    ///
    /// Can be applied to the following HTML elements: `<script>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script#defer)
    Defer => "defer",
    /// Defines the text direction. Allowed values are ltr (Left-To-Right) or
    /// rtl (Right-To-Left)
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/dir)
    Dir => "dir",
    /// *Missing MDN description*
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/dirname)
    DirName => "dirname",
    /// Indicates whether the user can interact with the element.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<fieldset>`, `<input>`, `<optgroup>`, `<option>`, `<select>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/disabled)
    Disabled => "disabled",
    /// Indicates that the hyperlink is to be used for downloading a resource.
    ///
    /// Can be applied to the following HTML elements: `<a>`, `<area>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/download)
    Download => "download",
    /// Defines whether the element can be dragged.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/draggable)
    Draggable => "draggable",
    /// Defines the content type of the form data when the
    /// method is POST.
    ///
    /// Can be applied to the following HTML elements: `<form>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#enctype)
    EncType => "enctype",
    /// The enterkeyhint
    /// specifies what action label (or icon) to present for the enter key on
    /// virtual keyboards. The attribute can be used with form controls (such as
    /// the value of textarea elements), or in elements in an
    /// editing host (e.g., using contenteditable attribute).
    ///
    /// Experimental.
    ///
    /// Can be applied to the following HTML elements: `<textarea>`, contenteditable.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/enterkeyhint)
    EnterKeyHint => "enterkeyhint",
    /// Describes elements which belongs to this one.
    ///
    /// Can be applied to the following HTML elements: `<label>`, `<output>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/for)
    For => "for",
    /// Indicates the form that is the owner of the element.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<fieldset>`, `<input>`, `<label>`, `<meter>`, `<object>`, `<output>`, `<progress>`, `<select>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/form)
    Form => "form",
    /// Indicates the action of the element, overriding the action defined in
    /// the `<form>`.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<button>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/formaction)
    FormAction => "formaction",
    /// If the button/input is a submit button (e.g. type="submit"),
    /// this attribute sets the encoding type to use during form submission. If
    /// this attribute is specified, it overrides the
    /// enctype attribute of the button's
    /// form owner.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/formenctype)
    FormEnctype => "formenctype",
    /// If the button/input is a submit button (e.g. type="submit"),
    /// this attribute sets the submission method to use during form submission
    /// (GET, POST, etc.). If this attribute is
    /// specified, it overrides the method attribute of the
    /// button's form owner.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/formmethod)
    FormMethod => "formmethod",
    /// If the button/input is a submit button (e.g. type="submit"),
    /// this boolean attribute specifies that the form is not to be validated
    /// when it is submitted. If this attribute is specified, it overrides the
    /// novalidate attribute of the button's
    /// form owner.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/formnovalidate)
    FormNoValidate => "formnovalidate",
    /// If the button/input is a submit button (e.g. type="submit"),
    /// this attribute specifies the browsing context (for example, tab, window,
    /// or inline frame) in which to display the response that is received after
    /// submitting the form. If this attribute is specified, it overrides the
    /// target attribute of the button's
    /// form owner.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/formtarget)
    FormTarget => "formtarget",
    /// IDs of the `<th>` elements which applies to this
    /// element.
    ///
    /// Can be applied to the following HTML elements: `<td>`, `<th>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/headers)
    Headers => "headers",
    /// Specifies the height of elements listed here. For all other elements,
    /// use the CSS height property.
    /// Note: In some instances, such as
    /// `<div>`, this is a legacy attribute, in
    /// which case the CSS height property should
    /// be used instead.
    ///
    /// Can be applied to the following HTML elements: `<canvas>`, `<embed>`, `<iframe>`, `<img>`, `<input>`, `<object>`, `<video>`.
    Height => "height",
    /// Prevents rendering of given element, while keeping child elements, e.g.
    /// script elements, active.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/hidden)
    Hidden => "hidden",
    /// Indicates the lower bound of the upper range.
    ///
    /// Can be applied to the following HTML elements: `<meter>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter#high)
    High => "high",
    /// The URL of a linked resource.
    ///
    /// Can be applied to the following HTML elements: `<a>`, `<area>`, `<base>`, `<link>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/href)
    Href => "href",
    /// Specifies the language of the linked resource.
    ///
    /// Can be applied to the following HTML elements: `<a>`, `<link>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/hreflang)
    HrefLang => "hreflang",
    /// Defines a pragma directive.
    ///
    /// Can be applied to the following HTML elements: `<meta>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta#http-equiv)
    HttpEquiv => "http-equiv",
    /// Often used with CSS to style a specific element. The value of this
    /// attribute must be unique.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id)
    Id => "id",
    /// Specifies a
    /// Subresource Integrity
    /// value that allows browsers to verify what they fetch.
    ///
    /// Can be applied to the following HTML elements: `<link>`, `<script>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/Security/Subresource_Integrity)
    Integrity => "integrity",
    /// This attribute tells the browser to ignore the actual intrinsic size of
    /// the image and pretend it's the size specified in the attribute.
    ///
    /// Deprecated.
    ///
    /// Can be applied to the following HTML elements: `<img>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#intrinsicsize)
    IntrinsicSize => "intrinsicsize",
    /// Provides a hint as to the type of data that might be entered by the user
    /// while editing the element or its contents. The attribute can be used
    /// with form controls (such as the value of
    /// textarea elements), or in elements in an editing host
    /// (e.g., using contenteditable attribute).
    ///
    /// Can be applied to the following HTML elements: `<textarea>`, contenteditable.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/inputmode)
    InputMode => "inputmode",
    /// Indicates that the image is part of a server-side image map.
    ///
    /// Can be applied to the following HTML elements: `<img>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#ismap)
    IsMap => "ismap",
    /// *Missing MDN description*
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemprop)
    ItemProp => "itemprop",
    /// Specifies the kind of text track.
    ///
    /// Can be applied to the following HTML elements: `<track>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track#kind)
    Kind => "kind",
    /// Specifies a user-readable title of the element.
    ///
    /// Can be applied to the following HTML elements: `<optgroup>`, `<option>`, `<track>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/label)
    Label => "label",
    /// Defines the language used in the element.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/lang)
    Lang => "lang",
    /// Defines the script language used in the element.
    ///
    /// Deprecated.
    ///
    /// Can be applied to the following HTML elements: `<script>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script#language)
    Language => "language",
    /// Indicates if the element should be loaded lazily
    /// (loading="lazy") or loaded immediately
    /// (loading="eager").
    ///
    /// Experimental.
    ///
    /// Can be applied to the following HTML elements: `<img>`, `<iframe>`.
    Loading => "loading",
    /// Identifies a list of pre-defined options to suggest to the user.
    ///
    /// Can be applied to the following HTML elements: `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#list)
    List => "list",
    /// Indicates whether the media should start playing from the start when
    /// it's finished.
    ///
    /// Can be applied to the following HTML elements: `<audio>`, `<marquee>`, `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/loop)
    Loop => "loop",
    /// Indicates the upper bound of the lower range.
    ///
    /// Can be applied to the following HTML elements: `<meter>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter#low)
    Low => "low",
    /// Specifies the URL of the document's cache manifest.
    /// Note: This attribute is obsolete, use
    /// `<link rel="manifest">`
    /// instead.
    ///
    /// Deprecated.
    ///
    /// Can be applied to the following HTML elements: `<html>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/html#manifest)
    Manifest => "manifest",
    /// Indicates the maximum value allowed.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<meter>`, `<progress>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/max)
    Max => "max",
    /// Defines the maximum number of characters allowed in the element.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/maxlength)
    MaxLength => "maxlength",
    /// Defines the minimum number of characters allowed in the element.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/minlength)
    MinLength => "minlength",
    /// Specifies a hint of the media for which the linked resource was
    /// designed.
    ///
    /// Can be applied to the following HTML elements: `<a>`, `<area>`, `<link>`, `<source>`, `<style>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/media)
    Media => "media",
    /// Defines which HTTP method to use when
    /// submitting the form. Can be GET (default) or
    /// POST.
    ///
    /// Can be applied to the following HTML elements: `<form>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#method)
    Method => "method",
    /// Indicates the minimum value allowed.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<meter>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/min)
    Min => "min",
    /// Indicates whether multiple values can be entered in an input of the type
    /// email or file.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<select>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/multiple)
    Multiple => "multiple",
    /// Indicates whether the audio will be initially silenced on page load.
    ///
    /// Can be applied to the following HTML elements: `<audio>`, `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/muted)
    Muted => "muted",
    /// Name of the element. For example used by the server to identify the
    /// fields in form submits.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<form>`, `<fieldset>`, `<iframe>`, `<input>`, `<object>`, `<output>`, `<select>`, `<textarea>`, `<map>`, `<meta>`, `<param>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/name)
    Name => "name",
    /// This attribute indicates that the form shouldn't be validated when
    /// submitted.
    ///
    /// Can be applied to the following HTML elements: `<form>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form#novalidate)
    NoValidate => "novalidate",
    /// Indicates whether the contents are currently visible (in the case of
    /// a `<details>` element) or whether the dialog is active
    /// and can be interacted with (in the case of a
    /// `<dialog>` element).
    ///
    /// Can be applied to the following HTML elements: `<details>`, `<dialog>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/open)
    Open => "open",
    /// Indicates the optimal numeric value.
    ///
    /// Can be applied to the following HTML elements: `<meter>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter#optimum)
    Optimum => "optimum",
    /// Defines a regular expression which the element's value will be validated
    /// against.
    ///
    /// Can be applied to the following HTML elements: `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/pattern)
    Pattern => "pattern",
    /// The ping attribute specifies a space-separated list of URLs
    /// to be notified if a user follows the hyperlink.
    ///
    /// Can be applied to the following HTML elements: `<a>`, `<area>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#ping)
    Ping => "ping",
    /// Provides a hint to the user of what can be entered in the field.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/placeholder)
    PlaceHolder => "placeholder",
    /// A Boolean attribute indicating that the video is to be played "inline"; that is, within the element's playback area. Note that the absence of this attribute does not imply that the video will always be played in fullscreen.
    ///
    /// Can be applied to the following HTML elements: `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video#playsinline)
    PlaysInline => "playsinline",
    /// A URL indicating a poster frame to show until the user plays or seeks.
    ///
    /// Can be applied to the following HTML elements: `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video#poster)
    Poster => "poster",
    /// Indicates whether the whole resource, parts of it or nothing should be
    /// preloaded.
    ///
    /// Can be applied to the following HTML elements: `<audio>`, `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/preload)
    Preload => "preload",
    /// Indicates whether the element can be edited.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/readonly)
    ReadOnly => "readonly",
    /// Specifies which referrer is sent when fetching the resource.
    ///
    /// Can be applied to the following HTML elements: `<a>`, `<area>`, `<iframe>`, `<img>`, `<link>`, `<script>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/referralpolicy)
    ReferrerPolicy => "referrerpolicy",
    /// Specifies the relationship of the target object to the link object.
    ///
    /// Can be applied to the following HTML elements: `<a>`, `<area>`, `<link>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rel)
    Rel => "rel",
    /// Indicates whether this element is required to fill out or not.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<select>`, `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/required)
    Required => "required",
    /// Indicates whether the list should be displayed in a descending order
    /// instead of an ascending order.
    ///
    /// Can be applied to the following HTML elements: `<ol>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol#reversed)
    Reversed => "reversed",
    /// Defines an explicit role for an element for use by assistive technologies.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles)
    Role => "role",
    /// Defines the number of rows in a text area.
    ///
    /// Can be applied to the following HTML elements: `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea#rows)
    Rows => "rows",
    /// Defines the number of rows a table cell should span over.
    ///
    /// Can be applied to the following HTML elements: `<td>`, `<th>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rowspan)
    RowSpan => "rowspan",
    /// Stops a document loaded in an iframe from using certain features (such
    /// as submitting forms or opening new windows).
    ///
    /// Can be applied to the following HTML elements: `<iframe>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#sandbox)
    SandBox => "sandbox",
    /// Defines the cells that the header test (defined in the
    /// th element) relates to.
    ///
    /// Can be applied to the following HTML elements: `<th>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th#scope)
    Scope => "scope",
    /// *Missing MDN description*
    ///
    /// Non-standard, deprecated.
    ///
    /// Can be applied to the following HTML elements: `<style>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style#scoped)
    Scoped => "scoped",
    /// Defines a value which will be selected on page load.
    ///
    /// Can be applied to the following HTML elements: `<option>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option#selected)
    Selected => "selected",
    /// *Missing MDN description*
    ///
    /// Can be applied to the following HTML elements: `<a>`, `<area>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/shape)
    Shape => "shape",
    /// Defines the width of the element (in pixels). If the element's
    /// type attribute is text or
    /// password then it's the number of characters.
    ///
    /// Can be applied to the following HTML elements: `<input>`, `<select>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/size)
    Size => "size",
    /// *Missing MDN description*
    ///
    /// Can be applied to the following HTML elements: `<link>`, `<img>`, `<source>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/sizes)
    Sizes => "sizes",
    /// Assigns a slot in a shadow DOM shadow tree to an element.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/slot)
    Slot => "slot",
    /// *Missing MDN description*
    ///
    /// Can be applied to the following HTML elements: `<col>`, `<colgroup>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/span)
    Span => "span",
    /// Indicates whether spell checking is allowed for the element.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/spellcheck)
    SpellCheck => "spellcheck",
    /// The URL of the embeddable content.
    ///
    /// Can be applied to the following HTML elements: `<audio>`, `<embed>`, `<iframe>`, `<img>`, `<input>`, `<script>`, `<source>`, `<track>`, `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/src)
    Src => "src",
    /// *Missing MDN description*
    ///
    /// Can be applied to the following HTML elements: `<iframe>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#srcdoc)
    SrcDoc => "srcdoc",
    /// *Missing MDN description*
    ///
    /// Can be applied to the following HTML elements: `<track>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track#srclang)
    SrcLang => "srclang",
    /// One or more responsive image candidates.
    ///
    /// Can be applied to the following HTML elements: `<img>`, `<source>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/srcset)
    SrcSet => "srcset",
    /// Defines the first number if other than 1.
    ///
    /// Can be applied to the following HTML elements: `<ol>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol#start)
    Start => "start",
    /// *Missing MDN description*
    ///
    /// Can be applied to the following HTML elements: `<input>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/step)
    Step => "step",
    /// Defines CSS styles which will override styles previously set.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/style)
    Style => "style",
    /// *Missing MDN description*
    ///
    /// Deprecated.
    ///
    /// Can be applied to the following HTML elements: `<table>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table#summary)
    Summary => "summary",
    /// Overrides the browser's default tab order and follows the one specified
    /// instead.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/tabindex)
    TabIndex => "tabindex",
    /// Specifies where to open the linked document (in the case of an
    /// `<a>` element) or where to display the response received
    /// (in the case of a `<form>` element)
    ///
    /// Can be applied to the following HTML elements: `<a>`, `<area>`, `<base>`, `<form>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/target)
    Target => "target",
    /// Text to be displayed in a tooltip when hovering over the element.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/title)
    Title => "title",
    /// Specify whether an element's attribute values and the values of its
    /// Text node
    /// children are to be translated when the page is localized, or whether to
    /// leave them unchanged.
    ///
    /// Global attribute, can be applied to any HTML element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/translate)
    Translate => "translate",
    /// Defines the type of the element.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<input>`, `<embed>`, `<object>`, `<ol>`, `<script>`, `<source>`, `<style>`, `<menu>`, `<link>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/type)
    Type => "type",
    /// *Missing MDN description*
    ///
    /// Can be applied to the following HTML elements: `<img>`, `<input>`, `<object>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/usemap)
    UseMap => "usemap",
    /// Defines a default value which will be displayed in the element on page
    /// load.
    ///
    /// Can be applied to the following HTML elements: `<button>`, `<data>`, `<input>`, `<li>`, `<meter>`, `<option>`, `<progress>`, `<param>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/value)
    DefaultValue => "value",
    /// For the elements listed here, this establishes the element's width.
    /// Note: For all other instances, such as
    /// `<div>`, this is a legacy attribute, in
    /// which case the CSS width property should be
    /// used instead.
    ///
    /// Can be applied to the following HTML elements: `<canvas>`, `<embed>`, `<iframe>`, `<img>`, `<input>`, `<object>`, `<video>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/width)
    Width => "width",
    /// Indicates whether the text should be wrapped.
    ///
    /// Can be applied to the following HTML elements: `<textarea>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea#wrap)
    Wrap => "wrap",
}
