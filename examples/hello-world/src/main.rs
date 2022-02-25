use wasmide::prelude::*;

fn main() {
    app(Style("container mx-auto bg-blue-200 grid justify-items-center items-center"))
        .with(Component::text(
            Value("Hello, World!"),
            Style("text-9xl text-center"),
        ));
}
