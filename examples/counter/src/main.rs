use wasmide::prelude::*;

fn main() {
    let counter = Store::new(0);

    Component::root(Style("container mx-auto bg-red-200 grid place-items-center"))
        .with(html::text(
            counter.clone(),
            Style("text-9xl"),
        ))
        .with(html::button(
            Value("Increment"),
            move || { counter.update(|count| count + 1).ok(); },
            Style("bg-white rounded border-black border-2 p-5 hover:bg-gray-200"),
        ));
}