#[allow(unused_macros)]
mod common;

use common::*;

use std::ops::Range;

use size_hinter::{ExactLen, SizeHinter};

const TEST_ITER: Range<usize> = 1..5;
const TEST_LEN: usize = 4;

test_len!(len_basic, ExactLen::new(TEST_ITER, TEST_LEN), TEST_LEN);
test_size_hint!(size_hint_basic, ExactLen::new(TEST_ITER, TEST_LEN), (TEST_LEN, Some(TEST_LEN)));

test_iter_hint_state!(
    forward_iteration,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next => Some(1), len: 3;
    next => Some(2), len: 2;
    next => Some(3), len: 1
);

test_iter_hint_state!(
    backward_iteration,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next_back => Some(4), len: 3;
    next_back => Some(3), len: 2;
    next_back => Some(2), len: 1
);

test_iter_hint_state!(
    forward_fused,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next => Some(1), len: 3;
    next => Some(2), len: 2;
    next => Some(3), len: 1;
    next => Some(4), len: 0;
    next => None::<usize>, len: 0;
    next => None::<usize>, len: 0
);

test_iter_hint_state!(
    backward_fused,
    TEST_ITER.exact_len(TEST_LEN) => len: TEST_LEN,
    next_back => Some(4), len: 3;
    next_back => Some(3), len: 2;
    next_back => Some(2), len: 1;
    next_back => Some(1), len: 0;
    next_back => None::<usize>, len: 0;
    next_back => None::<usize>, len: 0
);
