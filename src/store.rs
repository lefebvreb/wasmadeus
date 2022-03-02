//! Building blocks for reactivity. 
//! 
//! This module provides a trait, [`Subscribable`], which is used to
//! work with stores, and three implementations of it: [`Store`], [`Value`] and [`Derived`].
//! The first one is mutable, the second one is not and the third one is obtained by
//! through composition of store.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::rc::Rc;
use alloc::vec::Vec;

use crate::error::{FrontendError, Result};

/// A trait for the ability of a type to be subscribed/unsubscribed to.
/// 
/// Upon subscription with [`Self::subscribe`], a value of type [`StoreUnsubscriber`] is returned
/// which allows unsubscription through the [`StoreUnsubscriber::unsubscribe`] method.
pub trait Subscribable<T> {
    /// Subscribe to the value of type `T`. The given closure will be called immediately 
    /// with the current value as well as every time it changes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new(1);
    /// a.subscribe(|x| println!("{}", x)); // Prints 1
    /// a.update(|x| x + 1).ok(); // Prints 2
    /// ```
    #[must_use]
    fn subscribe(&self, notify: impl FnMut(&T) + 'static) -> StoreUnsubscriber;
}

// The internal representation of a store, designed to be held by a Rc.
struct InternalStore<T> {
    // The current data.
    data: T,
    // A map of the subscribers' notify functions indexed by their IDs.
    subscribers: Vec<(u32, Box<dyn FnMut(&T)>)>,
    // A counter that is incremented with each new subscription.
    unique_count: u32,
    // A flag that signal whether the store is currently being updated.
    updating: bool,
    // A vector holding all subscriptions/unscubscribtions operations delayed because of an update.
    delayed: VecDeque<StoreOperation<T>>,
}

// An operation on a Store.
enum StoreOperation<T> {
    Subscribe {
        notify: Box<dyn FnMut(&T)>,
        idx: u32,
    },
    SubscribeNoNotify {
        notify: Box<dyn FnMut(&T)>,
        idx: u32,
    },
    Unsubscribe(u32),
}

impl<T> InternalStore<T> {
    // Constructs a new InternalStore from a given value.
    #[inline]
    fn new(data: T) -> Self {
        Self {
            data,
            subscribers: Vec::new(),
            unique_count: 0,
            updating: false,
            delayed: VecDeque::new(),
        }
    }

    // Mutates the state of the store by applying the specified closure.
    #[inline]
    fn mutate(&mut self, mutater: impl FnOnce(&mut T)) -> Result<()> {        
        if self.updating {
            return Err(FrontendError::StoreUpdating);
        }

        self.updating = true;

        mutater(&mut self.data);

        for (_, subscriber) in &mut self.subscribers {
            subscriber(&self.data);
        }

        while let Some(op) = self.delayed.pop_front() {
            self.do_operation(op);
        }

        self.updating = false;

        Ok(())
    }

    // Pushes a new operation to the queue.
    #[inline]
    fn push_op(&mut self, op: StoreOperation<T>) {
        if self.updating {
            self.delayed.push_back(op);
        } else {
            self.do_operation(op);
        }
    }

    // Carries the operation on the store.
    #[inline]
    fn do_operation(&mut self, op: StoreOperation<T>) {
        match op {
            StoreOperation::Subscribe {idx, notify} => {
                self.subscribers.push((idx, notify));
                self.subscribers.last_mut().unwrap().1(&self.data);
            }
            StoreOperation::SubscribeNoNotify {idx, notify} => {
                self.subscribers.push((idx, notify));
            }
            StoreOperation::Unsubscribe(idx) => {
                let i = self.subscribers.partition_point(|&(i, _)| i < idx);
                if self.subscribers[i].0 == idx {
                    let _ = self.subscribers.remove(i);
                }
            }
        }
    }

    // Subscribes to the store, returns the unique id of the subscriber.
    #[inline]
    fn subscribe(&mut self, notify: impl FnMut(&T) + 'static) -> u32 {
        let idx = self.unique_count;
        self.unique_count += 1;

        let op = StoreOperation::Subscribe {
            notify: Box::new(notify),
            idx,
        };
        self.push_op(op);

        idx
    }

