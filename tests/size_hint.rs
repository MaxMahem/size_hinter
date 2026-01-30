use std::ops::{Bound, RangeBounds};

use size_hinter::*;

/// Test constructor or conversion that should succeed
macro_rules! ctor {
    // Test any expression that returns Result, expecting success
    ($name:ident, $expr:expr => ok($exp_lower:expr, $exp_upper:expr)) => {
        #[test]
        fn $name() {
            let hint = $expr.expect("should succeed");
            assert_eq!(hint.lower, $exp_lower);
            assert_eq!(hint.upper, $exp_upper);
        }
    };
    // Test any expression that returns Result, expecting error
    ($name:ident, $expr:expr => err($exp_err:expr)) => {
        #[test]
        #[allow(clippy::reversed_empty_ranges)]
        fn $name() {
            let err = $expr.expect_err("should fail");
            assert_eq!(err, $exp_err);
        }
    };
    // Test ctor that does not fail
    ($name:ident, $expr:expr => ($exp_lower:expr, $exp_upper:expr)) => {
        #[test]
        fn $name() {
            let hint = $expr;
            assert_eq!(hint.lower, $exp_lower);
            assert_eq!(hint.upper, $exp_upper);
        }
    };
    ($name:ident, $expr:expr => panic $message:literal) => {
        #[test]
        #[should_panic(expected = $message)]
        fn $name() {
            _ = $expr;
        }
    };
}

/// Test binary operations between two SizeHints
/// Always tests both forward (hint1 op hint2) and reverse (hint2 op hint1) directions
macro_rules! binary_op {
    ($name:ident, $op:ident, $hint1:expr, $hint2:expr => $forward:expr, $reverse:expr) => {
        #[test]
        fn $name() {
            assert_eq!(SizeHint::$op($hint1, $hint2), $forward);
            assert_eq!(SizeHint::$op($hint2, $hint1), $reverse);
        }
    };
}

/// Test getter/transform operations on a SizeHint
macro_rules! transform {
    ($name:ident, $hint:expr, $method:ident() == $expected:expr) => {
        #[test]
        fn $name() {
            let hint = $hint;
            assert_eq!(hint.$method(), $expected);
        }
    };
}

mod ctor {
    use super::*;

    ctor!(new_valid, SizeHint::new(3, Some(10)) => (3, Some(10)));
    ctor!(new_invalid, SizeHint::new(10, Some(5)) => panic "values should describe a valid size hint");
    ctor!(try_bounded_valid, SizeHint::try_bounded(3, 10) => ok(3, Some(10)));
    ctor!(try_bounded_invalid, SizeHint::try_bounded(10, 5) => err(InvalidSizeHint));
    ctor!(bounded_valid, SizeHint::bounded(3, 10) => (3, Some(10)));
    ctor!(bounded_invalid, SizeHint::bounded(10, 5) => panic "values should describe a valid size hint");
    ctor!(default, SizeHint::default() => (0, None));
    ctor!(universal, SizeHint::UNIVERSAL => (0, None));
    ctor!(unbounded, SizeHint::unbounded(42) => (42, None));
    ctor!(at_most, SizeHint::at_most(42) => (0, Some(42)));
    ctor!(exact, SizeHint::exact(42) => (42, Some(42)));
}

mod try_from_tuple {
    use super::*;

    ctor!(valid, SizeHint::try_from((3, Some(7))) => ok(3, Some(7)));
    ctor!(unbounded, SizeHint::try_from((5, None)) => ok(5, None));
    ctor!(invalid, SizeHint::try_from((10, Some(5))) => err(InvalidSizeHint));
}

mod try_from_range {
    use super::*;

    ctor!(valid, SizeHint::try_from(3..8) => ok(3, Some(7)));
    ctor!(empty, SizeHint::try_from(5..5) => err(InvalidSizeHint));
    ctor!(empty_end, SizeHint::try_from(0..0) => err(InvalidSizeHint));
    ctor!(invalid, SizeHint::try_from(10..5) => err(InvalidSizeHint));
    ctor!(inclusive, SizeHint::try_from(3..=7) => ok(3, Some(7)));
    ctor!(inclusive_invalid, SizeHint::try_from(10..=5) => err(InvalidSizeHint));
    ctor!(full, SizeHint::from(..) => (0, None));
    ctor!(from, SizeHint::from(5..) => (5, None));
    ctor!(to, SizeHint::try_from(..8) => ok(0, Some(7)));
    ctor!(to_empty, SizeHint::try_from(..0) => err(InvalidSizeHint));
    ctor!(to_inclusive, SizeHint::from(..=7) => (0, Some(7)));
}

mod decrement {
    use super::*;

    transform!(normal, SizeHint::bounded(5, 10), decrement() == (4, Some(9)));
    transform!(saturating_lower, SizeHint::bounded(0, 5), decrement() == (0, Some(4)));
    transform!(saturating_both, SizeHint::ZERO, decrement() == SizeHint::ZERO);
    transform!(unbounded, SizeHint::unbounded(10), decrement() == (9, None));
    transform!(universal, SizeHint::UNIVERSAL, decrement() == SizeHint::UNIVERSAL);
}

