use core::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Error type for reporting invalid size hints where the size hint would be empty or invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid size hint: values describe an invalid or empty range")]
pub struct InvalidSizeHint;

/// A size hint for an iterator.
///
/// This is an immutable wrapper around the standard iterator size hint tuple
/// `(usize, Option<usize>)`, providing strong gurantees about the bound values
/// (ie. `lower <= upper`), and additional functionality and conversions.
///
/// A size hint describes the range of possible values for the length of an iterator, that is, the
/// number of elements that it will return when enumerated to exhaustion from its current state.
/// While this count does not have to be *exact* (unless the size hint describes an exact bounds,
/// one where the upper and lower bounds are equal) it is an error for the iterator to then produce
/// a number of elements that violates the bounds established by the hint.
///
/// A size hint can never describe an empty range, as 0 is always a valid number of elements
/// remaining for an iterator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[readonly::make]
pub struct SizeHint {
    /// The inclusive lower bound of the size hint.
    pub lower: usize,
    /// The inclusive upper bound of the size hint, or `None` if the upper bound is unbounded.
    pub upper: Option<usize>,
}

impl SizeHint {
    /// A size hint that is always valid, never changes, and conveys no information.
    pub const UNIVERSAL: Self = Self { lower: 0, upper: None };

    /// A size hint that indicates that the iterator will yield no elements.
    pub const ZERO: Self = Self { lower: 0, upper: Some(0) };

    /// Creates a new size hint with the given lower and optional upper bounds.
    ///
    /// # Panics
    ///
    /// Panics if the upper bound is present and less than the lower bound.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::SizeHint;
    /// let hint = SizeHint::new(5, Some(10));
    /// assert_eq!(hint.lower, 5);
    /// assert_eq!(hint.upper, Some(10));
    ///```
    #[inline]
    #[must_use]
    pub const fn new(lower: usize, upper: Option<usize>) -> Self {
        match Self::try_new(lower, upper) {
            Ok(hint) => hint,
            Err(_) => panic!("values should describe a valid size hint"),
        }
    }

    /// Tries to create a new size hint with the given lower and optional upper bounds.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidSizeHint`] if the upper bound is present and less than the lower bound.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::{SizeHint, InvalidSizeHint};
    /// # fn main() -> Result<(), InvalidSizeHint> {
    /// let hint = SizeHint::try_new(5, Some(10))?;
    /// assert_eq!(hint.lower, 5);
    /// assert_eq!(hint.upper, Some(10));
    ///
    /// let err: InvalidSizeHint = SizeHint::try_new(10, Some(5)).expect_err("Lower bound is greater than upper bound");
    /// # Ok(())
    /// # }
    ///```
    #[inline]
    pub const fn try_new(lower: usize, upper: Option<usize>) -> Result<Self, InvalidSizeHint> {
        match (lower, upper) {
            (lower, Some(upper)) if lower > upper => Err(InvalidSizeHint),
            _ => Ok(Self { lower, upper }),
        }
    }

