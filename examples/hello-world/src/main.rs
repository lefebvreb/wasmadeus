use wasmide::prelude::*;

fn main() {
    let counter = Store::new(0);

    app(Style::NONE)
        .with(Component::text(Value("Hello, world!"), Style::NONE))
        .with(Component::text(counter.compose(|x| x.to_string()).unwrap(), Style::NONE))
        .with(Component::button(
            Value("Increment"),
            move || { counter.update(|count| count + 1).ok(); },
            Style::NONE,
        ));
}