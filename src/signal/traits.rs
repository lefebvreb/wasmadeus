use super::{SignalMutatingError, Unsubscriber};

pub trait Value<T> {
    fn for_each<F>(&self, f: F) -> Unsubscriber<T>
    where
        F: FnMut(&T) + 'static;

    fn for_each_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &mut Unsubscriber<T>) + 'static;
}

pub trait Signal<T>: Value<T> {
    fn try_get(&self) -> Result<T, SignalMutatingError>
    where
        T: Clone;

    fn get(&self) -> T
    where
        T: Clone,
    {
        self.try_get().unwrap()
    }
}
