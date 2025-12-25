#[allow(unused_macros, unused_imports)]
mod common;

use common::*;

use std::ops::Range;

use size_hinter::*;

const TEST_ITER: Range<usize> = 1..5;

initial_state!(basic_hint, TEST_ITER.hint_size(3, 5), hint: (3, Some(5)));
initial_state!(min_hint, TEST_ITER.hint_min(2), hint: (2, None));
initial_state!(hidden_hint, TEST_ITER.hide_size(), hint: (0, None));
initial_state!(default_hint, HintSize::<Range<usize>>::default(), hint: SizeHint::UNIVERSAL);
initial_state!(invalid_bounds, TEST_ITER.hint_size(5, 3), panic: "Invalid size hint");
initial_state!(new_upper_too_small, TEST_ITER.hint_size(2, 2), panic: "Invalid size hint");
initial_state!(new_lower_too_large, TEST_ITER.hint_size(6, 10), panic: "Invalid size hint");
initial_state!(min_lower_too_large, TEST_ITER.hint_min(6), panic: "Invalid size hint");

iter_state!(
    forward_iter,
    TEST_ITER.hint_size(4, 6) => hint: (4, Some(6)),
    next => Some(1), hint: (3, Some(5));
    next => Some(2), hint: (2, Some(4));
    next => Some(3), hint: (1, Some(3));
);

iter_state!(
    backward_iter,
    TEST_ITER.hint_size(4, 6) => hint: (4, Some(6)),
    next_back => Some(4), hint: (3, Some(5));
    next_back => Some(3), hint: (2, Some(4));
    next_back => Some(2), hint: (1, Some(3));
);

iter_state!(
    unbounded_upper,
    TEST_ITER.hide_size() => hint: SizeHint::UNIVERSAL,
    next => Some(1), hint: SizeHint::UNIVERSAL;
    next => Some(2), hint: SizeHint::UNIVERSAL;
    next => Some(3), hint: SizeHint::UNIVERSAL;
    next => Some(4), hint: SizeHint::UNIVERSAL;
    next => None::<usize>, hint: SizeHint::UNIVERSAL;
    next => None::<usize>, hint: SizeHint::UNIVERSAL;
);

iter_state!(
    saturating_decrement,
    TEST_ITER.hint_size(2, 5) => hint: (2, Some(5)),
    next => Some(1), hint: (1, Some(4));
    next => Some(2), hint: (0, Some(3));
    next => Some(3), hint: (0, Some(2));
    next => Some(4), hint: (0, Some(1));
    next => None::<usize>, hint: (0, Some(1));
);

iter_state!(
    forward_fused,
    TEST_ITER.hint_size(4, 4) => hint: (4, Some(4)),
    next => Some(1), hint: (3, Some(3));
    next => Some(2), hint: (2, Some(2));
    next => Some(3), hint: (1, Some(1));
    next => Some(4), hint: (0, Some(0));
    next => None::<usize>, hint: (0, Some(0));
    next => None::<usize>, hint: (0, Some(0));
);

iter_state!(
    backward_fused,
    TEST_ITER.hint_size(4, 4) => hint: (4, Some(4)),
    next_back => Some(4), hint: (3, Some(3));
    next_back => Some(3), hint: (2, Some(2));
    next_back => Some(2), hint: (1, Some(1));
    next_back => Some(1), hint: (0, Some(0));
    next_back => None::<usize>, hint: (0, Some(0));
    next_back => None::<usize>, hint: (0, Some(0));
);
