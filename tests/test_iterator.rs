mod macros;

use size_hinter::TestIterator;

#[test]
fn new() {
    let iter = TestIterator::<()>::new((5, Some(10)));
    assert_eq!(iter.size_hint(), (5, Some(10)));
}

const EXACT_LEN: usize = 5;

#[test]
fn exact() {
    let iter = TestIterator::<()>::exact(EXACT_LEN);
    assert_eq!(iter.size_hint(), (EXACT_LEN, Some(EXACT_LEN)));
}

#[test]
fn exact_len() {
    let iter = TestIterator::<()>::exact(EXACT_LEN);
    assert_eq!(iter.len(), EXACT_LEN);
}

#[test]
fn invalid_size_hint_is_invalid() {
    let (lower, upper) = TestIterator::<()>::invalid().size_hint();
    assert!(lower > upper.unwrap(), "Size hint should be invalid");
}

mod panic {
    use super::*;

    macros::panics!(on_next, TestIterator::<()>::invalid().next(), "TestIterator is not iteratable");
    macros::panics!(on_next_back, TestIterator::<()>::invalid().next_back(), "TestIterator is not iteratable");
    macros::panics!(invalid_len, TestIterator::<()>::invalid().len(), "Inexact size hint");
}