    /// Creates a new size hint with the given lower and optional upper bounds.
    ///
    /// # Panics
    ///
    /// Panics if `lower` is greater than `upper`.
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
    pub const fn bounded(lower: usize, upper: usize) -> Self {
        match Self::try_bounded(lower, upper) {
            Ok(hint) => hint,
            Err(_) => panic!("values should describe a valid size hint"),
        }
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
    /// let hint = SizeHint::try_bounded(5, 10)?;
    /// assert_eq!(hint.lower, 5);
    /// assert_eq!(hint.upper, Some(10));
    ///
    /// let err: InvalidSizeHint = SizeHint::try_bounded(10, 5).expect_err("SizeHint should be invalid");
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
    pub const fn as_hint(self) -> (usize, Option<usize>) {
        (self.lower, self.upper)
    }

    /// Returns a new [`SizeHint`] with the lower and upper bounds (if present) decremented by 1.
    ///
    /// This is useful for decrementing the size hint of an iterator after it has been advanced.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::SizeHint;
    /// let hint = SizeHint::bounded(5, 10);
    /// assert_eq!(hint.decrement(), SizeHint::bounded(4, 9));
    /// ```
    #[inline]
    #[must_use]
    pub fn decrement(self) -> Self {
        Self { lower: self.lower.saturating_sub(1), upper: self.upper.map(|upper| upper.saturating_sub(1)) }
    }

    /// Returns `true` if this size hint range overlaps with another size hint range.
    ///
    /// Two ranges overlap if there exists at least one value that could be contained in both.
    /// This operation is the negation of [`Self::disjoint`], and is also commutative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::SizeHint;
    /// assert!(SizeHint::overlaps(SizeHint::bounded(3, 6), SizeHint::bounded(5, 10)), "should overlap at 5 and 6");
    /// assert!(SizeHint::overlaps(SizeHint::exact(5), SizeHint::UNIVERSAL), "Universal should overlap with any size hint");
    /// assert!(!SizeHint::overlaps(SizeHint::unbounded(11), SizeHint::exact(5)), "should not overlap");
    /// assert!(SizeHint::overlaps(SizeHint::unbounded(11), SizeHint::UNIVERSAL), "two unbounded hints should overlap");
    /// ```
    #[inline]
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        match (self.as_hint(), other.as_hint()) {
            ((a_lower, Some(a_upper)), (b_lower, Some(b_upper))) => a_lower <= b_upper && b_lower <= a_upper,
            ((_, Some(a_upper)), (b_lower, None)) => a_upper >= b_lower,
            ((a_lower, None), (_, Some(b_upper))) => b_upper >= a_lower,
            ((_, None), (_, None)) => true,
        }
    }

    /// Returns `true` if this size hint range is disjoint with another range.
    ///
    /// Two ranges are disjoint if there exists no value that could be contained in both.
    /// This operation is the negation of [`Self::overlaps`], and is also commutative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::SizeHint;
    /// assert!(SizeHint::disjoint(SizeHint::exact(5), SizeHint::unbounded(10)), "should be disjoint");
    /// assert!(SizeHint::disjoint(SizeHint::exact(5), SizeHint::bounded(6, 10)), "should be disjoint");
    /// assert!(!SizeHint::disjoint(SizeHint::unbounded(11), SizeHint::UNIVERSAL), "two unbounded hints should never be disjoint");
    /// ```
    #[inline]
    #[must_use]
    pub const fn disjoint(self, other: Self) -> bool {
        !Self::overlaps(self, other)
    }

    /// Returns `true` if this size hint range is completely contained within another range.
    ///
    /// This operation is not commutative, i.e. `a.subset_of(b)` does not imply `b.subset_of(a)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use size_hinter::SizeHint;
    /// assert!(SizeHint::bounded(4, 6).subset_of(SizeHint::bounded(4, 6)), "should be a subset (equal ranges)");
    /// assert!(SizeHint::bounded(4, 6).subset_of(SizeHint::bounded(3, 9)), "should be a subset");
    /// assert!(!SizeHint::bounded(3, 9).subset_of(SizeHint::bounded(4, 6)), "should not be a subset (not commutative)");
    /// assert!(!SizeHint::bounded(2, 6).subset_of(SizeHint::bounded(3, 6)), "should not be a subset (lower bound)");
    /// assert!(!SizeHint::bounded(4, 7).subset_of(SizeHint::bounded(3, 6)), "should not be a subset (upper bound)");
    /// ```
    #[inline]
    #[must_use]
    pub const fn subset_of(self, other: Self) -> bool {
        match (self.as_hint(), other.as_hint()) {
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
        let end = range.end.checked_sub(1).ok_or(InvalidSizeHint)?;
        Self::try_bounded(range.start, end)
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
        let end = range.end.checked_sub(1).ok_or(InvalidSizeHint)?;
        Self::try_bounded(0, end)
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
