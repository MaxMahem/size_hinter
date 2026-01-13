/// A [`Iterator`] that reports an invalid (empty) size hint, with lower bound > upper bound.
///
/// This is useful for testing how consumers handle invalid size hints.
///
/// This iterator panics when [`Self::next`] is called, and returns an invalid size hint when
/// [`Self::size_hint`] is called.
///
/// # Examples
///
/// ```rust
/// # use size_hinter::InvalidIterator;
/// let iter = InvalidIterator;
/// let (lower, upper) = iter.size_hint();
/// assert!(lower > upper.unwrap(), "Size hint should be invalid");
/// ```
#[derive(Debug, Default)]
pub struct InvalidIterator;

impl Iterator for InvalidIterator {
    type Item = ();

    /// Always panics.
    fn next(&mut self) -> Option<Self::Item> {
        panic!("next called on InvalidIterator")
    }

    /// Always returns an invalid size hint, with lower bound > upper bound.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (10, Some(5))
    }
}

impl core::iter::FusedIterator for InvalidIterator {}
