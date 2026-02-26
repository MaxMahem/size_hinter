mod macros;

use macros::*;

use std::ops::Range;

use size_hinter::*;

const TEST_ITER: Range<usize> = 1..5;

mod new_hint {
    use super::*;

    test_ctor!(valid, TEST_ITER.hint_size(3, 5) => hint: (3, Some(5)));
    test_ctor!(invalid_bounds, TEST_ITER.hint_size(5, 3) => panic: "Invalid size hint");
    test_ctor!(upper_too_small, TEST_ITER.hint_size(2, 2) => panic: "Invalid size hint");
    test_ctor!(lower_too_large, TEST_ITER.hint_size(6, 10) => panic: "Invalid size hint");
}

mod try_hint {
    use super::*;

    test_ctor!(valid, TEST_ITER.try_hint_size(3, 5).unwrap() => hint: (3, Some(5)));
    test_ctor!(invalid_bounds, TEST_ITER.try_hint_size(5, 3) => Err);
    test_ctor!(upper_too_small, TEST_ITER.try_hint_size(2, 2) => Err);
    test_ctor!(lower_too_large, TEST_ITER.try_hint_size(6, 10) => Err);
}

mod min {
    use super::*;

    test_ctor!(valid, TEST_ITER.hint_min(2) => hint: (2, None));
    test_ctor!(lower_too_large, TEST_ITER.hint_min(6) => panic: "Invalid size hint");
}

mod try_min {
    use super::*;

    test_ctor!(valid, TEST_ITER.try_hint_min(2).unwrap() => hint: (2, None::<usize>));
    test_ctor!(lower_too_large, TEST_ITER.try_hint_min(6) => Err);
}

test_ctor!(hidden, TEST_ITER.hide_size() => hint: SizeHint::UNIVERSAL);
test_ctor!(default, HintSize::<Range<usize>>::default() => hint: SizeHint::UNIVERSAL);

mod panic_on_invalid {
    use super::*;
    use size_hinter::INVALID_UNIT_ITERATOR;

    test_ctor!(new, HintSize::new(INVALID_UNIT_ITERATOR, 1, 2) => panic: "iterator's size hint should be valid");
    test_ctor!(try_new, HintSize::try_new(INVALID_UNIT_ITERATOR, 1, 2) => panic: "iterator's size hint should be valid");
    test_ctor!(min, HintSize::min(INVALID_UNIT_ITERATOR, 1) => panic: "iterator's size hint should be valid");
    test_ctor!(try_min, HintSize::try_min(INVALID_UNIT_ITERATOR, 1) => panic: "iterator's size hint should be valid");
}

mod iter {
    use super::*;

    test_iter!(
        forward,
        TEST_ITER.hint_size(4, 6) => hint: (4, Some(6)),
        next => Some(1), hint: (3, Some(5));
        next => Some(2), hint: (2, Some(4));
        next => Some(3), hint: (1, Some(3));
    );

    test_iter!(
        backward,
        TEST_ITER.hint_size(4, 6) => hint: (4, Some(6)),
        next_back => Some(4), hint: (3, Some(5));
        next_back => Some(3), hint: (2, Some(4));
        next_back => Some(2), hint: (1, Some(3));
    );

    test_iter!(
        unbounded_upper,
        TEST_ITER.hide_size() => hint: SizeHint::UNIVERSAL,
        next => Some(1), hint: SizeHint::UNIVERSAL;
        next => Some(2), hint: SizeHint::UNIVERSAL;
        next => Some(3), hint: SizeHint::UNIVERSAL;
        next => Some(4), hint: SizeHint::UNIVERSAL;
        next => None::<usize>, hint: SizeHint::UNIVERSAL;
        next => None::<usize>, hint: SizeHint::UNIVERSAL;
    );

    test_iter!(
        saturating_decrement,
        TEST_ITER.hint_size(2, 5) => hint: (2, Some(5)),
        next => Some(1), hint: (1, Some(4));
        next => Some(2), hint: (0, Some(3));
        next => Some(3), hint: (0, Some(2));
        next => Some(4), hint: (0, Some(1));
        next => None::<usize>, hint: (0, Some(0));
        next => None::<usize>, hint: (0, Some(0));
    );

    test_iter!(
        forward_fused,
        TEST_ITER.hint_size(4, 4) => hint: (4, Some(4)),
        next => Some(1), hint: (3, Some(3));
        next => Some(2), hint: (2, Some(2));
        next => Some(3), hint: (1, Some(1));
        next => Some(4), hint: (0, Some(0));
        next => None::<usize>, hint: (0, Some(0));
        next => None::<usize>, hint: (0, Some(0));
    );

    test_iter!(
        backward_fused,
        TEST_ITER.hint_size(4, 4) => hint: (4, Some(4)),
        next_back => Some(4), hint: (3, Some(3));
        next_back => Some(3), hint: (2, Some(2));
        next_back => Some(2), hint: (1, Some(1));
        next_back => Some(1), hint: (0, Some(0));
        next_back => None::<usize>, hint: (0, Some(0));
        next_back => None::<usize>, hint: (0, Some(0));
    );
}
