use core::{iter::FusedIterator, ops::Not};

use fluent_result::bool::Then;

use crate::InvalidSizeHint;
use crate::size_hint::SizeHint;

#[cfg(doc)]
use crate::*;

/// An [`Iterator`] adaptor that provides a custom [`Iterator::size_hint`] implementation.
///
/// This is most useful for providing a specific [`Iterator::size_hint`] implementation or
/// hiding an underlying one for testing. The [`Iterator::size_hint`] implementation provided
/// by this adaptor will change as the iterator is consumed.
///
/// In some cases using this adaptor may allow for optimization, though if an exact length is
/// known it is recommended to use [`ExactLen`] instead. Using this adaptor to wrap an iterator
/// that implements [`ExactSizeIterator`] or `TrustedLen` may also prevent optimizations.
///
/// Note this type is readonly. The field values may be read, but not modified.
///
/// # Fused iterator requirement
///
/// [`HintSize`]s with an unbounded size hint are required to be wrapped around a [`FusedIterator`],
/// This is required because a valid unbounded upper size hint may be non-zero even after the
/// iterator returns [`None`], and thus a valid hint cannot be provided if iteration was to resume.
/// Consider using an unbounded wrapper ([`HintSize::hide`] or [`HintSize::min`]) if the iterator is
/// fused.
///
/// # Safety
///
/// `HintSize` is always safe to use - it will never cause undefined behavior or memory unsafety,
/// regardless of the hint values provided.
///
/// Validation during construction ensures that hints don't contradict the wrapped iterator's
/// guarantees. This is, the adaptor cannot produce a size hint that claims a upper bound less than
/// the wrapped iterator's lower bound or a lower bound greater than the wrapped iterator's upper
/// bound. If necessary this validation can be bypassed by wrapping in first [`HintSize::hide`] and
/// then in the desired adaptor.
///
/// Regardless, it is still the caller's responsibility to ensure that the size hints accurately
/// represent the number of elements remaining in the iterator. Incorrect size hints may cause
/// incorrect behavior or panics in code that relies on these values.
///
/// # Examples
///
/// Hiding an iterator's size hint for testing.
///
/// ```rust
/// # use size_hinter::{HintSize, SizeHint};
/// let mut hidden = HintSize::hide(1..5);
///
/// assert_eq!(hidden.size_hint(), SizeHint::UNIVERSAL, "Initial size hint is universal");
/// assert_eq!(hidden.next(), Some(1), "Underlying iterator is not changed");
/// assert_eq!(hidden.size_hint(), SizeHint::UNIVERSAL, "Size hint remains universal");
/// ```
///
/// Providing a specific size hint.
///
/// ```rust
/// # use size_hinter::HintSize;
/// let mut iter = HintSize::new(1..5, 3, 6);
///
/// assert_eq!(iter.size_hint(), (3, Some(6)), "should match the provided size hint");
/// assert_eq!(iter.next(), Some(1), "Underlying iterator is not changed");
/// assert_eq!(iter.next_back(), Some(4), "Underlying iterator is not changed");
/// assert_eq!(iter.size_hint(), (1, Some(4)), "should reflect the new state");
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
    /// Internal monomorphized failable constructor. Creates a [`HintSize`] with the provided `hint`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidSizeHint`] if the hint does not overlap with the `iterator`'s size hint.
    ///
    /// # Panics
    ///
    /// Panics if `iterator`'s [`Iterator::size_hint`] is invalid
    #[inline]
    #[track_caller]
    fn try_new_impl(iterator: I, hint: SizeHint) -> Result<Self, InvalidSizeHint> {
        let wrapped: SizeHint = iterator.size_hint().try_into().expect("iterator's size hint should be valid");
        SizeHint::overlaps(hint, wrapped).not().then_err(InvalidSizeHint)?;
        Ok(Self { iterator, hint })
    }

    /// Wraps `iterator` in a new [`HintSize`] with an initial bounded size hint of
    /// `(lower, Some(upper))`.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `iterator`'s [`Iterator::size_hint`] is invalid
    /// - `lower > upper`
    /// - `upper` is less than the wrapped iterator's lower bound
    /// - `lower` is greater than the wrapped iterator's upper bound (if present)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::HintSize;
    /// let mut iter = HintSize::new(1..5, 2, 6);
    /// assert_eq!(iter.size_hint(), (2, Some(6)), "should match the provided size hint");
    /// ```
    #[inline]
    pub fn new<IntoIter>(iterator: IntoIter, lower: usize, upper: usize) -> Self
    where
        IntoIter: IntoIterator<IntoIter = I>,
        I: FusedIterator,
    {
        Self::try_new(iterator, lower, upper).expect("Invalid size hint")
    }

    /// Tries to wrap `iterator` in a new [`HintSize`] with an initial bounded size hint of
    /// `(lower, Some(upper))`.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidSizeHint`] if:
    /// - `lower > upper`
    /// - `upper` is less than the wrapped iterator's lower bound
    /// - `lower` is greater than the wrapped iterator's upper bound (if present)
    ///
    /// # Panics
    ///
    /// Panics if `iterator`'s [`Iterator::size_hint`] is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::{HintSize, InvalidSizeHint};
    /// # fn main() -> Result<(), InvalidSizeHint> {
    /// let mut iter = HintSize::try_new(1..5, 2, 6)?;
    /// assert_eq!(iter.size_hint(), (2, Some(6)), "Initial size hint");
    ///
    /// let err: InvalidSizeHint = HintSize::try_new(1..5, 6, 2).expect_err("lower bound should be > upper bound");
    /// let err: InvalidSizeHint = HintSize::try_new(1..5, 6, 10).expect_err("hint lower bound > iterator's upper bound");
    /// let err: InvalidSizeHint = HintSize::try_new(1..5, 1, 3).expect_err("hint upper bound < iterator's lower bound");
    /// # Ok(())
    /// }
    /// ```
    #[inline]
    pub fn try_new<II>(iterator: II, lower: usize, upper: usize) -> Result<Self, InvalidSizeHint>
    where
        II: IntoIterator<IntoIter = I>,
    {
        Self::try_new_impl(iterator.into_iter(), SizeHint::try_bounded(lower, upper)?)
    }

    /// Wraps `iterator` in a new [`HintSize`] with an unbounded size hint based on `lower`.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `iterator`'s [`Iterator::size_hint`] is invalid
    /// - `lower` is greater than the wrapped iterator's upper bound (if present).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::HintSize;
    /// let mut iter = HintSize::min(1..5, 2);
    /// assert_eq!(iter.size_hint(), (2, None), "Initial size hint reflects lower");
    /// ```
    #[inline]
    pub fn min(iterator: impl IntoIterator<IntoIter = I>, lower: usize) -> Self {
        Self::try_min(iterator, lower).expect("Invalid size hint")
    }

    /// Tries to wrap `iterator` in a new [`HintSize`] with an unbounded size hint based on `lower`.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidSizeHint`] if `lower` is greater than the wrapped iterator's upper
    /// bound (if present).
    ///
    /// # Panics
    ///
    /// Panics if `iterator`'s [`Iterator::size_hint`] is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::{HintSize, InvalidSizeHint};
    /// # fn main() -> Result<(), InvalidSizeHint> {
    /// let iter = HintSize::try_min(1..5, 2).expect("Should be valid");
    /// assert_eq!(iter.size_hint(), (2, None));
    ///
    /// let err: InvalidSizeHint = HintSize::try_min(1..5, 6).expect_err("lower bound should be > wrapped iterator's upper bound");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn try_min(iterator: impl IntoIterator<IntoIter = I>, lower: usize) -> Result<Self, InvalidSizeHint> {
        Self::try_new_impl(iterator.into_iter(), SizeHint::unbounded(lower))
    }

    /// Wraps `iterator` with a new [`Iterator::size_hint`] implementation with a universal size hint.
    ///
    /// This implementation, and the size hint it returns, is always correct, and never changes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::HintSize;
    /// let mut iter = HintSize::hide(1..5);
    ///
    /// assert_eq!(iter.size_hint(), (0, None), "Initial size hint is universal");
    /// assert_eq!(iter.next(), Some(1), "Underlying iterator is not changed");
    /// assert_eq!(iter.size_hint(), (0, None), "Size hint remains universal");
    /// ```
    #[inline]
    pub fn hide(iterator: impl IntoIterator<IntoIter = I>) -> Self {
        Self { iterator: iterator.into_iter(), hint: SizeHint::UNIVERSAL }
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
}

impl<I: Iterator> Iterator for HintSize<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            item @ Some(_) => {
                self.hint = self.hint.decrement();
                item
            }
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.hint.into()
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for HintSize<I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.iterator.next_back() {
            item @ Some(_) => {
                self.hint = self.hint.decrement();
                item
            }
            None => None,
        }
    }
}

impl<I: Iterator + FusedIterator> FusedIterator for HintSize<I> {}
