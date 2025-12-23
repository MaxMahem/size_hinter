use std::iter::FusedIterator;

#[cfg(doc)]
use crate::*;

/// A size hint for an iterator.
pub type SizeHint = (usize, Option<usize>);

/// A universally applicable size hint that never changes and conveys no information.
pub const UNIVERSAL_SIZE_HINT: SizeHint = (0, None);

/// An [`Iterator`] adaptor that provides a custom [`Iterator::size_hint`] implementation.
///
/// This is most useful for providing a specific [`Iterator::size_hint`] implementation or
/// hiding an underlying one for testing. The [`Iterator::size_hint`] implementation provided
/// by this adaptor will change as the iterator is consumed.
///
/// In some cases using this adaptor may allow for optimization, though if an exact length is
/// known it is recommended to use [`ExactLen`] instead. Using this adaptor to wrap an iterator
/// that implements [`ExactSizeIterator`] or `TrustedLen` may lead to performance penalties.
///
/// Note this type is readonly. The field values may be read, but not modified.
///
/// # Safety
///
/// `HintSize` should be safe to use in any scenario, regardless of the values provided.
/// However it is the caller's responsibility to ensure that the values provided are accurate.
/// Providing an incorrect size hint may lead to incorrect behavior or panics in code that relies
/// on these values.
///
/// Since size hints are bounds, not promises of actual values returned, any lower bound value less
/// than or equal to the actual length, and any upper bound value greater than or equal to the actual
/// length, are valid.
///
/// # Examples
///
/// Hiding an iterator's size hint for testing.
///
/// ```rust
/// use size_hinter::{HintSize, UNIVERSAL_SIZE_HINT};
///
/// let mut hidden = HintSize::hide(1..5);
///
/// assert_eq!(hidden.size_hint(), UNIVERSAL_SIZE_HINT, "Initial size hint is universal");
/// assert_eq!(hidden.next(), Some(1), "Underlying iterator is not changed");
/// assert_eq!(hidden.size_hint(), UNIVERSAL_SIZE_HINT, "Size hint remains universal");
/// ```
///
/// Providing a specific size hint.
///
/// ```rust
/// use size_hinter::HintSize;
///
/// let mut iter = HintSize::new(1..5, 3, 6);
///
/// assert_eq!(iter.size_hint(), (3, Some(6)), "Initial size hint");
/// assert_eq!(iter.next(), Some(1), "Underlying iterator is not changed");
/// assert_eq!(iter.size_hint(), (2, Some(5)), "Size hint reflects the new state");
/// ```
#[derive(Debug, Default, Clone)]
#[readonly::make]
pub struct HintSize<I: Iterator> {
    /// The underlying iterator.
    pub iterator: I,
    /// The current size hint.
    pub hint: SizeHint,
}

impl<I: Iterator> HintSize<I> {
    /// Wraps `iterator` with a new [`Iterator::size_hint`] implementation with an initial
    /// [`SizeHint`] of `(lower_bound, Some(upper_bound))`.
    ///
    /// `iterator`'s [`IntoIterator::IntoIter`] must implement [`FusedIterator`] because a meaningful
    /// implementation of [`Iterator::size_hint`] cannot be provided if the iterator is not fused.
    ///
    /// See [type level documentation](HintSize) for more details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use size_hinter::HintSize;
    ///
    /// let mut iter = HintSize::new(1..5, 2, 6);
    ///
    /// assert_eq!(iter.size_hint(), (2, Some(6)), "Initial size hint");
    /// assert_eq!(iter.next(), Some(1), "Underlying iterator is not changed");
    /// assert_eq!(iter.size_hint(), (1, Some(5)), "Size hint reflects the new state");
    /// ```
    #[inline]
    pub fn new<II>(iterator: II, lower_bound: usize, upper_bound: usize) -> Self
    where
        II: IntoIterator<IntoIter = I>,
        I: FusedIterator,
    {
        Self { iterator: iterator.into_iter(), hint: (lower_bound, Some(upper_bound)) }
    }

    /// Wraps `iterator` with a new [`Iterator::size_hint`] implementation with an initial
    /// [`SizeHint`] of `(lower_bound, None)`.
    ///
    /// See [type level documentation](HintSize) for more details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use size_hinter::HintSize;
    ///
    /// let mut iter = HintSize::min(1..5, 2);
    ///
    /// assert_eq!(iter.size_hint(), (2, None), "Initial size hint reflects lower_bound");
    /// assert_eq!(iter.next(), Some(1), "Underlying iterator is not changed");
    /// assert_eq!(iter.size_hint(), (1, None), "Size hint reflects the new state");
    /// ```
    #[inline]
    pub fn min(iterator: impl IntoIterator<IntoIter = I>, lower_bound: usize) -> Self {
        Self { iterator: iterator.into_iter(), hint: (lower_bound, None) }
    }

    /// Wraps `iterator` with a new [`Iterator::size_hint`] implementation with the [`UNIVERSAL_SIZE_HINT`].
    ///
    /// This implementation, and the [`UNIVERSAL_SIZE_HINT`] it returns, is always correct,
    /// and never changes. It is most useful for testing.
    ///
    /// See [type level documentation](HintSize) for more details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use size_hinter::HintSize;
    ///
    /// let mut iter = HintSize::hide(1..5);
    ///
    /// assert_eq!(iter.size_hint(), (0, None), "Initial size hint is universal");
    /// assert_eq!(iter.next(), Some(1), "Underlying iterator is not changed");
    /// assert_eq!(iter.size_hint(), (0, None), "Size hint is universal");
    /// ```
    #[inline]
    pub fn hide(iterator: impl IntoIterator<IntoIter = I>) -> Self {
        Self { iterator: iterator.into_iter(), hint: UNIVERSAL_SIZE_HINT }
    }

    /// Consumes the adaptor and returns the underlying iterator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use size_hinter::HintSize;
    ///
    /// let iter: std::vec::IntoIter<i32> = vec![1, 2, 3].into_iter();
    /// let hint_iter = HintSize::hide(iter);
    /// let inner: std::vec::IntoIter<i32> = hint_iter.into_inner();
    /// ```
    #[inline]
    pub fn into_inner(self) -> I {
        self.iterator
    }

    /// Decrements the size hint by one.
    #[inline]
    fn decrement_hint(&mut self) {
        self.hint.0 = self.hint.0.saturating_sub(1);
        self.hint.1 = self.hint.1.map(|upper| upper.saturating_sub(1));
    }
}

impl<I: Iterator> Iterator for HintSize<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            item @ Some(_) => {
                self.decrement_hint();
                item
            }
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> SizeHint {
        self.hint
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for HintSize<I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.iterator.next_back() {
            item @ Some(_) => {
                self.decrement_hint();
                item
            }
            None => None,
        }
    }
}

impl<I: Iterator + FusedIterator> FusedIterator for HintSize<I> {}
