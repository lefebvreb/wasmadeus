use wasmadeus::prelude::*;

fn main() {
    let counter = SignalMut::new(0);

    html::div(())
        .with(
            html::h1(())
                .text(counter)
        )
        .with(
            html::button(OnClick)
                .text("Increment Counter")
        )
                
}
