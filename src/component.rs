use web_sys::Element;

use crate::attribute::Attributes;

#[derive(Clone)]
pub struct Component {
    element: Element,
}

impl Component {
    fn new<A: Attributes>(tag: &str, attributes: A) -> Component {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(tag)
            .unwrap();

        attributes.apply_to(&element);

        Component { element }
    }

    pub fn element(&self) -> &Element {
        &self.element
    }
}

macro_rules! elements {
    {
        $(
            $(#[$attr:meta])*
            $rust_name: ident => $html_name: expr,
        )*
    } => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $rust_name<A: Attributes>(attributes: A) -> Component {
                Component::new($html_name, attributes)
            }
        )*
    };
}

// Programmatically gathered from https://developer.mozilla.org/en-US/docs/Web/HTML/Element.
elements! {
    /// Represents the root (top-level element) of an HTML document, so it is also referred to as the root element. All other elements must be descendants of this element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/html)
    html => "html",
    /// Specifies the base URL to use for all relative URLs in a document. There can be only one such element in a document.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base)
    base => "base",
    /// Contains machine-readable information (metadata) about the document, like its title, scripts, and style sheets.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/head)
    head => "head",
    /// Specifies relationships between the current document and an external resource. This element is most commonly used to link to CSS, but is also used to establish site icons (both "favicon" style icons and icons for the home screen and apps on mobile devices) among other things.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link)
    link => "link",
    /// Represents metadata that cannot be represented by other HTML meta-related elements, like `<base>`, `<link>`, `<script>`, `<style>` and `<title>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
    meta => "meta",
    /// Contains style information for a document, or part of a document. It contains CSS, which is applied to the contents of the document containing this element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style)
    style => "style",
    /// Defines the document's title that is shown in a browser's title bar or a page's tab. It only contains text; tags within the element are ignored.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title)
    title => "title",
    /// represents the content of an HTML document. There can be only one such element in a document.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body)
    body => "body",
    /// Indicates that the enclosed HTML provides contact information for a person or people, or for an organization.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/address)
    address => "address",
    /// Represents a self-contained composition in a document, page, application, or site, which is intended to be independently distributable or reusable (e.g., in syndication). Examples include: a forum post, a magazine or newspaper article, or a blog entry, a product card, a user-submitted comment, an interactive widget or gadget, or any other independent item of content.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article)
    article => "article",
    /// Represents a portion of a document whose content is only indirectly related to the document's main content. Asides are frequently presented as sidebars or call-out boxes.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/aside)
    aside => "aside",
    /// Represents a footer for its nearest ancestor sectioning content or sectioning root element. A `<footer>` typically contains information about the author of the section, copyright data or links to related documents.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/footer)
    footer => "footer",
    /// Represents introductory content, typically a group of introductory or navigational aids. It may contain some heading elements but also a logo, a search form, an author name, and other elements.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/header)
    header => "header",
    /// Represent six levels of section headings. `<h1>` is the highest section level and `<h6>` is the lowest.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
    h1 => "h1",
    /// Represent six levels of section headings. `<h1>` is the highest section level and `<h6>` is the lowest.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
    h2 => "h2",
    /// Represent six levels of section headings. `<h1>` is the highest section level and `<h6>` is the lowest.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
    h3 => "h3",
    /// Represent six levels of section headings. `<h1>` is the highest section level and `<h6>` is the lowest.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
    h4 => "h4",
    /// Represent six levels of section headings. `<h1>` is the highest section level and `<h6>` is the lowest.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
    h5 => "h5",
    /// Represent six levels of section headings. `<h1>` is the highest section level and `<h6>` is the lowest.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
    h6 => "h6",
    /// Represents a heading grouped with any secondary content, such as subheadings, an alternative title, or a tagline.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hgroup)
    hgroup => "hgroup",
    /// Represents the dominant content of the body of a document. The main content area consists of content that is directly related to or expands upon the central topic of a document, or the central functionality of an application.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/main)
    main => "main",
    /// Represents a section of a page whose purpose is to provide navigation links, either within the current document or to other documents. Common examples of navigation sections are menus, tables of contents, and indexes.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav)
    nav => "nav",
    /// Represents a generic standalone section of a document, which doesn't have a more specific semantic element to represent it. Sections should always have a heading, with very few exceptions.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)
    section => "section",
    /// Indicates that the enclosed text is an extended quotation. Usually, this is rendered visually by indentation. A URL for the source of the quotation may be given using the cite attribute, while a text representation of the source can be given using the `<cite>` element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote)
    blockquote => "blockquote",
    /// Provides the description, definition, or value for the preceding term (`<dt>`) in a description list (`<dl>`).
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd)
    dd => "dd",
    /// The generic container for flow content. It has no effect on the content or layout until styled in some way using CSS (e.g., styling is directly applied to it, or some kind of layout model like flexbox is applied to its parent element).
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
    div => "div",
    /// Represents a description list. The element encloses a list of groups of terms (specified using the `<dt>` element) and descriptions (provided by `<dd>` elements). Common uses for this element are to implement a glossary or to display metadata (a list of key-value pairs).
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dl)
    dl => "dl",
    /// Specifies a term in a description or definition list, and as such must be used inside a `<dl>` element. It is usually followed by a `<dd>` element; however, multiple `<dt>` elements in a row indicate several terms that are all defined by the immediate next `<dd>` element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt)
    dt => "dt",
    /// Represents a caption or legend describing the rest of the contents of its parent `<figure>` element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption)
    figcaption => "figcaption",
    /// Represents self-contained content, potentially with an optional caption, which is specified using the `<figcaption>` element. The figure, its caption, and its contents are referenced as a single unit.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figure)
    figure => "figure",
    /// Represents a thematic break between paragraph-level elements: for example, a change of scene in a story, or a shift of topic within a section.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hr)
    hr => "hr",
    /// Represents an item in a list. It must be contained in a parent element: an ordered list (`<ol>`), an unordered list (`<ul>`), or a menu (`<menu>`). In menus and unordered lists, list items are usually displayed using bullet points. In ordered lists, they are usually displayed with an ascending counter on the left, such as a number or letter.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/li)
    li => "li",
    /// A semantic alternative to `<ul>`, but treated by browsers (and exposed through the accessibility tree) as no different than `<ul>`. It represents an unordered list of items (which are represented by `<li>` elements).
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menu)
    menu => "menu",
    /// Represents an ordered list of items — typically rendered as a numbered list.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol)
    ol => "ol",
    /// Represents a paragraph. Paragraphs are usually represented in visual media as blocks of text separated from adjacent blocks by blank lines and/or first-line indentation, but HTML paragraphs can be any structural grouping of related content, such as images or form fields.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)
    p => "p",
    /// Represents preformatted text which is to be presented exactly as written in the HTML file. The text is typically rendered using a non-proportional, or monospaced, font. Whitespace inside this element is displayed as written.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre)
    pre => "pre",
    /// Represents an unordered list of items, typically rendered as a bulleted list.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ul)
    ul => "ul",
    /// Together with its href attribute, creates a hyperlink to web pages, files, email addresses, locations in the same page, or anything else a URL can address.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
    a => "a",
    /// Represents an abbreviation or acronym.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/abbr)
    abbr => "abbr",
    /// Used to draw the reader's attention to the element's contents, which are not otherwise granted special importance. This was formerly known as the Boldface element, and most browsers still draw the text in boldface. However, you should not use `<b>` for styling text or granting importance. If you wish to create boldface text, you should use the CSS font-weight property. If you wish to indicate an element is of special importance, you should use the strong element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/b)
    b => "b",
    /// Tells the browser's bidirectional algorithm to treat the text it contains in isolation from its surrounding text. It's particularly useful when a website dynamically inserts some text and doesn't know the directionality of the text being inserted.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdi)
    bdi => "bdi",
    /// Overrides the current directionality of text, so that the text within is rendered in a different direction.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdo)
    bdo => "bdo",
    /// Produces a line break in text (carriage-return). It is useful for writing a poem or an address, where the division of lines is significant.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br)
    br => "br",
    /// Used to mark up the title of a cited creative work. The reference may be in an abbreviated form according to context-appropriate conventions related to citation metadata.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/cite)
    cite => "cite",
    /// Displays its contents styled in a fashion intended to indicate that the text is a short fragment of computer code. By default, the content text is displayed using the user agent default monospace font.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code)
    code => "code",
    /// Links a given piece of content with a machine-readable translation. If the content is time- or date-related, the time element must be used.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data)
    data => "data",
    /// Used to indicate the term being defined within the context of a definition phrase or sentence. The ancestor `<p>` element, the `<dt>`/`<dd>` pairing, or the nearest section ancestor of the `<dfn>` element, is considered to be the definition of the term.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dfn)
    dfn => "dfn",
    /// Marks text that has stress emphasis. The `<em>` element can be nested, with each level of nesting indicating a greater degree of emphasis.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/em)
    em => "em",
    /// Represents a range of text that is set off from the normal text for some reason, such as idiomatic text, technical terms, taxonomical designations, among others. Historically, these have been presented using italicized type, which is the original source of the `<i>` naming of this element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/i)
    i => "i",
    /// Represents a span of inline text denoting textual user input from a keyboard, voice input, or any other text entry device. By convention, the user agent defaults to rendering the contents of a `<kbd>` element using its default monospace font, although this is not mandated by the HTML standard.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/kbd)
    kbd => "kbd",
    /// Represents text which is marked or highlighted for reference or notation purposes due to the marked passage's relevance in the enclosing context.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/mark)
    mark => "mark",
    /// Indicates that the enclosed text is a short inline quotation. Most modern browsers implement this by surrounding the text in quotation marks. This element is intended for short quotations that don't require paragraph breaks; for long quotations use the `<blockquote>` element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/q)
    q => "q",
    /// Used to provide fall-back parentheses for browsers that do not support display of ruby annotations using the `<ruby>` element. One `<rp>` element should enclose each of the opening and closing parentheses that wrap the `<rt>` element that contains the annotation's text.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rp)
    rp => "rp",
    /// Specifies the ruby text component of a ruby annotation, which is used to provide pronunciation, translation, or transliteration information for East Asian typography. The `<rt>` element must always be contained within a `<ruby>` element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rt)
    rt => "rt",
    /// Represents small annotations that are rendered above, below, or next to base text, usually used for showing the pronunciation of East Asian characters. It can also be used for annotating other kinds of text, but this usage is less common.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby)
    ruby => "ruby",
    /// Renders text with a strikethrough, or a line through it. Use the `<s>` element to represent things that are no longer relevant or no longer accurate. However, `<s>` is not appropriate when indicating document edits; for that, use the del and ins elements, as appropriate.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s)
    s => "s",
    /// Used to enclose inline text which represents sample (or quoted) output from a computer program. Its contents are typically rendered using the browser's default monospaced font (such as Courier or Lucida Console).
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/samp)
    samp => "samp",
    /// Represents side-comments and small print, like copyright and legal text, independent of its styled presentation. By default, it renders text within it one font-size smaller, such as from small to x-small.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/small)
    small => "small",
    /// A generic inline container for phrasing content, which does not inherently represent anything. It can be used to group elements for styling purposes (using the class or id attributes), or because they share attribute values, such as lang. It should be used only when no other semantic element is appropriate. `<span>` is very much like a div element, but div is a block-level element whereas a `<span>` is an inline-level element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span)
    span => "span",
    /// Indicates that its contents have strong importance, seriousness, or urgency. Browsers typically render the contents in bold type.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strong)
    strong => "strong",
    /// Specifies inline text which should be displayed as subscript for solely typographical reasons. Subscripts are typically rendered with a lowered baseline using smaller text.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sub)
    sub => "sub",
    /// Specifies inline text which is to be displayed as superscript for solely typographical reasons. Superscripts are usually rendered with a raised baseline using smaller text.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sup)
    sup => "sup",
    /// Represents a specific period in time. It may include the datetime attribute to translate dates into machine-readable format, allowing for better search engine results or custom features such as reminders.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time)
    time => "time",
    /// Represents a span of inline text which should be rendered in a way that indicates that it has a non-textual annotation. This is rendered by default as a simple solid underline, but may be altered using CSS.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/u)
    u => "u",
    /// Represents the name of a variable in a mathematical expression or a programming context. It's typically presented using an italicized version of the current typeface, although that behavior is browser-dependent.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/var)
    var => "var",
    /// Represents a word break opportunity—a position within text where the browser may optionally break a line, though its line-breaking rules would not otherwise create a break at that location.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/wbr)
    wbr => "wbr",
    /// Defines an area inside an image map that has predefined clickable areas. An image map allows geometric areas on an image to be associated with hyperlink.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
    area => "area",
    /// Used to embed sound content in documents. It may contain one or more audio sources, represented using the src attribute or the source element: the browser will choose the most suitable one. It can also be the destination for streamed media, using a MediaStream.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio)
    audio => "audio",
    /// Embeds an image into the document.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
    img => "img",
    /// Used with `<area>` elements to define an image map (a clickable link area).
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map)
    map => "map",
    /// Used as a child of the media elements, audio and video. It lets you specify timed text tracks (or time-based data), for example to automatically handle subtitles. The tracks are formatted in WebVTT format (.vtt files)—Web Video Text Tracks.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track)
    track => "track",
    /// Embeds a media player which supports video playback into the document. You can use `<video>` for audio content as well, but the audio element may provide a more appropriate user experience.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video)
    video => "video",
    /// Embeds external content at the specified point in the document. This content is provided by an external application or other source of interactive content such as a browser plug-in.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/embed)
    embed => "embed",
    /// Represents a nested browsing context, embedding another HTML page into the current one.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe)
    iframe => "iframe",
    /// Represents an external resource, which can be treated as an image, a nested browsing context, or a resource to be handled by a plugin.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/object)
    object => "object",
    /// Contains zero or more `<source>` elements and one `<img>` element to offer alternative versions of an image for different display/device scenarios.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture)
    picture => "picture",
    /// Enables the embedding of another HTML page into the current one for the purposes of allowing smoother navigation into new pages.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/portal)
    portal => "portal",
    /// Specifies multiple media resources for the picture, the audio element, or the video element. It is a void element, meaning that it has no content and does not have a closing tag. It is commonly used to offer the same media content in multiple file formats in order to provide compatibility with a broad range of browsers given their differing support for image file formats and media file formats.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source)
    source => "source",
    /// Container defining a new coordinate system and viewport. It is used as the outermost element of SVG documents, but it can also be used to embed an SVG fragment inside an SVG or HTML document.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg)
    svg => "svg",
    /// The top-level element in MathML. Every valid MathML instance must be wrapped in it. In addition you must not nest a second `<math>` element in another, but you can have an arbitrary number of other child elements in it.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/math)
    math => "math",
    /// Container element to use with either the canvas scripting API or the WebGL API to draw graphics and animations.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas)
    canvas => "canvas",
    /// Defines a section of HTML to be inserted if a script type on the page is unsupported or if scripting is currently turned off in the browser.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noscript)
    noscript => "noscript",
    /// Used to embed executable code or data; this is typically used to embed or refer to JavaScript code. The `<script>` element can also be used with other languages, such as WebGL's GLSL shader programming language and JSON.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script)
    script => "script",
    /// Represents a range of text that has been deleted from a document. This can be used when rendering "track changes" or source code diff information, for example. The `<ins>` element can be used for the opposite purpose: to indicate text that has been added to the document.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del)
    del => "del",
    /// Represents a range of text that has been added to a document. You can use the `<del>` element to similarly represent a range of text that has been deleted from the document.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins)
    ins => "ins",
    /// Specifies the caption (or title) of a table.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/caption)
    caption => "caption",
    /// Defines a column within a table and is used for defining common semantics on all common cells. It is generally found within a `<colgroup>` element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col)
    col => "col",
    /// Defines a group of columns within a table.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup)
    colgroup => "colgroup",
    /// Represents tabular data — that is, information presented in a two-dimensional table comprised of rows and columns of cells containing data.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table)
    table => "table",
    /// Encapsulates a set of table rows (`<tr>` elements), indicating that they comprise the body of the table (`<table>`).
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tbody)
    tbody => "tbody",
    /// Defines a cell of a table that contains data. It participates in the table model.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td)
    td => "td",
    /// Defines a set of rows summarizing the columns of the table.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tfoot)
    tfoot => "tfoot",
    /// Defines a cell as header of a group of table cells. The exact nature of this group is defined by the scope and headers attributes.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th)
    th => "th",
    /// Defines a set of rows defining the head of the columns of the table.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/thead)
    thead => "thead",
    /// Defines a row of cells in a table. The row's cells can then be established using a mix of `<td>` (data cell) and `<th>` (header cell) elements.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tr)
    tr => "tr",
    /// An interactive element activated by a user with a mouse, keyboard, finger, voice command, or other assistive technology. Once activated, it then performs an action, such as submitting a form or opening a dialog.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
    button => "button",
    /// Contains a set of `<option>` elements that represent the permissible or recommended options available to choose from within other controls.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist)
    datalist => "datalist",
    /// Used to group several controls as well as labels (`<label>`) within a web form.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
    fieldset => "fieldset",
    /// Represents a document section containing interactive controls for submitting information.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form)
    form => "form",
    /// Used to create interactive controls for web-based forms in order to accept data from the user; a wide variety of types of input data and control widgets are available, depending on the device and user agent. The `<input>` element is one of the most powerful and complex in all of HTML due to the sheer number of combinations of input types and attributes.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
    input => "input",
    /// Represents a caption for an item in a user interface.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label)
    label => "label",
    /// Represents a caption for the content of its parent `<fieldset>`.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/legend)
    legend => "legend",
    /// Represents either a scalar value within a known range or a fractional value.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter)
    meter => "meter",
    /// Creates a grouping of options within a `<select>` element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
    optgroup => "optgroup",
    /// Used to define an item contained in a select, an `<optgroup>`, or a `<datalist>` element. As such, `<option>` can represent menu items in popups and other lists of items in an HTML document.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option)
    option => "option",
    /// Container element into which a site or app can inject the results of a calculation or the outcome of a user action.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output)
    output => "output",
    /// Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress)
    progress => "progress",
    /// Represents a control that provides a menu of options.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
    select => "select",
    /// Represents a multi-line plain-text editing control, useful when you want to allow users to enter a sizeable amount of free-form text, for example a comment on a review or feedback form.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    textarea => "textarea",
    /// Creates a disclosure widget in which information is visible only when the widget is toggled into an "open" state. A summary or label must be provided using the `<summary>` element.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details)
    details => "details",
    /// Represents a dialog box or other interactive component, such as a dismissible alert, inspector, or subwindow.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog)
    dialog => "dialog",
    /// Specifies a summary, caption, or legend for a details element's disclosure box. Clicking the `<summary>` element toggles the state of the parent `<details>` element open and closed.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/summary)
    summary => "summary",
    /// Part of the Web Components technology suite, this element is a placeholder inside a web component that you can fill with your own markup, which lets you create separate DOM trees and present them together.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot)
    slot => "slot",
    /// A mechanism for holding HTML that is not to be rendered immediately when a page is loaded but may be instantiated subsequently during runtime using JavaScript.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template)
    template => "template",
    /// Allows authors to clearly indicate a sequence of characters that compose an acronym or abbreviation for a word.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/acronym)
    acronym => "acronym",
    /// Renders the enclosed text at a font size one level larger than the surrounding text (medium becomes large, for example). The size is capped at the browser's maximum permitted font size.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/big)
    big => "big",
    /// Displays its block-level or inline contents centered horizontally within its containing element.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/center)
    center => "center",
    /// An obsolete part of the Web Components suite of technologies—was used inside of Shadow DOM as an insertion point, and wasn't meant to be used in ordinary HTML. It has now been replaced by the `<slot>` element, which creates a point in the DOM at which a shadow DOM can be inserted. Consider using `<slot>` instead.
    ///
    /// Deprecated.
    content => "content",
    /// Container for a directory of files and/or folders, potentially with styles and icons applied by the user agent. Do not use this obsolete element; instead, you should use the `<ul>` element for lists, including lists of files.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dir)
    dir => "dir",
    /// Defines the font size, color and face for its content.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/font)
    font => "font",
    /// Defines a particular area in which another HTML document can be displayed. A frame should be used within a `<frameset>`.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/frame)
    frame => "frame",
    /// Used to contain `<frame>` elements.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/frameset)
    frameset => "frameset",
    /// An ancient and poorly supported precursor to the `<img>` element. It should not be used.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/image)
    image => "image",
    /// Used to insert a scrolling area of text. You can control what happens when the text reaches the edges of its content area using its attributes.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/marquee)
    marquee => "marquee",
    /// Represents a command that a user is able to invoke through a popup menu. This includes context menus, as well as menus that might be attached to a menu button.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menuitem)
    menuitem => "menuitem",
    /// Prevents the text it contains from automatically wrapping across multiple lines, potentially resulting in the user having to scroll horizontally to see the entire width of the text.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nobr)
    nobr => "nobr",
    /// An obsolete, non-standard way to provide alternative, or "fallback", content for browsers that do not support the embed element or do not support the type of embedded content an author wishes to use. This element was deprecated in HTML 4.01 and above in favor of placing fallback content between the opening and closing tags of an `<object>` element.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noembed)
    noembed => "noembed",
    /// Provides content to be presented in browsers that don't support (or have disabled support for) the `<frame>` element. Although most commonly-used browsers support frames, there are exceptions, including certain special-use browsers including some mobile browsers, as well as text-mode browsers.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noframes)
    noframes => "noframes",
    /// Defines parameters for an `<object>` element.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/param)
    param => "param",
    /// Renders everything following the start tag as raw text, ignoring any following HTML. There is no closing tag, since everything after it is considered raw text.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/plaintext)
    plaintext => "plaintext",
    /// Used to delimit the base text component of a ruby annotation, i.e. the text that is being annotated. One `<rb>` element should wrap each separate atomic segment of the base text.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rb)
    rb => "rb",
    /// Embraces semantic annotations of characters presented in a ruby of `<rb>` elements used inside of `<ruby>` element. `<rb>` elements can have both pronunciation (`<rt>`) and semantic (`<rtc>`) annotations.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rtc)
    rtc => "rtc",
    /// An obsolete part of the Web Components technology suite that was intended to be used as a shadow DOM insertion point. You might have used it if you have created multiple shadow roots under a shadow host. Consider using `<slot>` instead.
    ///
    /// Deprecated.
    shadow => "shadow",
    /// Places a strikethrough (horizontal line) over text.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strike)
    strike => "strike",
    /// Creates inline text which is presented using the user agent default monospace font face. This element was created for the purpose of rendering text as it would be displayed on a fixed-width display such as a teletype, text-only screen, or line printer.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tt)
    tt => "tt",
    /// Renders text between the start and end tags without interpreting the HTML in between and using a monospaced font. The HTML2 specification recommended that it should be rendered wide enough to allow 80 characters per line.
    ///
    /// Deprecated.
    ///
    /// [MDN documentation.](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/xmp)
    xmp => "xmp",
}
