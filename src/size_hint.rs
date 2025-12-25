use core::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Error type for invalid size hints where the lower bound exceeds the upper bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid size hint: values describe an invalid or empty range")]
pub struct InvalidSizeHint;

/// A size hint for an iterator.
///
/// This is a wrapper around the standard iterator size hint tuple `(usize, Option<usize>)`,
/// providing additional functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[readonly::make]
pub struct SizeHint {
    /// The lower bound of the size hint.
    pub lower: usize,
    /// The upper bound of the size hint.
    pub upper: Option<usize>,
}

impl SizeHint {
    /// The universal size hint that is always valid, never changes and conveys no information.
    pub const UNIVERSAL: Self = Self { lower: 0, upper: None };

    /// Creates a new size hint with the given lower and optional upper bounds.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidSizeHint`] if `lower` is greater than `upper`.
    ///
    /// # Panics
    ///
    /// Panics if the wrapped [`Iterator::size_hint`] is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::{SizeHint, InvalidSizeHint};
    /// let hint = SizeHint::bounded(5, 10);
    /// assert_eq!(hint.lower, 5);
    /// assert_eq!(hint.upper, Some(10));
    ///```
    #[inline]
    #[must_use]
    pub fn bounded(lower: usize, upper: usize) -> Self {
        Self::try_bounded(lower, upper).expect("size hint should be valid")
    }

    /// Tries to create a new bounded [`SizeHint`] with the given `lower` and `upper` bounds.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidSizeHint`] if `lower` is greater than `upper`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::{SizeHint, InvalidSizeHint};
    /// # fn main() -> Result<(), InvalidSizeHint> {
    /// let hint = SizeHint::bounded(5, 10)?;
    /// assert_eq!(hint.lower, 5);
    /// assert_eq!(hint.upper, Some(10));
    ///
    /// let err: InvalidSizeHint = SizeHint::bounded(10, 5).expect_err("SizeHint should be invalid");
    /// # Ok(())
    /// # }
    ///```
    #[inline]
    pub const fn try_bounded(lower: usize, upper: usize) -> Result<Self, InvalidSizeHint> {
        match lower > upper {
            true => Err(InvalidSizeHint),
            false => Ok(Self { lower, upper: Some(upper) }),
        }
    }

    /// Creates a new size hint with the given lower bound and no upper bound.
    ///
    /// ```rust
    /// # use size_hinter::SizeHint;
    /// let hint = SizeHint::unbounded(5);
    /// assert_eq!(hint.lower, 5);
    /// assert_eq!(hint.upper, None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn unbounded(lower: usize) -> Self {
        Self { lower, upper: None }
    }

    /// Creates a new size hint with an exact count.
    ///
    /// ```rust
    /// # use size_hinter::SizeHint;
    /// let hint = SizeHint::exact(5);
    /// assert_eq!(hint.lower, 5);
    /// assert_eq!(hint.upper, Some(5));
    /// ```
    #[inline]
    #[must_use]
    pub const fn exact(len: usize) -> Self {
        Self { lower: len, upper: Some(len) }
    }

    /// Returns the size hint as a tuple `(lower, upper)`.
    #[inline]
    #[must_use]
    pub const fn size_hint(&self) -> (usize, Option<usize>) {
        (self.lower, self.upper)
    }

    /// Returns a new [`SizeHint`] with the lower and upper bounds (if present) decremented by 1.
    #[inline]
    #[must_use]
    pub fn decrement(self) -> Self {
        Self { lower: self.lower.saturating_sub(1), upper: self.upper.map(|upper| upper.saturating_sub(1)) }
    }

    /// Returns `true` if this size hint range overlaps with another size hint range.
    ///
    /// Two ranges overlap if there exists at least one value that could be contained in both.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::{SizeHint, InvalidSizeHint};
    /// # fn main() -> Result<(), InvalidSizeHint> {
    /// let overlapping1 = SizeHint::bounded(3, 6)?;
    /// let overlapping2 = SizeHint::bounded(5, 10)?;
    /// assert!(SizeHint::overlaps(overlapping1, overlapping2), "should overlap at 5 and 6");
    ///
    /// let exact_hint = SizeHint::exact(5);
    /// assert!(SizeHint::overlaps(exact_hint, SizeHint::UNIVERSAL), "should overlap with any size hint");
    ///
    /// let unbounded_hint = SizeHint::unbounded(11);
    /// assert!(!SizeHint::overlaps(unbounded_hint, exact_hint), "should not overlap");
    /// # Ok(())
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        match ((self.lower, self.upper), (other.lower, other.upper)) {
            ((a_lower, Some(a_upper)), (b_lower, Some(b_upper))) => a_lower <= b_upper && b_lower <= a_upper,
            ((_, Some(a_upper)), (b_lower, None)) => a_upper >= b_lower,
            ((a_lower, None), (_, Some(b_upper))) => b_upper >= a_lower,
            ((_, None), (_, None)) => true,
        }
    }

    /// Returns `true` if this size hint range is completely contained within another range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::{SizeHint, InvalidSizeHint};
    /// # fn main() -> Result<(), InvalidSizeHint> {
    /// let subset = SizeHint::bounded(4, 6)?;
    /// let superset = SizeHint::bounded(3, 10)?;
    /// assert!(subset.is_subset_of(superset));
    ///
    /// let not_subset = SizeHint::bounded(2, 6)?;
    /// assert!(!not_subset.is_subset_of(superset));
    /// # Ok(())
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_subset_of(self, other: Self) -> bool {
        match ((self.lower, self.upper), (other.lower, other.upper)) {
            ((a_low, Some(a_up)), (b_low, Some(b_up))) => a_low >= b_low && a_up <= b_up,
            ((a_low, _), (b_low, None)) => a_low >= b_low,
            ((_, None), (_, Some(_))) => false,
        }
    }
}

