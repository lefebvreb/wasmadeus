#!/usr/bin/env python

import pandas as pd

ORIGIN = "https://developer.mozilla.org/en-US/docs/Web/HTML/Element"

tables = pd.read_html(ORIGIN, extract_links="all")

def make_element(row):
    (name, link), (description, _) = row
    print(name, link, description, "\n")

for table in tables:
    table.apply(make_element, "columns")
