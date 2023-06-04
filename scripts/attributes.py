#!/usr/bin/env python

import pandas as pd

ORIGIN = "https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes"

RENAME_OVERRIDE = {
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
    "intrinsicsize": "IntrinSicsize",
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

tables = pd.read_html(ORIGIN, extract_links="all")
attributes = tables[0]

def make_attribute(row):
    (name, link), (elements, _), (description, _) = row

    name, *warnings = name.replace("  ", " ").split(" ")
    warnings[1:] = [warning.lower() for warning in warnings[1:]]
    warnings = ", ".join(warnings)

    if name == "data-*":
        return

    doc = []
    doc.extend([line for line in description.split("  ")])

    if doc[0] == "":
        doc[0] = "*Missing MDN description*"

    if len(warnings) > 0:
        doc.append("")
        doc.append(f"{warnings}.")

    doc.append("")
    if elements == "Global attribute":
        doc.append("Global attribute, can be applied to any HTML element.")
    else:
        elements = elements.replace("  ", " ")
        doc.append(f"Can be applied to the following HTML elements: {elements}.")

    if link is not None:
        doc.append("")
        doc.append(f"[MDN documentation.](https://developer.mozilla.org{link})")

    doc = [f"    /// {row}\n" for row in doc]
    docstring = "".join(doc)
    docstring = docstring.replace("<", "`<").replace(">", ">`")

    if (override := RENAME_OVERRIDE.get(name)) is not None:
        rust_name = override
    else:
        rust_name = name[0].upper() + name[1:]

    code = f"    {rust_name} => \"{name}\","

    print(docstring + code)

print(f"// Programmatically gathered from {ORIGIN}.")
print("attributes! {")
attributes.apply(make_attribute, "columns")
print("}")
