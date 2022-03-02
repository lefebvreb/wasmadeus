use wasmide::prelude::*;

fn main() {
    let counter = Store::new(0);

    Component::body(Style("container mx-auto bg-red-200 grid grid-rows-2 grid-cols-1 justify-items-center items-center"))
        .with(Component::p(
            counter.clone(),
            Style("text-9xl text-center"),
        ))
        .with(Component::button(
            Value("Increment"),
            move || { counter.update(|count| count + 1).ok(); },
            Style("bg-white rounded border-black border-2 py-3 px-5 hover:bg-gray-200"),
        ));
}