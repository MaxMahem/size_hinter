use core::{iter::FusedIterator, panic};

use crate::SizeHint;

/// A test [`Iterator`] that can not be iterated over, but has an arbitrary size hint.
///
/// This is useful for testing how consumers handle various size hints.
///
/// # Type parameters
///
/// * `T` - The nominal item type of the iterator.
///
/// /// # Examples
///
/// ```rust
/// # use size_hinter::TestIterator;
/// let iter = TestIterator::<()>::new((5, Some(10)));
/// assert_eq!(iter.size_hint(), (5, Some(10)));
/// ```
pub struct TestIterator<T = ()> {
    size_hint: (usize, Option<usize>),
    _marker: core::marker::PhantomData<T>,
}

impl<T> TestIterator<T> {
    /// Creates a new [`TestIterator`] with the given `size_hint` as its size hint.
    ///
    /// The validity of the size hint is not checked.
    ///
    /// # Arguments
    ///
    /// * `size_hint` - The size hint to use. It's validity is not checked.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::TestIterator;
    /// let iter = TestIterator::<()>::new((5, Some(10)));
    /// assert_eq!(iter.size_hint(), (5, Some(10)));
    /// ```
    #[must_use]
    pub const fn new(size_hint: (usize, Option<usize>)) -> Self {
        Self { size_hint, _marker: core::marker::PhantomData }
    }

    /// Creates a new [`TestIterator`] with an exact size hint.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::TestIterator;
    /// let iter = TestIterator::<()>::exact(5);
    /// assert_eq!(iter.size_hint(), (5, Some(5)));
    /// ```
    #[must_use]
    pub const fn exact(len: usize) -> Self {
        Self::new((len, Some(len)))
    }

    /// Creates a new [`TestIterator`] with an invalid size hint.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::TestIterator;
    /// let iter = TestIterator::<()>::invalid();
    /// let (lower, upper) = iter.size_hint();
    /// assert!(lower > upper.unwrap(), "Size hint should be invalid");
    /// ```
    #[must_use]
    pub const fn invalid() -> Self {
        Self::INVALID
    }

    /// A [`TestIterator`] with a [`SizeHint::UNIVERSAL`] size hint.
    pub const UNIVERSAL: Self = Self::new(SizeHint::UNIVERSAL.as_hint());

    /// A [`TestIterator`] with a [`SizeHint::ZERO`] size hint.
    pub const ZERO: Self = Self::new(SizeHint::ZERO.as_hint());

    /// A [`TestIterator`] with an invalid size hint.
    pub const INVALID: Self = Self::new((10, Some(5)));
}

impl<T> Iterator for TestIterator<T> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint
    }

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!("TestIterator is not iteratable");
    }
}

impl<T> FusedIterator for TestIterator<T> {}

impl<T> ExactSizeIterator for TestIterator<T> {
    /// Returns the exact length of the iterator.
    ///
    /// # Panics
    ///
    /// Panics if the size hint is not exact.
    fn len(&self) -> usize {
        match self.size_hint() {
            (lower, Some(upper)) if lower == upper => lower,
            _ => panic!("Inexact size hint"),
        }
    }
}

impl<T> DoubleEndedIterator for TestIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        unimplemented!("TestIterator is not iteratable");
    }
}
