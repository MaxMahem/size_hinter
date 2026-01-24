#[allow(unused_macros)]
mod macros;

use size_hinter::InvalidIterator;

macros::panics!(panics_on_next, InvalidIterator::<()>::new().next(), "next called on InvalidIterator");
macros::panics!(panics_on_next_back, InvalidIterator::<()>::new().next_back(), "next_back called on InvalidIterator");

#[test]
fn reports_invalid_size_hint() {
    let iter = InvalidIterator::<()>::new();
    let (lower, upper) = iter.size_hint();
    assert!(lower > upper.unwrap(), "Size hint should be invalid");
}
