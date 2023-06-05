#!/usr/bin/env python

import pandas as pd

ORIGIN = "https://developer.mozilla.org/en-US/docs/Web/HTML/Element"

tables = pd.read_html(ORIGIN, extract_links="all")

def make_element(name, link, description, deprecated):
    doc = []
    doc.extend([line for line in description.split("  ")])

    if deprecated:
        doc.append("")
        doc.append("Deprecated.")

    if link is not None:
        doc.append("")
        doc.append(f"[MDN documentation.](https://developer.mozilla.org{link})")

    doc = [f"    /// {row}\n" for row in doc]
    docstring = "".join(doc)
    docstring = docstring.replace("<", "`<").replace(">", ">`")

    tag_name = name.replace("<", "").replace(">", "")
    code = f"    {tag_name} => \"{tag_name}\","

    print(docstring + code)

def extract_row(row, deprecated=False):
    (name, link), (description, _) = row

    if name.startswith("<h1>"):
        for name in name.split(", "):
            make_element(name, link, description, deprecated)
        return

    make_element(name, link, description, deprecated)

print(f"// Programmatically gathered from {ORIGIN}.")
print("elements! {")
for table in tables[:-1]:
    table.apply(extract_row, "columns")
tables[-1].apply(extract_row, "columns", deprecated=True)
print("}")