    // Unsubscribes the subscribee with the given ID from the store.
    #[inline]
    fn unsubscribe(&mut self, idx: u32) {
        let op = StoreOperation::Unsubscribe(idx);
        self.push_op(op);
    }

    // Composes the store with the given closure.
    // Returns a derived store along with the unique id of the subscription to the original store.
    #[inline]
    fn compose<U: 'static>(&mut self, composer: impl Fn(&T) -> U + 'static) -> (Derived<U>, u32) {
        // SAFETY: data is initialized upon subscription.
        let derived = Derived::new(composer(&self.data));
        let cloned = derived.clone();

        let idx = self.unique_count;
        self.unique_count += 1;

        let op = StoreOperation::SubscribeNoNotify {
            // SAFETY: a mutate operation is always safe. Because updates are tied
            // to the self store, we are guaranteed that updates are legals,
            // so we can unwrap the result.
            notify: Box::new(move |new| unsafe {
                cloned.internal().mutate(|data| *data = composer(new)).unwrap();
            }),
            idx,
        };
        self.push_op(op);

        (derived, idx)
    }
}

/// A cloneable reference to a mutable value, that can be subscribed/unsubscribed to.
/// 
/// Subscribers will be notified whenever the value of the store is updated. The ability 
/// to subscribe/unsbscribe to a store is provided by the [`Subscribable`] trait.
/// `Store`s are the fundamental building brick of reactivity in wasmide.
/// 
/// # Examples
/// 
/// ```
/// # use wasmide::prelude::*;
/// let a = Store::new(1);
/// a.subscribe(|x| println!("{}", x)); // Prints 1
/// a.update(|x| x + 1).ok(); // Prints 2
/// ```
pub struct Store<T: 'static>(Rc<UnsafeCell<InternalStore<T>>>);

impl<T: 'static> Store<T> {
    // Returns a mutable reference to the internal store.
    // This method is unsafe because it allows mutation of the internal store while not
    // checking borrowing rules.
    #[inline]
    unsafe fn internal(&self) -> &mut InternalStore<T> {
        &mut *self.0.get()
    }

    // Constructs an unsubscriber to self from the given index.
    #[inline]
    fn unsubscriber(&self, idx: u32) -> StoreUnsubscriber {
        let weak = Rc::downgrade(&self.0);
        StoreUnsubscriber::new(move || {
            if let Some(rc) = weak.upgrade() {
                // SAFETY: unsubscription is always safe.
                unsafe { Self(rc).internal().unsubscribe(idx); }
            }
        })
    }

    /// Constructs a new Store from a given value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new(42);
    /// ```
    #[inline]
    pub fn new(data: T) -> Self {
        Self(Rc::new(InternalStore::new(data).into()))
    }

    /// Mutates the value contained in the store by applying the given closure. Subscribers to
    /// the store will be notified of the change.
    /// 
    /// If the store is already being updated, an error of type [`FrontendError::StoreUpdating`]
    /// is returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new("hello".to_string());
    /// a.subscribe(|x| println!("{}", x)); // Prints "Hello"
    /// a.mutate(|x| *x = x.to_uppercase()).ok(); // Prints "HELLO"
    /// ```
    #[inline]
    pub fn mutate(&self, mutater: impl FnOnce(&mut T)) -> Result<()> {
        // SAFETY: Will toggle the updating flag that gates away other call to update(),
        // so that only one update can be carried out at a time: mutates the data,
        // and then call all the subscribers. Has an exclusive access to subscribers
        // and data by the updating flag.
        // The queue and updating and never borrowed.
        unsafe { self.internal().mutate(mutater) }
    }

    /// Updates the value contained in the store by applying the given closure. Subscribers to
    /// the store will be notified of the change.
    /// 
    /// If the store is already being updated, an error of type [`FrontendError::StoreUpdating`]
    /// is returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new("hello".to_string());
    /// a.subscribe(|x| println!("{}", x)); // Prints "Hello"
    /// a.update(|x| x.to_uppercase()).ok(); // Prints "HELLO"
    /// ```
    #[inline]
    pub fn update(&self, updater: impl FnOnce(&T) -> T) -> Result<()> {
        self.mutate(|data| *data = updater(data))
    }

    /// Replaces the value contained in the store by simply replacing it with the new one
    /// provided. This counts as an update, and all subscribers will be notified.
    /// 
    /// If the store is already being updated, an error of kind [`FrontendError::StoreUpdating`]
    /// is returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new(false);
    /// a.subscribe(|x| if *x { println!("Here") }); // Prints nothing
    /// a.set(true); // Prints "Here"
    #[inline]
    pub fn set(&self, value: T) -> Result<()> {
        self.mutate(move |data| *data = value)
    }

    /// Constructs a new store by composing `self` updates with the given closure. Upon creation and
    /// whenever the value of `self` changes, the new store's value will be updated by applying the 
    /// given closure on the value contained in `self`. 
    /// 
    /// The store returned by this function is of type [`Derived`].
    /// 
    /// If `self` is already being updated, an error of type [`FrontendError::StoreUpdating`] is
    /// returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new(1);
    /// let b = a.compose(|x| *x * 10); // b will always be 10 times a
    /// b.subscribe(|x| println!("{}", x)); // Prints 10
    /// a.update(|x| x + 1).ok(); // Prints 20
    /// ```
    #[inline]
    pub fn compose<U: 'static>(&self, composer: impl Fn(&T) -> U + 'static) -> Derived<U> {
        unsafe {
            let internal = self.internal();
            let (derived, idx) = internal.compose(composer);
            derived.set_unsubscriber(self.unsubscriber(idx));
            derived
        }
    }
}

