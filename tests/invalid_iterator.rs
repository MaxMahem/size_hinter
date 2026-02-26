mod macros;

use size_hinter::InvalidIterator;

macros::panics!(panics_on_next, InvalidIterator::<()>::new().next(), "InvalidIterator is not iteratable");
macros::panics!(panics_on_next_back, InvalidIterator::<()>::new().next_back(), "InvalidIterator is not iteratable");
macros::panics!(panics_on_len, InvalidIterator::<()>::new().len(), "InvalidIterator does not have a valid len");

#[test]
fn reports_invalid_size_hint() {
    let iter = InvalidIterator::<()>::new();
    let (lower, upper) = iter.size_hint();
    assert!(lower > upper.unwrap(), "Size hint should be invalid");
}
