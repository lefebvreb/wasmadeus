# Scripts and utilities

* `check.sh`: runs successively `cargo fmt`, `cargo clippy`, `cargo miri test` and `cargo rustdoc`, with all features combinations.
* `html-codegen.py`: scrapes MDN for the HTML attributes and elements definitions, then generates some rust code. The standard output can be directly redirected to `src/html.rs`.
