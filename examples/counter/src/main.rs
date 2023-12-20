use wasmadeus::prelude::*;

fn main() {
    ConsoleLogger::new().init().unwrap();

    let counter = SignalMut::new(0);
    let text = counter.map(|n| format!("Counter value: {n}"));

    html::div(())
        .with(html::h1(()).text(text))
        .with(html::button(()).text("Increment Counter"))
        .attach_to("#root")
        .unwrap_or_else(|_| log::error!("root element not found"));
}
