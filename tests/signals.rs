//! Run these with [miri](https://github.com/rust-lang/miri).

use wasmide::signal::Signal;

#[test]
pub fn unsubscribe_in_notify() {
    let signal = Signal::new("hello");

    signal.for_each_inner(|_, unsub| {
        unsub.unsubscribe();
    });
}

#[test]
pub fn unsubscribe_in_notify2() {
    let signal = Signal::new("hello");

    let mut count = 0;
    signal.for_each_inner(move |_, unsub| {
        match count {
            2 => unsub.unsubscribe(),
            _ => count += 1,
        }
    });

    signal.set("goodbye");
}

#[test]
pub fn get_in_mutate() {
    let signal = Signal::new("hello");

    signal.mutate(|txt| {
        signal.get();
        *txt = "goodbye";
    });
}
