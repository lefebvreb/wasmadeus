#!/usr/bin/env python

from dataclasses import dataclass
from typing import Optional

import pandas as pd

MDN_ELEMENTS = "https://developer.mozilla.org/en-US/docs/Web/HTML/Element"
MDN_ATTRIBUTES = "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes"

ATTRIBUTE_RENAME_OVERRIDE = {
    "accept-charset": "AcceptCharset",
    "accesskey": "AccessKey",
    "autocapitalize": "AutoCapitalize",
    "autocomplete": "AutoComplete",
    "autofocus": "AutoFocus",
    "autoplay": "AutoPlay",
    "bgcolor": "BgColor",
    "contenteditable": "ContentEditable",
    "contextmenu": "ContextMenu",
    "crossorigin": "CrossOrigin",
    "datetime": "DateTime",
    "dirname": "DirName",
    "enctype": "EncType",
    "enterkeyhint": "EnterKeyHint",
    "formaction": "FormAction",
    "formenctype": "FormEnctype",
    "formmethod": "FormMethod",
    "formnovalidate": "FormNoValidate",
    "formtarget": "FormTarget",
    "hreflang": "HrefLang",
    "http-equiv": "HttpEquiv",
    "intrinsicsize": "IntrinsicSize",
    "inputmode": "InputMode",
    "ismap": "IsMap",
    "itemprop": "ItemProp",
    "maxlength": "MaxLength",
    "minlength": "MinLength",
    "novalidate": "NoValidate",
    "placeholder": "PlaceHolder",
    "playsinline": "PlaysInline",
    "readonly": "ReadOnly",
    "referrerpolicy": "ReferrerPolicy",
    "rowspan": "RowSpan",
    "sandbox": "SandBox",
    "spellcheck": "SpellCheck",
    "srcdoc": "SrcDoc",
    "srclang": "SrcLang",
    "srcset": "SrcSet",
    "tabindex": "TabIndex",
    "usemap": "UseMap",
    "value": "DefaultValue",
}

@dataclass
class Element:
    name: str
    desc: str
    deprecated: bool
    mdn_link: str
    rust_name: str
    rust_link: str
    possible_attributes: list[str]

elements = {}

def make_element(name, link, desc, deprecated):
    global elements
    elements[name] = Element(
        name,
        desc,
        deprecated,
        f"[MDN documentation.](https://developer.mozilla.org{link})" if link else "*Missing MDN documentation*",
        name[1:-1],
        "/", # todo
        [],
    )

def extract_element(row, deprecated):
    (name, link), (desc, _) = row
    for name in name.split(", "):
        make_element(name, link, desc, deprecated)

tables = pd.read_html(MDN_ELEMENTS, extract_links="all")
for table in tables[:-1]:
    table.apply(extract_element, axis="columns", deprecated=False)
tables[-1].apply(extract_element, axis="columns", deprecated=True)

@dataclass
class Attribute:
    name: str
    desc: str
    deprecated: bool
    mdn_link: str
    rust_name: str
    rust_link: str
    possible_elements: list[str]

attributes = {}

print("\n".join(elements))
