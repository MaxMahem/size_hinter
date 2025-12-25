#[allow(unused_macros)]
mod common;

use common::*;

use std::ops::Range;

use size_hinter::{ExactLen, SizeHinter};

const TEST_ITER: Range<usize> = 1..5;
const TEST_LEN: usize = 4;

initial_state!(initial_hint, ExactLen::new(TEST_ITER, TEST_LEN), hint: (TEST_LEN, Some(TEST_LEN)));
initial_state!(initial_len, ExactLen::new(TEST_ITER, TEST_LEN), len: TEST_LEN);
initial_state!(len_too_small, TEST_ITER.exact_len(2), panic: "len should be within the wrapped iterator's size hint bounds: InvalidSizeHint");
initial_state!(len_too_large, TEST_ITER.exact_len(6), panic: "len should be within the wrapped iterator's size hint bounds: InvalidSizeHint");

iter_state!(
    forward_iteration,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next => Some(1), len: 3;
    next => Some(2), len: 2;
    next => Some(3), len: 1;
);

iter_state!(
    backward_iteration,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next_back => Some(4), len: 3;
    next_back => Some(3), len: 2;
    next_back => Some(2), len: 1;
);

iter_state!(
    forward_fused,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next => Some(1), len: 3;
    next => Some(2), len: 2;
    next => Some(3), len: 1;
    next => Some(4), len: 0;
    next => None::<usize>, len: 0;
    next => None::<usize>, len: 0;
);

iter_state!(
    backward_fused,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next_back => Some(4), len: 3;
    next_back => Some(3), len: 2;
    next_back => Some(2), len: 1;
    next_back => Some(1), len: 0;
    next_back => None::<usize>, len: 0;
    next_back => None::<usize>, len: 0;
);
