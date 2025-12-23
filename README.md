# `size_hinter`

[![Build](https://github.com/MaxMahem/size_hinter/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/size_hinter/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/size_hinter/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/size_hinter/size_hinter/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/size_hinter/status.svg)](https://deps.rs/repo/github/MaxMahem/size_hinter)
[![Crates.io](https://img.shields.io/crates/v/size_hinter)](https://crates.io/crates/size_hinter)
[![codecov](https://codecov.io/github/MaxMahem/size_hinter/graph/badge.svg?token=N5JJLLQ04L)](https://codecov.io/github/MaxMahem/size_hinter)
![GitHub License](https://img.shields.io/github/license/MaxMahem/size_hinter)

Iterator adaptors for overriding or specifying exact size hints in Rust.

## Overview

`size_hinter` provides an extension trait two iterator adaptors that allow you to control `size_hint()` and `len()` behavior of iterators, and a trait for fluently creating these adaptors.

- **`ExactLen`**: Wraps an iterator to provide an exact length via `ExactSizeIterator::len()` and a coresponding `Iterator::size_hint()`. This is useful when you know the exact length of an iterator that doesn't normally implement `ExactSizeIterator` (like `Filter`).
- **`HintSize`**: Wraps an `Iterator` in an adaptor that provides a custom `Iterator::size_hint()` implementation only. This is primarily useful for implementing a fixed universal size hint `(0, None)` for testing.
- **`SizeHinter`**: An extension trait for fluently creating these adaptors.

## Usage

### `ExactLen` - Adding Exact Length to Iterators

`ExactLen` provides an exact length via `ExactSizeIterator::len()` and a coresponding `Iterator::size_hint()`. This is useful when you know the exact length of an iterator that doesn't normally implement `ExactSizeIterator` (like `Filter`), and may allow for some performance optimizations.

```rust
use size_hinter::SizeHinter;

// Filter doesn't implement ExactSizeIterator, but we know there are 3 odd numbers
let mut nums = (1..=5).filter(|x| x % 2 == 1).exact_len(3);

assert_eq!(nums.len(), 3, "Length should be 3");
assert_eq!(nums.size_hint(), (3, Some(3)), "Size hint should match length");

assert_eq!(nums.next(), Some(1), "Underlying iterator should be unchanged");
assert_eq!(nums.len(), 2, "Length should change to match remaining elements");
assert_eq!(nums.size_hint(), (2, Some(2)), "Size hint should match length");

assert_eq!(nums.next_back(), Some(5), "DoubleEndedIterator::next_back should work as expected");
assert_eq!(nums.len(), 1, "Length should change to match remaining elements");
assert_eq!(nums.size_hint(), (1, Some(1)), "Size hint should match length");
```

### `HintSize` - Overrides Size Hints

`HintSize` provides a custom `Iterator::size_hint()` implementation only. This is primarily useful for implementing a fixed universal size hint `(0, None)` for testing, but any valid size hint can be provided.

```rust
use size_hinter::SizeHinter;

// Hide the size hint completely (returns (0, None))
let mut hidden = vec![1, 2, 3].into_iter().hide_size();
assert_eq!(hidden.size_hint(), (0, None), "Size hint is hidden");
assert_eq!(hidden.next(), Some(1), "Underlying iterator is not changed");
assert_eq!(hidden.size_hint(), (0, None), "Size hint remains hidden");

// Provide a custom size hint
let mut custom = vec![1, 2, 3].into_iter().hint_size(1, 10);
assert_eq!(custom.size_hint(), (1, Some(10)), "Size hint is set to (1, Some(10))");
assert_eq!(custom.next(), Some(1), "Underlying iterator is not changed");
assert_eq!(custom.size_hint(), (0, Some(9)), "Size hint is updated to (0, Some(9))");
```

## Performance Considerations

Wrapping an iterator that does not provide a detailed size hint or implement `ExactSizeIterator` may allow for some optimizations or performance improvements. However, it may lead to performance penalties if the wrapped iterator already implements `TrustedLen`, even if it does not implement `ExactSizeIterator`. For example, `std::iter::Chain`. Since this adaptor hides that implementation.

## Safety

Neither `ExactLen` nor `HintSize` should be *unsafe* to use in any scenario, regardless of the values they return. However it is the caller's responsibility to ensure that the length provided are accurate. Providing an incorrect length may lead to incorrect behavior or panics in code that relies on these values.
