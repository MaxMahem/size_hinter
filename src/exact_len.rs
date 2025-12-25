use core::{iter::FusedIterator, ops::RangeBounds};

use fluent_result::bool::Then;

#[cfg(doc)]
use crate::*;
use crate::{InvalidSizeHint, SizeHint};

/// A [`FusedIterator`] adaptor that provides an exact length via [`ExactSizeIterator`].
///
/// This is useful for iterators that don't normally implement [`ExactSizeIterator`] but for which
/// the exact number of elements the iterator will yield is known. This may allow for performance
/// optimizations, but may also prevent optimizations if the wrapped iterator implements
/// `TrustedLen`.
///
/// Implemented in terms of [`FusedIterator`], because meaningful a implementation
/// of [`ExactSizeIterator`] is not possible after the wrapped iterator returns [`None`].
///
/// Note that this type is readonly. Fields maybe be read, but not modified.
///
/// # Safety
///
/// `ExactLen` is always safe to use - it will never cause undefined behavior or memory unsafety,
/// regardless of the len value provided.
///
/// Validation during construction ensures that this adaptor's will not contradict the wrapped
/// [`Iterator::size_hint`]. However it is still the caller's responsibility to ensure that the
/// provided length is accurate. Inaccurate values may cause incorrect behavior or panics in
/// code that relies on these values.
///
/// # Examples
///
/// ```rust
/// # use size_hinter::ExactLen;
/// let odd_numbers = (1..=5).filter(|x| x % 2 == 1);
/// let mut three_odds = ExactLen::new(odd_numbers, 3);
///
/// assert_eq!(three_odds.len(), 3, "len should match the initial length");
/// assert_eq!(three_odds.size_hint(), (3, Some(3)), "size_hint should match the len");
///
/// assert_eq!(three_odds.next(), Some(1), "The underlying iterator is unchanged");
/// assert_eq!(three_odds.len(), 2, "len should match the remaining length");
/// assert_eq!(three_odds.size_hint(), (2, Some(2)), "size_hint should match len");
///
/// assert_eq!(three_odds.next_back(), Some(5), "The underlying iterator is unchanged");
/// assert_eq!(three_odds.len(), 1, "len should match the remaining length");
/// assert_eq!(three_odds.size_hint(), (1, Some(1)), "size_hint should match len");
/// ```
#[derive(Debug, Clone)]
#[readonly::make]
pub struct ExactLen<I: FusedIterator> {
    /// The underlying iterator.
    pub iterator: I,
    /// The exact length of the iterator.
    pub len: usize,
}

impl<I: FusedIterator> ExactLen<I> {
    /// Wraps `iterator` with a new [`ExactSizeIterator::len`] implementation based on the
    /// provided `len` value.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `iterator`'s size hint is not valid
    /// - `len` is less than `iterator`'s lower bound
    /// - `len` is greater than `iterator`'s upper bound (if present)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let odd_numbers = (1..=5).filter(|x| x % 2 == 1);
    /// let mut three_odds = ExactLen::new(odd_numbers, 3);
    /// assert_eq!(three_odds.len(), 3, "len should match the initial length");
    /// ```
    #[inline]
    pub fn new(iterator: impl IntoIterator<IntoIter = I>, len: usize) -> Self {
        Self::try_new(iterator, len).expect("len should be within the wrapped iterator's size hint bounds")
    }

    /// Tries to wraps `iterator` in a new [`ExactSizeIterator::len`] based on `len`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidSizeHint`] if `len` is not within `iterator`'s size hint.
    ///
    /// # Panics
    ///
    /// Panics if `iterator`'s size hint is not valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::{ExactLen, InvalidSizeHint};
    /// let err: InvalidSizeHint = ExactLen::try_new(1..5, 10).expect_err("iter size hint should not contain len");
    /// ```
    #[inline]
    pub fn try_new(iterator: impl IntoIterator<IntoIter = I>, len: usize) -> Result<Self, InvalidSizeHint> {
        let iterator = iterator.into_iter();
        let wrapped = SizeHint::try_from(iterator.size_hint()).expect("wrapped iterator size_hint should be valid");
        (!wrapped.contains(&len)).then_err(InvalidSizeHint).map(|()| Self { iterator, len })
    }

    /// Consumes the adaptor and returns the underlying iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use size_hinter::ExactLen;
    ///
    /// let iter: std::vec::IntoIter<i32> = vec![1, 2, 3].into_iter();
    /// let exact_iter = ExactLen::new(iter, 3);
    /// let inner: std::vec::IntoIter<i32> = exact_iter.into_inner();
    /// ```
    #[inline]
    pub fn into_inner(self) -> I {
        self.iterator
    }
}

impl<I: FusedIterator> Iterator for ExactLen<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            item @ Some(_) => {
                self.len = self.len.saturating_sub(1);
                item
            }
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        SizeHint::exact(self.len).into()
    }
}

impl<I: FusedIterator> ExactSizeIterator for ExactLen<I> {
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
}

impl<I: DoubleEndedIterator + FusedIterator> DoubleEndedIterator for ExactLen<I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.iterator.next_back() {
            Some(item) => {
                self.len = self.len.saturating_sub(1);
                Some(item)
            }
            None => None,
        }
    }
}

impl<I: FusedIterator> FusedIterator for ExactLen<I> {}
