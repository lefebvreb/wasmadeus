//! Building bricks for reactivity. 
//! 
//! This module provides a trait, [`Subscribable`], which is used to
//! work with stores, and three implementations of it: [`Store`], [`Value`] and [`Derived`].
//! The first one is mutable, the second one is not and the third one is obtained by
//! through composition of store.

use core::cell::UnsafeCell;
use core::mem;

use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;

use crate::errors::{FrontendError, Result};

/// A trait for the ability of a type to be subscribed/unsubscribed to.
/// 
/// Upon subscription with [`Self::subscribe`], a value of type `Self::Unsubscriber` is returned
/// which allows unsubscription through the [`Unsubscriber::unsubscribe`] method.
pub trait Subscribable<T> {
    /// The type that is provided upon subscription and that can be used to unsubscribe later.
    type Unsubscriber: Unsubscriber;

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
    fn subscribe(&self, notify: impl FnMut(&T) + 'static) -> Self::Unsubscriber;
}

/// A trait implemented by unsubscription handles.
/// 
/// After calling [`Unsubscriber::unsubscribe`], the store will no
/// longer notify the closure given through [`Subscribable::subscribe`].
pub trait Unsubscriber {
    /// Unsubscribe from the previous subscription. The closure provided to [`Self::subscribe`] will
    /// no longer be notified of changes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::prelude::*;
    /// let a = Store::new(1);
    /// let unsub = a.subscribe(|x| println!("{}", x)); // Prints 1
    /// unsub.unsubscribe();
    /// a.update(|x| x + 1).ok(); // Prints nothing
    /// ```
    fn unsubscribe(&self);
} 

// The internal representation of a store, designed to be held by a Rc.
struct InternalStore<T> {
    // The current data.
    data: T,
    // A map of the subscribers' notify functions indexed by their IDs.
    subscribers: Vec<(usize, Box<dyn FnMut(&T)>)>,
    // A counter that is incremented with each new subscription.
    unique_count: usize,
    // A flag that signal whether the store is currently being updated.
    updating: bool,
    // A vector holding all subscriptions/unscubscribtions operations delayed because of an update.
    delayed: VecDeque<StoreOperation<T>>,
}

// An operation on a Store.
enum StoreOperation<T> {
    Subscribe {
        notify: Box<dyn FnMut(&T)>,
        idx: usize,
    },
    Unsubscribe(usize),
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

    // Pushes a new operation.
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
            StoreOperation::Unsubscribe(idx) => {
                let i = self.subscribers.partition_point(|&(i, _)| i < idx);
                if self.subscribers[i].0 == idx {
                    let _ = self.subscribers.remove(i);
                }
            }
        }
    }

    // Unsubscribes the subscribee with the given ID from the store.
    #[inline]
    fn unsubscribe(&mut self, idx: usize) {
        let op = StoreOperation::Unsubscribe(idx);
        self.push_op(op);
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
    /// a.mutate(|x| x.to_uppercase()).ok(); // Prints "HELLO"
    /// ```
    #[inline]
    pub fn mutate(&self, mutater: impl FnOnce(&mut T)) -> Result<()> {
        // SAFETY: Will toggle the updating flag that gates away other call to update(),
        // so that only one update can be carried out at a time: mutates the data,
        // and then call all the subscribers. Has an exclusive access to subscribers
        // and data by the updating flag.
        // The queue and updating and never borrowed.
        let internal = unsafe { self.internal() };
        
        if internal.updating {
            return Err(FrontendError::StoreUpdating);
        }

        internal.updating = true;

        mutater(&mut internal.data);

        for (_, subscriber) in &mut internal.subscribers {
            subscriber(&internal.data);
        }

        while let Some(op) = internal.delayed.pop_front() {
            internal.do_operation(op);
        }

        internal.updating = false;

        Ok(())
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
    /// let b = a.compose(|x| *x * 10).unwrap(); // b will always be 10 times a
    /// b.subscribe(|x| println!("{}", x)); // Prints 10
    /// a.update(|x| x + 1).ok(); // Prints 20
    /// ```
    #[inline]
    pub fn compose<U: 'static>(&self, composer: impl Fn(&T) -> U + 'static) -> Result<Derived<U>> {
        // SAFETY: only reads updating.
        let internal = unsafe { self.internal() };

        if internal.updating {
            return Err(FrontendError::StoreUpdating);
        }

        // SAFETY: data will be initialized on subscription.
        let derived = Store::new(unsafe { mem::zeroed::<U>() });
        let derived_clone = derived.clone();

        let unsub = self.subscribe(move |data| {
            derived_clone.set(composer(data)).unwrap();
        });

        Ok(Derived::new(derived, unsub))
    }
}

impl<T> Clone for Store<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// The internal value of a store or derived store unscubscriber, with
// a weak reference to the store and the ID of the subscription to
// eventually undo.
struct InternalUnsubscriber<T> {
    weak: Weak<UnsafeCell<InternalStore<T>>>,
    idx: usize,
}

impl<T: 'static> InternalUnsubscriber<T> {
    // Performs the unsubscription and consumes self.
    #[inline]
    fn unsubscribe(&self) {
        if let Some(rc) = self.weak.upgrade() {
            // SAFETY: simply pushes an operation to the store's queue.
            // The queue is never borrowed.
            unsafe { Store(rc).internal().unsubscribe(self.idx); }
        }
    }
}

