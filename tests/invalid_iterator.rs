use size_hinter::InvalidIterator;

#[test]
#[should_panic(expected = "next called on InvalidIterator")]
fn panics_on_next() {
    let mut iter = InvalidIterator;
    iter.next();
}

#[test]
fn reports_invalid_size_hint() {
    let iter = InvalidIterator;
    let (lower, upper) = iter.size_hint();
    assert!(lower > upper.unwrap(), "Size hint should be invalid");
}
