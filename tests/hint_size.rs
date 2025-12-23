#[allow(unused_macros, unused_imports)]
mod common;

use common::*;

use std::ops::Range;

use size_hinter::{HintSize, SizeHinter, UNIVERSAL_SIZE_HINT};

const TEST_ITER: Range<usize> = 1..5;

test_size_hint!(basic_hint, TEST_ITER.hint_size(3, 5), (3, Some(5)));
test_size_hint!(min_hint, TEST_ITER.hint_min(2), (2, None));
test_size_hint!(hidden_hint, TEST_ITER.hide_size(), (0, None));
test_size_hint!(default_hint, HintSize::<Range<usize>>::default(), UNIVERSAL_SIZE_HINT);

test_iter_hint_state!(
    forward_iter,
    TEST_ITER.hint_size(4, 6) => hint: (4, Some(6)),
    next => Some(1), hint: (3, Some(5));
    next => Some(2), hint: (2, Some(4));
    next => Some(3), hint: (1, Some(3))
);

test_iter_hint_state!(
    backward_iter,
    TEST_ITER.hint_size(4, 6) => hint: (4, Some(6)),
    next_back => Some(4), hint: (3, Some(5));
    next_back => Some(3), hint: (2, Some(4));
    next_back => Some(2), hint: (1, Some(3))
);

test_iter_hint_state!(
    unbounded_upper,
    TEST_ITER.hide_size() => hint: UNIVERSAL_SIZE_HINT,
    next => Some(1), hint: UNIVERSAL_SIZE_HINT;
    next => Some(2), hint: UNIVERSAL_SIZE_HINT;
    next => Some(3), hint: UNIVERSAL_SIZE_HINT;
    next => Some(4), hint: UNIVERSAL_SIZE_HINT;
    next => None::<usize>, hint: UNIVERSAL_SIZE_HINT;
    next => None::<usize>, hint: UNIVERSAL_SIZE_HINT
);

test_iter_hint_state!(
    saturating_decrement,
    TEST_ITER.hint_size(2, 3) => hint: (2, Some(3)),
    next => Some(1), hint: (1, Some(2));
    next => Some(2), hint: (0, Some(1));
    next => Some(3), hint: (0, Some(0));
    next => Some(4), hint: (0, Some(0));
    next => None::<usize>, hint: (0, Some(0));
    next => None::<usize>, hint: (0, Some(0))
);

test_iter_hint_state!(
    forward_fused,
    TEST_ITER.hint_size(4, 4) => hint: (4, Some(4)),
    next => Some(1), hint: (3, Some(3));
    next => Some(2), hint: (2, Some(2));
    next => Some(3), hint: (1, Some(1));
    next => Some(4), hint: (0, Some(0));
    next => None::<usize>, hint: (0, Some(0));
    next => None::<usize>, hint: (0, Some(0))
);

test_iter_hint_state!(
    backward_fused,
    TEST_ITER.hint_size(4, 4) => hint: (4, Some(4)),
    next_back => Some(4), hint: (3, Some(3));
    next_back => Some(3), hint: (2, Some(2));
    next_back => Some(2), hint: (1, Some(1));
    next_back => Some(1), hint: (0, Some(0));
    next_back => None::<usize>, hint: (0, Some(0));
    next_back => None::<usize>, hint: (0, Some(0))
);