mod properties {
    use super::*;

    transform!(size_hint, SizeHint::exact(7), as_hint() == (7, Some(7)));
    transform!(start, SizeHint::exact(7), start_bound() == Bound::Included(&7));
    transform!(end, SizeHint::exact(7), end_bound() == Bound::Included(&7));
    transform!(unbounded_end, SizeHint::unbounded(5), end_bound() == Bound::Unbounded);
}

mod overlaps {
    use super::*;

    binary_op!(partial, overlaps, SizeHint::bounded(3, 6), SizeHint::bounded(5, 10) => true, true);
    binary_op!(fully_contained, overlaps, SizeHint::bounded(4, 6), SizeHint::bounded(3, 10) => true, true);
    binary_op!(adjacent_no_overlap, overlaps, SizeHint::bounded(3, 6), SizeHint::bounded(7, 10) => false, false);
    binary_op!(touching_boundary, overlaps, SizeHint::bounded(3, 6), SizeHint::bounded(6, 10) => true, true);
    binary_op!(exact_same, overlaps, SizeHint::bounded(5, 5), SizeHint::bounded(5, 5) => true, true);
    binary_op!(unbounded_with_bounded, overlaps, SizeHint::unbounded(5), SizeHint::bounded(7, 10) => true, true);
    binary_op!(unbounded_no_overlap, overlaps, SizeHint::unbounded(15), SizeHint::bounded(3, 10) => false, false);
    binary_op!(both_unbounded, overlaps, SizeHint::unbounded(5), SizeHint::unbounded(10) => true, true);
}

mod disjoint {
    use super::*;

    binary_op!(partial_overlap, disjoint, SizeHint::bounded(3, 6), SizeHint::bounded(5, 10) => false, false);
    binary_op!(fully_contained, disjoint, SizeHint::bounded(4, 6), SizeHint::bounded(3, 10) => false, false);
    binary_op!(adjacent_no_overlap, disjoint, SizeHint::bounded(3, 6), SizeHint::bounded(7, 10) => true, true);
    binary_op!(touching_boundary, disjoint, SizeHint::bounded(3, 6), SizeHint::bounded(6, 10) => false, false);
    binary_op!(exact_same, disjoint, SizeHint::bounded(5, 5), SizeHint::bounded(5, 5) => false, false);
    binary_op!(unbounded_with_bounded, disjoint, SizeHint::unbounded(5), SizeHint::bounded(7, 10) => false, false);
    binary_op!(unbounded_no_overlap, disjoint, SizeHint::unbounded(15), SizeHint::bounded(3, 10) => true, true);
    binary_op!(both_unbounded, disjoint, SizeHint::unbounded(5), SizeHint::unbounded(10) => false, false);
}

mod subset_of {
    use super::*;

    binary_op!(partial, subset_of, SizeHint::bounded(4, 6), SizeHint::bounded(3, 10) => true, false);
    binary_op!(invalid_lower_too_small, subset_of, SizeHint::bounded(2, 6), SizeHint::bounded(3, 10) => false, false);
    binary_op!(invalid_upper_too_large, subset_of, SizeHint::bounded(4, 11), SizeHint::bounded(3, 10) => false, false);
    binary_op!(equal, subset_of, SizeHint::bounded(5, 10), SizeHint::bounded(5, 10) => true, true);
    binary_op!(bounded_in_unbounded, subset_of, SizeHint::bounded(5, 10), SizeHint::unbounded(3) => true, false);
    binary_op!(unbounded_not_in_bounded, subset_of, SizeHint::unbounded(5), SizeHint::bounded(3, 10) => false, false);
}

mod into_tuple {
    use super::*;

    #[test]
    fn bounded() {
        let hint = SizeHint::bounded(3, 7);
        let tuple: (usize, Option<usize>) = hint.into();
        assert_eq!(tuple, (3, Some(7)));
    }

    #[test]
    fn unbounded() {
        let hint = SizeHint::unbounded(5);
        let tuple: (usize, Option<usize>) = hint.into();
        assert_eq!(tuple, (5, None));
    }
}

mod partial_eq {
    use super::*;

    #[test]
    fn with_tuple() {
        let hint = SizeHint::bounded(3, 7);
        assert_eq!(hint, (3, Some(7)));
        assert_eq!((3, Some(7)), hint);
    }

    #[test]
    fn unbounded() {
        let hint = SizeHint::unbounded(5);
        assert_eq!(hint, (5, None));
        assert_eq!((5, None), hint);
    }
}

mod accessors {
    use super::*;

    transform!(lower_bounded, SizeHint::bounded(5, 10), lower() == 5);
    transform!(lower_unbounded, SizeHint::unbounded(5), lower() == 5);
    transform!(upper_bounded, SizeHint::bounded(5, 10), upper() == Some(10));
    transform!(upper_unbounded, SizeHint::unbounded(5), upper() == None);

    #[test]
    fn const_context() {
        const HINT: SizeHint = SizeHint::new(10, Some(20));
        const LOWER: usize = HINT.lower();
        const UPPER: Option<usize> = HINT.upper();
        assert_eq!(LOWER, 10);
        assert_eq!(UPPER, Some(20));
    }
}
