use core::iter::FusedIterator;

/// A [`FusedIterator`] adaptor that provides an exact length via [`ExactSizeIterator`].
///
/// This is useful for iterators that don't normally implement [`ExactSizeIterator`]
/// but where you know the exact length. And may allow some performance optimizations.
///
/// However, it may lead to performance penalties if the wrapped iterator already
/// implements `TrustedLen`, even if it does not implement [`ExactSizeIterator`].
/// For example, [`std::iter::Chain`]. Since this adaptor hides that implementation.
///
/// Implemented in terms of [`FusedIterator`], because meaningful a implementation
/// of [`ExactSizeIterator`] is not possible after the wrapped iterator returns [`None`].
///
/// Note that this type is readonly. Fields maybe be read, but not modified.
///
/// # Safety
///
/// `ExactLen` should be *safe* to use in any scenario, regardless of the values
/// it returns. It is the caller's responsibility to ensure that the length
/// provided are accurate. Providing an incorrect length may lead to incorrect behavior
/// or panics in code that relies on these values.
///
/// # Examples
///
/// ```rust
/// use size_hinter::ExactLen;
///
/// let odd_numbers = (1..=5).filter(|x| x % 2 == 1);
/// let mut three_odds = ExactLen::new(odd_numbers, 3);
///
/// assert_eq!(three_odds.len(), 3, "len should match the initial length");
/// assert_eq!(three_odds.size_hint(), (3, Some(3)), "size_hint should match the len");
///
/// assert_eq!(three_odds.next(), Some(1), "The underlying iterator is unchanged");
/// assert_eq!(three_odds.len(), 2, "len should match the remaining length");
/// assert_eq!(three_odds.size_hint(), (2, Some(2)), "size_hint should match len");
/// ```
#[derive(Debug, Clone)]
#[readonly::make]
pub struct ExactLen<I: FusedIterator> {
    /// The underlying iterator.
    pub iterator: I,
    /// The exact length of the iterator.
    pub len: usize,
}

impl<I: FusedIterator> ExactLen<I> {
    /// Wraps `iterator` with a new [`ExactSizeIterator::len`] implementation based on the
    /// provided `len` value.
    ///
    /// The caller must ensure that `len` accurately represents the number of elements remaining
    /// in the iterator. Providing an incorrect length may lead to unexpected behavior or panics in
    /// code that relies on [`ExactSizeIterator::len`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use size_hinter::ExactLen;
    ///
    /// let odd_numbers = (1..=5).filter(|x| x % 2 == 1);
    /// let mut three_odds = ExactLen::new(odd_numbers, 3);
    ///
    /// assert_eq!(three_odds.len(), 3, "len should match the initial length");
    /// assert_eq!(three_odds.size_hint(), (3, Some(3)), "size_hint should match the len");
    ///
    /// assert_eq!(three_odds.next(), Some(1), "The underlying iterator is unchanged");
    /// assert_eq!(three_odds.len(), 2, "len should match the remaining length");
    /// assert_eq!(three_odds.size_hint(), (2, Some(2)), "size_hint should match len");
    /// ```
    #[inline]
    pub fn new(iterator: impl IntoIterator<IntoIter = I>, len: usize) -> Self {
        Self { iterator: iterator.into_iter(), len }
    }

    /// Consumes the adaptor and returns the underlying iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use size_hinter::ExactLen;
    ///
    /// let iter: std::vec::IntoIter<i32> = vec![1, 2, 3].into_iter();
    /// let exact_iter = ExactLen::new(iter, 3);
    /// let inner: std::vec::IntoIter<i32> = exact_iter.into_inner();
    /// ```
    #[inline]
    pub fn into_inner(self) -> I {
        self.iterator
    }
}

impl<I: FusedIterator> Iterator for ExactLen<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            item @ Some(_) => {
                self.len = self.len.saturating_sub(1);
                item
            }
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<I: FusedIterator> ExactSizeIterator for ExactLen<I> {
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
}

impl<I: DoubleEndedIterator + FusedIterator> DoubleEndedIterator for ExactLen<I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.iterator.next_back() {
            Some(item) => {
                self.len = self.len.saturating_sub(1);
                Some(item)
            }
            None => None,
        }
    }
}

impl<I: FusedIterator> FusedIterator for ExactLen<I> {}
