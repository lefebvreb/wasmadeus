//! Run these with [miri](https://github.com/rust-lang/miri).

use wasmadeus::signal::SignalMut;

#[test]
fn unsubscribe_in_notify() {
    let signal = SignalMut::new("hello");

    signal.for_each_inner(move |_, unsub| {
        unsub.unsubscribe();
    });
}

#[test]
fn unsubscribe_in_second_notify() {
    let signal = SignalMut::new("hello");

    let mut count = 0;
    signal.for_each_inner(move |_, unsub| match count {
        1 => unsub.unsubscribe(),
        _ => count += 1,
    });

    signal.set("goodbye");
}

#[test]
#[should_panic]
fn get_in_mutate() {
    let signal = SignalMut::new("hello");

    signal.mutate(|txt| {
        signal.get();
        *txt = "goodbye";
    });
}

#[test]
fn map() {
    let half = SignalMut::new(21);
    let double = half.map(|i| i * 2);
    assert_eq!(double.get(), 42);
}
