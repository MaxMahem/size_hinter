mod macros;

use size_hinter::TestIterator;

#[test]
fn new() {
    let iter = TestIterator::<()>::new((5, Some(10)));
    assert_eq!(iter.size_hint(), (5, Some(10)));
}

macros::panics!(panics_on_next, TestIterator::<()>::invalid().next(), "TestIterator is not iteratable");
macros::panics!(panics_on_next_back, TestIterator::<()>::invalid().next_back(), "TestIterator is not iteratable");
macros::panics!(panics_on_len, TestIterator::<()>::invalid().len(), "TestIterator does not have a valid len");

#[test]
fn invalid_size_hint_is_invalid() {
    let (lower, upper) = TestIterator::<()>::invalid().size_hint();
    assert!(lower > upper.unwrap(), "Size hint should be invalid");
}
