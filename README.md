# Wasmide frontend framework

An experimental frontend web framework in pure rust.

Wasmide is focused on simplicity, ease of use and performance.

```rust
TODO
```

# Features

Wasmide features:

+ Reactive programming types and concepts.
+ Modular, customizable and reusable components.
+ Rustic API featuring no macro magic.
+ `no_std` support.
+ Integrates nicely with HTML, CSS and JavaScript.
+ Easy bundling and deployment with [trunk](https://trunkrs.dev/).

# Examples

See the [examples](https://github.com/L-Benjamin/wasmide/tree/main/examples) directory for a list of examples built with Wasmide.

<!-- # Roadmap

Wasmide won't get stabilized any time soon. I would like to wait for wasm to get
native access to the dom before that, which might take a (very) long time.

For now, Wasmide is only a proof of concept. Here are the points I'd like to expand on in the near future:
+ Remove, or at least limit, the current extensive use of `unsafe` code. This will need quite some work in the store module. Eventually the goal would be to `#![forbid(unsafe_code)` without sacrificing performance.
+ Make more components constructors. The goal is to cover most html components, at least the ones that are not deprecated.
+ Profiling and benchmarks for both binary size and performance. -->