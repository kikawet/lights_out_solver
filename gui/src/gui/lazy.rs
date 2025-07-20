///
/// Enum to manage state of deferred computed values
///
/// This enum should be used while waiting for async results,
/// consider Option when only Empty and Completed states are required
///
/// This Enum is not a future and state is meant to be computed and updated manually.
///
#[derive(PartialEq)]
pub enum Lazy<T> {
    /// Starting value
    Empty,
    /// Resource is being computed
    Requested,
    /// Resource computation completed, you can use now the result
    Completed(T),
}

impl<T> Default for Lazy<T> {
    /// Get [`Lazy::Empty`]
    fn default() -> Self {
        Lazy::Empty
    }
}

impl<T> Lazy<T> {
    /// Throw away state and revert to default
    pub fn discard(&mut self) {
        *self = Lazy::default();
    }

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