impl<T> Clone for Store<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<T> for Store<T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

/// A type that can then be used to unsubscribe from the store.
/// 
/// It is returned by [`Store::subscribe`], and must be passed 
/// to [`StoreUnsubscriber::unsubscribe`] to unsubscribe from the store.
/// 
/// # Examples
/// 
/// ```
/// # use wasmide::prelude::*;
/// let a = Store::new("hello");
/// let unsub = a.subscribe(|x| println!("{}", x)); // Prints "hello"
/// unsub.unsubscribe();
/// a.set("goodbye"); // Prints nothing
/// ```
#[must_use]
pub struct StoreUnsubscriber(Option<Box<dyn FnOnce()>>);

impl StoreUnsubscriber {
    /// Constructs a new store unsubscriber.
    /// 
    /// The provided closure will be called when the unsubscription is consumed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new(42);
    /// let unsub = a.subscribe(|x| println!("{}", x)); // Prints 42
    /// unsub.unsubscribe();
    /// a.set(5); // Prints nothing
    /// ```
    #[inline]
    pub fn new(unsubscribe: impl FnOnce() + 'static) -> Self {
        StoreUnsubscriber(Some(Box::new(unsubscribe)))
    }

    /// Consumes `self` and performs the unsubscription.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new(1);
    /// let unsub = a.subscribe(|x| println!("{}", x)); // Prints 1
    /// unsub.unsubscribe();
    /// a.update(|x| *x + 1); // Prints nothing
    /// ```
    #[inline]
    pub fn unsubscribe(mut self) {
        self.0.take().unwrap()();
    }
}

impl<T: 'static> Subscribable<T> for Store<T> {
    #[inline]
    #[must_use]
    fn subscribe(&self, notify: impl FnMut(&T) + 'static) -> StoreUnsubscriber {
        // SAFETY: increments the unique_count, and then pushes the operation to the queue.
        // The unique_count and queue are never borrowed.  
        let idx = unsafe { self.internal().subscribe(notify) };
        self.unsubscriber(idx)
    }
}

struct InternalDerived<T> {
    store: InternalStore<T>,
    unsub: Option<StoreUnsubscriber>,
}

impl<T> InternalDerived<T> {
    // Constructs a new derived store.
    #[inline]
    fn new(data: T) -> Self {
        Self {
            store: InternalStore::new(data),
            unsub: None,
        }
    }
}

impl<T> Drop for InternalDerived<T> {
    // When a derived store is dropped, it unsubscribes from the store it is tied to.
    #[inline]
    fn drop(&mut self) {
        self.unsub.take().unwrap().unsubscribe();
    }
}

