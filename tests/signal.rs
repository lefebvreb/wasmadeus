//! Run these with [miri](https://github.com/rust-lang/miri).

use wasmadeus::signal::{Mutable, Signal, Value};

#[test]
fn value_polymorphism() {
    fn do_something(v: impl Value<i32>) {
        v.for_each(|i| println!("{i}"));
    }

    let mutable = Mutable::new(777);

    do_something(&42);
    do_something(&mutable);
}

#[test]
fn unsubscribe_in_notify() {
    let signal = Mutable::new("hello");

    signal.for_each_inner(move |_, unsub| {
        unsub.unsubscribe();
    });
}

#[test]
fn unsubscribe_in_second_notify() {
    let signal = Mutable::new("hello");

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
    let signal = Mutable::new("hello");

    signal.mutate(|txt| {
        signal.get();
        *txt = "goodbye";
    });
}

// #[test]
// fn map() {
//     let half = Mutable::new(21);
//     let double = half.map(|i| i * 2);
//     assert_eq!(double.get(), 42);
// }
