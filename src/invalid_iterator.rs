use core::iter::FusedIterator;
use core::marker::PhantomData;

/// A [`Iterator`] that reports an invalid (empty) size hint, with lower bound > upper bound.
///
/// This is useful for testing how consumers handle invalid size hints.
/// It panics when [`Self::next`] or [`Self::next_back`] is called,
/// and returns an invalid size hint for [`Self::size_hint`].
///
/// If the type parameter is not important, consider using [`INVALID_UNIT_ITERATOR`].
/// If you need to use a generic type parameter, consider using [`InvalidIterator::DEFAULT`].
///
/// # Examples
///
/// ```rust
/// # use size_hinter::InvalidIterator;
/// let iter = InvalidIterator::<()>::DEFAULT;
/// let (lower, upper) = iter.size_hint();
/// assert!(lower > upper.unwrap(), "Size hint should be invalid");
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct InvalidIterator<T = ()>(PhantomData<T>);

/// A constant instance of an [`InvalidIterator`] with `()` as the item type.
pub const INVALID_UNIT_ITERATOR: InvalidIterator<()> = InvalidIterator::DEFAULT;

impl<T> InvalidIterator<T> {
    /// A constant instance of `InvalidIterator`.
    pub const DEFAULT: Self = Self::new();

    /// Creates a new `InvalidIterator`.
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Iterator for InvalidIterator<T> {
    type Item = T;

    /// Always panics.
    fn next(&mut self) -> Option<Self::Item> {
        panic!("next called on InvalidIterator")
    }

    /// Always returns an invalid size hint, with lower bound > upper bound.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (10, Some(5))
    }
}

impl<T> DoubleEndedIterator for InvalidIterator<T> {
    /// Always panics.
    fn next_back(&mut self) -> Option<Self::Item> {
        panic!("next_back called on InvalidIterator")
    }
}

impl<T> FusedIterator for InvalidIterator<T> {}
