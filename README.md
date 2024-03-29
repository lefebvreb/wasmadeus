<div align="center">
    <img alt="wasmadeus" src="logo.svg" height="300"/>
    <h1>Wasmadeus</h1>
    <a href="https://github.com/lefebvreb/wasmadeus"><img alt="github" src="https://img.shields.io/badge/github-lefebvreb/wasmadeus-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20"></a>
    <a href="https://crates.io/crates/wasmadeus"><img alt="crates.io" src="https://img.shields.io/crates/v/wasmadeus.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20"></a>
    <a href="https://docs.rs/wasmadeus"><img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-wasmadeus-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20"></a>
</div>

<br>

An experimental frontend web framework in pure rust.

Wasmadeus is focused on simplicity and robustness.

```rust
// TODO: cool counter example
```

## Features

* Functional reactive programming (FRP) types and concepts.
* Modular, customizable and reusable components.
* Rustic API featuring no macro magic.
* Nice abstractions for fetch and other browser primitives.
* Easy bundling with [trunk](https://trunkrs.dev/).
<!-- + `no_std` support, light code size. (this is blocked on `web_sys` not being `no_std`) -->

## Examples

See the [examples](https://github.com/L-Benjamin/wasmadeus/tree/main/examples) directory for a list of examples built with Wasmadeus.

## FAQ

* **Is it faster than *`<popular JS framework>`* ?**

Probably not. WebAssembly is still a young technology, and did not receive the optimizations that were given to JS over decades. More importantly, WebAssembly still lacks access to the DOM, any UI operation requires an extra layer of JS to complete.

* **Can I use it with *`<favorite NPM package>`* ?**

Sure, but you will have to bring your own glue.

* **Can I contribute/give feedback ?**

Yes, Gladly!

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
