use std::iter::FusedIterator;

use crate::HintSize;

#[cfg(doc)]
use crate::*;

/// Extension trait for [`Iterator`] and [`FusedIterator`] to create iterators with custom
/// [`Iterator::size_hint`] and/or [`ExactSizeIterator::len`] implementations.
#[sealed::sealed]
pub trait SizeHinter: Iterator + Sized {
    /// Wraps this [`FusedIterator`] in a [`HintSize`] with a [`SizeHint`] based on
    /// `lower_bound` and `upper_bound`.
    ///
    /// It is the caller's responsibility to ensure that `lower_bound` and `upper_bound`
    /// are valid bounds for the number of elements remaining in the iterator.
    /// See [`HintSize::new`] for more details.
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
    fn hint_size(self, lower_bound: usize, upper_bound: usize) -> HintSize<Self>
    where
        Self: FusedIterator,
    {
        HintSize::new(self, lower_bound, upper_bound)
    }

    /// Wraps this [`Iterator`] in a [`HintSize`] with a [`SizeHint`] based on `lower_bound`.
    ///
    /// It is the caller's responsibility to ensure that `lower_bound` is a valid lower bound
    /// for the number of elements remaining in the iterator. See [`HintSize::min`] for more details.
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
    fn hint_min(self, lower_bound: usize) -> HintSize<Self> {
        HintSize::min(self, lower_bound)
    }

    /// Wraps this [`Iterator`] in a [`HintSize`] with a [`UNIVERSAL_SIZE_HINT`].
    ///
    /// This implementation, and the [`UNIVERSAL_SIZE_HINT`] it returns, is always correct,
    /// and never changes. It is most useful for testing.
    ///
    /// See [`HintSize::hide`] for more details.
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

    /// Wraps this [`FusedIterator`] in a [`ExactLen`] with a length of `len`.
    ///
    /// It is the caller's responsibility to ensure that `len` accurately represents the
    /// number of elements remaining in the iterator. This can be a performance
    /// *pessimization* if the iterator already implements `TrustedLen`, even if it does
    /// not implement [`ExactSizeIterator`] (for example, [`std::iter::Chain`]). See
    /// [`ExactLen::new`] for more details.
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
}

#[sealed::sealed]
impl<I: Iterator> SizeHinter for I {}
