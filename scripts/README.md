# Scripts and Utilities

* `check.sh`: run successively `cargo fmt`, `cargo clippy` and `cargo miri test`, with all features combinations.
* `codegen.py`: scrapes MDN for the HTML attributes and elements definitions, then generates some rust code, that can be redirected to `src/html.rs`.
