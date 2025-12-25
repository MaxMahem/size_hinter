use core::iter::FusedIterator;

use crate::HintSize;

#[cfg(doc)]
use crate::*;

/// Extension trait for [`Iterator`] and [`FusedIterator`] to create iterators with custom
/// [`Iterator::size_hint`] and/or [`ExactSizeIterator::len`] implementations.
#[sealed::sealed]
pub trait SizeHinter: Iterator + Sized {
    /// Wraps this [`FusedIterator`] in a [`HintSize`] that produces a [`SizeHint`] based on
    /// `lower` and `upper`.
    ///
    /// This is most useful for testing, but can also enable optimizations if a more accurate
    /// [`SizeHint`] is available. Prefer [`exact_len`] if the exact number of elements is known.
    /// This adaptor can also prevent optimizations if the iterator already implements `TrustedLen`.
    ///
    /// It is the caller's responsibility to ensure that `lower` and `upper` are accurate bounds
    /// for the number of elements remaining in this iterator. Incorrect values may cause errors or
    /// panics in code that relies on this [`Iterator::size_hint`].
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `lower > upper`
    /// - `upper` is less than this [`Iterator::size_hint`]'s lower bound
    /// - `lower` is greater than this [`Iterator::size_hint`]'s upper bound (if present)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use size_hinter::SizeHinter;
    ///
    /// let mut iter = (1..5).hint_size(2, 6);
    ///
    /// assert_eq!(iter.size_hint(), (2, Some(6)), "Should match initial size hint");
    /// assert_eq!(iter.next(), Some(1), "Should not change underlying iterator");
    /// assert_eq!(iter.size_hint(), (1, Some(5)), "Should reflect new state");
    /// ```
    #[inline]
    fn hint_size(self, lower: usize, upper: usize) -> HintSize<Self>
    where
        Self: FusedIterator,
    {
        HintSize::new(self, lower, upper)
    }

    /// Wraps this [`Iterator`] in a [`HintSize`] that produces a [`SizeHint`] based on `lower`.
    ///
    /// This is useful for testing, but can also enable optimizations if a more accurate lower
    /// bound is known for this iterator. This adaptor can also prevent optimizations if this
    /// iterator already implements `TrustedLen`.
    ///
    /// It is the caller's responsibility to ensure that `lower` is an accurate lower bound for the
    /// number of elements remaining in this iterator. An incorrect value may cause errors or
    /// panics in code that relies on this [`Iterator::size_hint`].
    ///
    /// # Panics
    ///
    /// Panics if `lower` is greater than the upper bound of this [`Iterator::size_hint`] (if present).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use size_hinter::SizeHinter;
    ///
    /// let mut iter = (1..5).hint_min(4);
    ///
    /// assert_eq!(iter.size_hint(), (4, None), "Should match initial lower bound");
    /// assert_eq!(iter.next(), Some(1), "Should not change underlying iterator");
    /// assert_eq!(iter.size_hint(), (3, None), "Should reflect new lower bound");
    /// ```
    #[inline]
    fn hint_min(self, lower: usize) -> HintSize<Self> {
        HintSize::min(self, lower)
    }

    /// Wraps this [`Iterator`] in a [`HintSize`] that produces a [`UNIVERSAL_SIZE_HINT`].
    ///
    /// This implementation, and the [`UNIVERSAL_SIZE_HINT`] it returns, is always correct,
    /// and never changes. It is most useful for testing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use size_hinter::SizeHinter;
    ///
    /// let mut iter = (1..5).hide_size();
    ///
    /// assert_eq!(iter.size_hint(), (0, None), "Should match universal size hint");
    /// assert_eq!(iter.next(), Some(1), "Should not change underlying iterator");
    /// assert_eq!(iter.size_hint(), (0, None), "Should still match universal size hint");
    /// ```
    #[inline]
    fn hide_size(self) -> HintSize<Self> {
        HintSize::hide(self)
    }

    /// Wraps this [`FusedIterator`] in a [`ExactLen`] that provides [`ExactSizeIterator::len`]
    /// based on `len`.
    ///
    /// This is useful for iterators that don't normally implement [`ExactSizeIterator`] but for
    /// which the exact number of elements the iterator will yield is known. This may allow
    /// performance optimizations. However it may also prevent optimizations if the wrapped
    /// iterator already implements `TrustedLen`, even if it does not implement
    /// [`ExactSizeIterator`]. For example, [`Chain`].
    ///
    /// [`ExactLen`] also produces a corresponding [`SizeHint`] based on `len`.
    ///
    /// It is the caller's responsibility to ensure that `len` accurately represents the number of
    /// elements remaining in this iterator. An incorrect value may cause errors or panics in code
    /// that relies on [`ExactSizeIterator::len`].
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - the wrapped [`Iterator::size_hint`] is invalid
    /// - `len` is less than the wrapped [`Iterator::size_hint`]'s lower bound
    /// - `len` is greater than the wrapped [`Iterator::size_hint`]'s upper bound (if present)
    ///
    /// # Examples
    ///
    /// ```
    /// use size_hinter::SizeHinter;
    ///
    /// let mut iter = (1..5).exact_len(4);
    ///
    /// assert_eq!(iter.len(), 4, "Length should match len");
    /// assert_eq!(iter.size_hint(), (4, Some(4)), "Size hint should match len");
    ///
    /// assert_eq!(iter.next(), Some(1), "Should not change underlying iterator");
    /// assert_eq!(iter.len(), 3, "Length should match new len");
    /// assert_eq!(iter.size_hint(), (3, Some(3)), "Size hint should match new len");
    /// ```
    #[inline]
    fn exact_len(self, len: usize) -> crate::ExactLen<Self>
    where
        Self: FusedIterator,
    {
        crate::ExactLen::new(self, len)
    }

    /// Tries to wrap this [`FusedIterator`] in a [`ExactLen`] that provides [`ExactSizeIterator::len`]
    /// based on `len`.
    ///
    /// See [`Self::exact_len`] for more details.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidSizeHint`] if the wrapped iterator's size hint does not contain `len`.
    ///
    /// # Panics
    ///
    /// Panics if the wrapped iterator's [`Iterator::size_hint`] is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use size_hinter::{SizeHinter, InvalidSizeHint};
    /// # fn main() -> Result<(), InvalidSizeHint> {
    /// let mut iter = (1..=5).filter(|&x| x % 2 == 0).try_exact_len(3)?;
    /// assert_eq!(iter.len(), 3, "Length should match len");
    /// assert_eq!(iter.size_hint(), (3, Some(3)), "Size hint should match len");
    ///
    /// let err: InvalidSizeHint = (10..20).try_exact_len(15)
    ///     .expect_err("Len should not be within wrapped iterator size hint");
    /// # Ok(())
    /// ```
    #[inline]
    fn try_exact_len(self, len: usize) -> Result<crate::ExactLen<Self>, crate::InvalidSizeHint>
    where
        Self: FusedIterator,
    {
        crate::ExactLen::try_new(self, len)
    }
}

#[sealed::sealed]
impl<I: Iterator> SizeHinter for I {}
