# `size_hinter`

[![Build](https://github.com/MaxMahem/size_hinter/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/size_hinter/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/size_hinter/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/size_hinter/size_hinter/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/size_hinter/status.svg)](https://deps.rs/repo/github/MaxMahem/size_hinter)
[![Crates.io](https://img.shields.io/crates/v/size_hinter)](https://crates.io/crates/size_hinter)
[![codecov](https://codecov.io/github/MaxMahem/size_hinter/graph/badge.svg?token=N5JJLLQ04L)](https://codecov.io/github/MaxMahem/size_hinter)
![GitHub License](https://img.shields.io/github/license/MaxMahem/size_hinter)

Iterator adaptors for overriding or specifying exact size hints in Rust.

## Overview

`size_hinter` provides two iterator adaptors, an extension trait, and a foundational type for working with iterator size hints.

- **`SizeHint`**: An immutable type representing a size hint with strong guarantees about bounds validity (`lower <= upper`), providing additional functionality and conversions.
- **`ExactLen`**: Wraps an iterator to provide an exact length via `ExactSizeIterator::len()` and a coresponding `Iterator::size_hint()`. This is useful when you know the exact length of an iterator that doesn't normally implement `ExactSizeIterator` (like `Filter`).
- **`HintSize`**: Wraps an `Iterator` in an adaptor that provides a custom `Iterator::size_hint()` implementation only. This is primarily useful for implementing a fixed universal size hint `(0, None)` for testing.
- **`TestIterator`**: An test iterator that can not be iterated over, but has an arbitrary size hint.
- **`InvalidIterator`**: An iterator that reports an invalid size hint `(lower > upper)`.
- **`SizeHinter`**: An extension trait for fluently creating these adaptors.

This crate is `no_std` compatible and contains no `unsafe` code.

## Installation

It's on [crates.io](https://crates.io/crates/size_hinter).

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

`HintSize` provides a custom `Iterator::size_hint()` implementation. This is primarily useful for implementing a fixed universal size hint `(0, None)` for testing, but any valid size hint can be provided.

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

### `SizeHint` - Working with Size Hints Directly

`SizeHint` is a type-safe wrapper around the standard iterator size hint tuple `(usize, Option<usize>)`. It provides strong guarantees about bound validity and offers additional functionality for working with size hints.

```rust
use size_hinter::SizeHint;

// Create a bounded size hint (min 2, max 10 elements)
let hint = SizeHint::bounded(2, 10);
assert_eq!(hint.lower, 2);
assert_eq!(hint.upper, Some(10));

// Create an exact size hint
let exact = SizeHint::exact(5);
assert_eq!(exact.lower, 5);
assert_eq!(exact.upper, Some(5));

// Create an unbounded size hint (at least 3 elements, no upper limit)
let unbounded = SizeHint::unbounded(3);
assert_eq!(unbounded.lower, 3);
assert_eq!(unbounded.upper, None);

// Check if two size hints overlap
assert!(SizeHint::bounded(3, 6).overlaps(SizeHint::bounded(5, 10)));
assert!(SizeHint::exact(5).disjoint(SizeHint::bounded(10, 20)));
```

## Adaptor Performance Considerations

Wrapping an iterator that does not provide a detailed size hint or implement `ExactSizeIterator` may allow for some optimizations or performance improvements. However, it may lead to performance penalties if the wrapped iterator already implements `TrustedLen`, even if it does not implement `ExactSizeIterator`. For example, `std::iter::Chain`. Since this adaptor hides that implementation.

## Safety

`ExactLen` and `HintSize` are always safe to use - they will never cause undefined behavior or memory unsafety, regardless of the values provided.

Both adaptors validate that provided hints/lengths are logical (lower bound <= upper bound) and don't contradict the wrapped iterator's stated bounds. An adaptor can provide a hint or length that introduces new information, such as a new lower bound that is higher than provided one, but cannot claim a new lower bound higher than the wrapped iterator's max bound (if present).

It is still the caller's responsibility to ensure that the provided hints/lengths are accurate. Inaccurate values may prevent optimizations or cause issues in code that relies on these values for allocation or other decisions.
