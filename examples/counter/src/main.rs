use wasmadeus::prelude::*;

fn main() {
    ConsoleLogger::new().init().unwrap();
    // let counter = SignalMut::new(0);

    html::div(())
        // .with(html::h1(()).text(counter))
        // .with(html::button(OnClick).text("Increment Counter"));
        .attach_to("#root")
        .unwrap_or_else(|_| log::error!("root element not found"));
}
