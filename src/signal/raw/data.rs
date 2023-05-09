use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use crate::signal::{SignalError, Result};

/// The state of a signal's data.
#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    Idling,
    Borrowed,
    Mutating,
    Uninit,
}

pub struct SignalData<T> {
    state: Cell<State>,
    value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> SignalData<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self {
            state: Cell::new(State::Idling),
            value: UnsafeCell::new(MaybeUninit::new(initial_value)),
        }
    }

    #[inline]
    pub fn uninit() -> Self {
        Self {
            state: Cell::new(State::Uninit),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    #[inline]
    pub fn borrow_then<F>(&self, action: F) -> Result<()>
    where
        F: FnOnce(&T),
    {
        let value = self.value.get();

        match self.state.get() {
            State::Idling => unsafe {
                self.state.set(State::Borrowed);
                action((*value).assume_init_ref());
                self.state.set(State::Mutating);
            },
            State::Borrowed => unsafe {
                action((*value).assume_init_ref());
            },
            State::Mutating => return Err(SignalError),
            _ => (),
        }

        Ok(())
    }

    pub fn try_get(&self) -> Result<T> 
    where
        T: Clone,
    {
        if matches!(self.state.get(), State::Mutating | State::Uninit) {
            return Err(SignalError);
        }

        let value = self.value.get();
        Ok(unsafe { (*value).assume_init_ref().clone() })
    }

    pub fn try_set(&self, new_value: T) -> Result<()> {
        let value = self.value.get();

        match self.state.get() {
            State::Idling => unsafe {
                *(*value).assume_init_mut() = new_value;
            },
            State::Uninit => unsafe {
                (*value).write(new_value);
            },
            _ => return Err(SignalError),
        }

        Ok(())
    }

    pub fn try_mutate<F>(&self, mutate: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        if self.state.get() != State::Idling {
            return Err(SignalError);
        }

        self.state.set(State::Mutating);
        unsafe {
            let value = self.value.get();
            mutate((*value).assume_init_mut());
        }
        self.state.set(State::Idling);

        Ok(())
    }
}
