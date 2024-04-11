pub enum Lazy<T> {
    Empty,
    Requested,
    Completed(T),
}

impl<T> Lazy<T> {
    #[inline]
    pub const fn as_ref(&self) -> Lazy<&T> {
        match *self {
            Lazy::Empty => Lazy::Empty,
            Lazy::Requested => Lazy::Requested,
            Lazy::Completed(ref x) => Lazy::Completed(x),
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> Lazy<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Lazy::Empty => Lazy::Empty,
            Lazy::Requested => Lazy::Requested,
            Lazy::Completed(x) => Lazy::Completed(f(x)),
        }
    }

    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Lazy::Completed(x) => x,
            Lazy::Empty | Lazy::Requested => T::default(),
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Lazy<&mut T> {
        match *self {
            Lazy::Completed(ref mut x) => Lazy::Completed(x),
            Lazy::Requested => Lazy::Requested,
            Lazy::Empty => Lazy::Empty,
        }
    }
}