/// A type that can then be used to unsubscribe from the store.
/// 
/// It is returned by [`Store::subscribe`], and must be passed 
/// to [`Store::unsubscribe`] to unsubscribe from the store.
/// 
/// # Examples
/// 
/// ```
/// # use wasmide::prelude::*;
/// let a = Store::new(1);
/// let unsub = a.subscribe(|x| println!("{}", x)); // Prints 1
/// unsub.unsubscribe();
/// a.set(2); // Prints nothing
/// ```
pub struct StoreUnsubscriber<T>(InternalUnsubscriber<T>);

impl<T> StoreUnsubscriber<T> {
    // Constructs a new store unsubscriber.
    #[inline]
    fn new(store: &Store<T>, idx: usize) -> Self {
        StoreUnsubscriber(InternalUnsubscriber { weak: Rc::downgrade(&store.0), idx })
    }
}

impl<T: 'static> Unsubscriber for StoreUnsubscriber<T> {
    #[inline]
    fn unsubscribe(&self) {
        self.0.unsubscribe();
    }
}

impl<T: 'static> Subscribable<T> for Store<T> {
    type Unsubscriber = StoreUnsubscriber<T>;

    #[inline]
    fn subscribe(&self, notify: impl FnMut(&T) + 'static) -> Self::Unsubscriber {
        // SAFETY: increments the unique_count, and then pushes the operation to the queue.
        // The unique_count and queue are never borrowed.  
        let internal = unsafe { &mut self.internal() };

        let idx = internal.unique_count;
        internal.unique_count += 1;

        let op = StoreOperation::Subscribe {
            notify: Box::new(notify),
            idx,
        };
        
        internal.push_op(op);

        StoreUnsubscriber::new(self, idx)
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
/// let b = a.compose(|x| *x * 10).unwrap(); // b will always be 10 times a
/// b.subscribe(|x| println!("{}", x)); // Prints 10
/// a.update(|x| x + 1).ok(); // Prints 20
/// ```
pub struct Derived<T: 'static> {
    store: Store<T>,
    unsub: Rc<dyn Unsubscriber>,
}

impl<T> Derived<T> {
    // Constructs a new derived store from an internal store and an unsubscriber.
    #[inline]
    fn new(store: Store<T>, unsub: impl Unsubscriber + 'static) -> Self {
        Derived {
            store,
            unsub: Rc::new(unsub),
        }
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
    /// let b = a.compose(|x| *x * 10).unwrap(); // b will always be 10 times a
    /// let c = b.compose(|x| *x + 1).unwrap(); // c will always be 10 times a plus 1
    /// c.subscribe(|x| println!("{}", x)); // Prints 11
    /// a.update(|x| x + 1).ok(); // Prints 21
    /// ```
    #[inline]
    pub fn compose<U: 'static>(&self, composer: impl Fn(&T) -> U + 'static) -> Result<Derived<U>> {
        self.store.compose(composer)
    }
}

impl<T> Clone for Derived<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            unsub: self.unsub.clone(),
        }
    }
}

impl<T> Drop for Derived<T> {
    // When a derived store is dropped, it unsubscribes from the store it is tied to.
    #[inline]
    fn drop(&mut self) {
        if Rc::strong_count(&self.unsub) == 1 {
            self.unsub.unsubscribe();
        }
    }
}

/// A type that can then be used to unsubscribe from a derived store.
/// 
/// It is returned by [`Derived::subscribe`], and must be passed 
/// to [`Derived::unsubscribe`] to unsubscribe from the derived store.
/// 
/// # Examples
/// 
/// ```
/// # use wasmide::prelude::*;
/// # use wasmide::stores::Derived;
/// let a = Store::new(1);
/// let b = a.compose(|x| *x * 10).unwrap();
/// let unsub = b.subscribe(|x| println!("{}", x)); // Prints 10
/// unsub.unsubscribe();
/// a.set(2); // Prints nothing
/// ```
pub struct DerivedUnsubscriber<T>(StoreUnsubscriber<T>);

impl<T: 'static> Unsubscriber for DerivedUnsubscriber<T> {
    #[inline]
    fn unsubscribe(&self) {
        self.0.unsubscribe();
    }
}

impl<T: 'static> Subscribable<T> for Derived<T> {
    type Unsubscriber = DerivedUnsubscriber<T>;

    #[inline]
    fn subscribe(&self, notify: impl FnMut(&T) + 'static) -> Self::Unsubscriber {
        DerivedUnsubscriber(self.store.subscribe(notify))
    }
}

/// An immutable store that can still be subscribed to.
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
    /// Retreives the value contained in this store.
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

/// A type returned by [`Value::subscribe`]. 
/// 
/// It serves as a placeholder for the trait [`Subscribable`]
/// and does litteraly nothing when consumed by [`Value::unsubscribe`].
/// 
/// # Examples
/// 
/// ```
/// # use wasmide::prelude::*;
/// let a = Value(1);
/// let unsub = a.subscribe(|x| println!("{}", x)); // Prints 1
/// unsub.unsubscribe();
/// ```
pub struct ValueUnsubscriber;

impl Unsubscriber for ValueUnsubscriber {
    #[inline]
    fn unsubscribe(&self) {}
}

impl<T> Subscribable<T> for Value<T> {
    type Unsubscriber = ValueUnsubscriber;

    #[inline]
    fn subscribe(&self, mut notify: impl FnMut(&T) + 'static) -> Self::Unsubscriber {
        notify(&self.0);
        ValueUnsubscriber
    }
}