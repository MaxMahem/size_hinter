#[allow(unused_macros)]
mod macros;

use macros::*;

use std::ops::Range;

use size_hinter::{ExactLen, SizeHinter};

const TEST_ITER: Range<usize> = 1..5;
const TEST_LEN: usize = 4;

test_ctor!(initial_hint, ExactLen::new(TEST_ITER, TEST_LEN) => hint: (TEST_LEN, Some(TEST_LEN)));
test_ctor!(initial_len, ExactLen::new(TEST_ITER, TEST_LEN) => len: TEST_LEN);
test_ctor!(len_too_small, TEST_ITER.exact_len(2) => panic: "len should be within the wrapped iterator's size hint bounds: InvalidSizeHint");
test_ctor!(len_too_large, TEST_ITER.exact_len(6) => panic: "len should be within the wrapped iterator's size hint bounds: InvalidSizeHint");
test_ctor!(len_too_small_err, TEST_ITER.try_exact_len(2) => Err);
test_ctor!(len_too_large_err, TEST_ITER.try_exact_len(6) => Err);

mod panic_on_invalid {
    use super::*;
    use size_hinter::INVALID_UNIT_ITERATOR;

    test_ctor!(new, ExactLen::new(INVALID_UNIT_ITERATOR, 1) => panic: "wrapped iterator size_hint should be valid");
    test_ctor!(try_new, ExactLen::try_new(INVALID_UNIT_ITERATOR, 1) => panic: "wrapped iterator size_hint should be valid");
}

test_iter!(
    forward_iteration,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next => Some(1), len: 3;
    next => Some(2), len: 2;
    next => Some(3), len: 1;
);

test_iter!(
    backward_iteration,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next_back => Some(4), len: 3;
    next_back => Some(3), len: 2;
    next_back => Some(2), len: 1;
);

test_iter!(
    forward_fused,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next => Some(1), len: 3;
    next => Some(2), len: 2;
    next => Some(3), len: 1;
    next => Some(4), len: 0;
    next => None::<usize>, len: 0;
    next => None::<usize>, len: 0;
);

test_iter!(
    backward_fused,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next_back => Some(4), len: 3;
    next_back => Some(3), len: 2;
    next_back => Some(2), len: 1;
    next_back => Some(1), len: 0;
    next_back => None::<usize>, len: 0;
    next_back => None::<usize>, len: 0;
);