/// A type used to maintain invariants in the application.
/// 
/// It is result of the calling [`Store::compose`]. This store can't be updated
/// manually and it's value changes everytime that of the store it is tied to does.
///  
/// # Examples
/// 
/// ```
/// # use wasmide::prelude::*;
/// let a = Store::new(1);
/// let b = a.compose(|x| *x * 10); // b will always be 10 times a
/// b.subscribe(|x| println!("{}", x)); // Prints 10
/// a.update(|x| x + 1).ok(); // Prints 20
/// ```
pub struct Derived<T: 'static>(Rc<UnsafeCell<InternalDerived<T>>>);

impl<T> Derived<T> {
    // Constructs a new derived store.
    #[inline]
    fn new(data: T) -> Self {
        Derived(Rc::new(InternalDerived::new(data).into()))
    }

    // Returns the internal store.
    #[inline]
    unsafe fn internal(&self) -> &mut InternalStore<T> {
        &mut (*self.0.get()).store
    }

    // Sets the unsubscriber field. Unsafe because it bypasses borrow
    // checking.
    #[inline]
    unsafe fn set_unsubscriber(&self, unsub: StoreUnsubscriber) {
        (*self.0.get()).unsub = Some(unsub);
    }

    // Constructs an unsubscriber to self from the given index.
    #[inline]
    fn unsubscriber(&self, idx: u32) -> StoreUnsubscriber {
        let weak = Rc::downgrade(&self.0);
        StoreUnsubscriber::new(move || {
            if let Some(rc) = weak.upgrade() {
                unsafe { Self(rc).internal().unsubscribe(idx); }
            }
        })
    }

    /// Constructs a new store by composing `self` updates with the given closure. Upon creation and
    /// whenever the value of `self` changes, the new store's value will be updated by applying the 
    /// given closure on the value contained in `self`. 
    /// 
    /// The store returned by this function is of type [`Derived`].
    /// 
    /// If `self` is already being updated, an error of type [`FrontendError::StoreUpdating`] is
    /// returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new(1);
    /// let b = a.compose(|x| *x * 10); // b will always be 10 times a
    /// let c = b.compose(|x| *x + 1); // c will always be 10 times a plus 1
    /// c.subscribe(|x| println!("{}", x)); // Prints 11
    /// a.update(|x| x + 1).ok(); // Prints 21
    /// ```
    #[inline]
    pub fn compose<U: 'static>(&self, composer: impl Fn(&T) -> U + 'static) -> Derived<U> {
        unsafe {
            let internal = self.internal();
            let (derived, idx) = internal.compose(composer);
            derived.set_unsubscriber(self.unsubscriber(idx));
            derived
        }
    }
}

impl<T> Clone for Derived<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> Subscribable<T> for Derived<T> {
    #[inline]
    #[must_use]
    fn subscribe(&self, notify: impl FnMut(&T) + 'static) -> StoreUnsubscriber {
        let idx = unsafe { self.internal().subscribe(notify) };
        self.unsubscriber(idx)
    }
}

/// An immutable value that can still be subscribed to.
/// 
/// `Value` implements [`Subscribable`], it represents a value that can't be
/// mutated. Subscriptions and unsubscription to a `Value` are much faster than
/// to a [`Store`].
/// 
/// # Examples
/// 
/// ```
/// # use wasmide::prelude::*;
/// let a = Value(1);
/// a.subscribe(|x| println!("{}", x)); // Prints 1
/// ```
#[derive(Clone)]
pub struct Value<T>(pub T);

impl<T> Value<T> {
    /// Retreives the value contained in `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Value(42);
    /// let n = a.take();
    /// assert_eq!(n, 42);
    /// ```
    #[inline]
    pub fn take(self) -> T {
        self.0
    }
}

impl<T> Deref for Value<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Value<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Value<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Subscribable<T> for Value<T> {
    #[inline]
    fn subscribe(&self, mut notify: impl FnMut(&T) + 'static) -> StoreUnsubscriber {
        notify(&self.0);
        StoreUnsubscriber::new(|| ())
    }
}