impl TryFrom<(usize, Option<usize>)> for SizeHint {
    type Error = InvalidSizeHint;

    #[inline]
    fn try_from(hint: (usize, Option<usize>)) -> Result<Self, Self::Error> {
        match hint {
            (lower, Some(upper)) => Self::try_bounded(lower, upper),
            (lower, None) => Ok(Self::unbounded(lower)),
        }
    }
}

impl From<SizeHint> for (usize, Option<usize>) {
    #[inline]
    fn from(hint: SizeHint) -> Self {
        (hint.lower, hint.upper)
    }
}

impl TryFrom<Range<usize>> for SizeHint {
    type Error = InvalidSizeHint;

    #[inline]
    fn try_from(range: Range<usize>) -> Result<Self, Self::Error> {
        range.end.checked_sub(1).map_or(Err(InvalidSizeHint), |end| Self::try_bounded(range.start, end))
    }
}

impl TryFrom<RangeInclusive<usize>> for SizeHint {
    type Error = InvalidSizeHint;

    #[inline]
    fn try_from(range: RangeInclusive<usize>) -> Result<Self, Self::Error> {
        Self::try_bounded(*range.start(), *range.end())
    }
}

impl From<RangeFull> for SizeHint {
    #[inline]
    fn from(_: RangeFull) -> Self {
        Self::UNIVERSAL
    }
}

impl From<RangeFrom<usize>> for SizeHint {
    #[inline]
    fn from(range: RangeFrom<usize>) -> Self {
        Self { lower: range.start, upper: None }
    }
}

impl TryFrom<RangeTo<usize>> for SizeHint {
    type Error = InvalidSizeHint;

    #[inline]
    fn try_from(range: RangeTo<usize>) -> Result<Self, Self::Error> {
        range.end.checked_sub(1).map_or(Err(InvalidSizeHint), |end| Self::try_bounded(0, end))
    }
}

impl From<RangeToInclusive<usize>> for SizeHint {
    #[inline]
    fn from(range: RangeToInclusive<usize>) -> Self {
        Self { lower: 0, upper: Some(range.end) }
    }
}

/// A [`SizeHint`] represents a range of possible iterator lengths.
impl RangeBounds<usize> for SizeHint {
    /// Returns the smallest possible iterator length. Always [`Bound::Included`].
    #[inline]
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.lower)
    }

    /// Returns the largest possible iterator length. Either [`Bound::Included`] or [`Bound::Unbounded`].
    #[inline]
    fn end_bound(&self) -> Bound<&usize> {
        self.upper.as_ref().map_or(Bound::Unbounded, Bound::Included)
    }
}

impl PartialEq<(usize, Option<usize>)> for SizeHint {
    fn eq(&self, other: &(usize, Option<usize>)) -> bool {
        self.lower == other.0 && self.upper == other.1
    }
}

impl PartialEq<SizeHint> for (usize, Option<usize>) {
    fn eq(&self, other: &SizeHint) -> bool {
        self.0 == other.lower && self.1 == other.upper
    }
}
