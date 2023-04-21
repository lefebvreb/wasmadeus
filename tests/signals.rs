//! Run those tests with [miri](https://github.com/rust-lang/miri).

use wasmide::signal::{Signal, Unsubscriber};

#[test]
pub fn unsubscribe_in_notify() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let unsub: Option<Unsubscriber<_>> = None;
    let rc = Rc::new(RefCell::new(unsub));
    let signal = Signal::new("hello");

    let rc_clone = rc.clone();
    let unsub = signal.subscribe(move |_| {
        if let Some(mut unsub) = rc_clone.borrow_mut().take() {
            unsub.unsubscribe();
        }
    });

    *rc.borrow_mut() = Some(unsub);
    signal.set("goodbye");
}