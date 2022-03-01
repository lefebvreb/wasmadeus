use wasmide::prelude::*;

fn main() {
    Component::body(Style("container mx-auto bg-blue-200 grid place-items-center"))
        .with(Component::p(
            Value("Hello, World!"),
            Style("text-9xl"),
        ));
}
