# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-01-01

### Added

- `SizeHint` - newtype wrapper around `(usize, Option<usize>)` with validation and helper methods
- `InvalidSizeHint` - error type for invalid size hints
- `HintSize::try_bounded()` - fallible version of `new()` that returns `Result<Self, InvalidSizeHint>` instead of panicking
- `HintSize::try_min()` - fallible version of `min()` that returns `Result<Self, InvalidSizeHint>` instead of panicking
- `ExactLen::try_new()` - fallible version of `new()` that returns `Result<Self, InvalidSizeHint>` instead of panicking
- `SizeHinter::try_hint_size()` - fallible extension method that returns `Result<HintSize<Self>, InvalidSizeHint>` instead of panicking
- `SizeHinter::try_hint_min()` - fallible extension method that returns `Result<HintSize<Self>, InvalidSizeHint>` instead of panicking

### Changed

- **Breaking Change**: `HintSize::new()` renamed to `HintSize::bounded`
  - Migration: change all uses of `HintSize::new()` to `HintSize::bounded()`
- **Breaking Change**: `HintSize::bounded()`, `HintSize::min()`, and `ExactLen::new()` now validate to prevent provably false bounds
  - `HintSize::bounded()` panics if:
    - `lower > upper`
    - `upper` is less than the wrapped iterator's lower bound (can't claim a maximum less than the guaranteed minimum)
    - `lower` is greater than the wrapped iterator's upper bound (if present) (can't claim a minimum higher than the known maximum)
  - `HintSize::min()` panics if:
    - `lower` is greater than the wrapped iterator's upper bound (if present)
  - `ExactLen::new()` panics if:
    - `len` is less than the wrapped iterator's lower bound
    - `len` is greater than the wrapped iterator's upper bound (if present)
  - All adaptor panic of the wrapped iterator's initial size hint is invalid
  - Migration: Refactor designs that provide provably false claims.
- `HintSize` now wraps a `SizeHint` instead of a tuple

## [0.2.0] - 2025-12-23

### Added

- `no_std` support
- `forbid(unsafe_code)`
- `FusedIterator` implementation for `ExactLen`

### Changed

- **Breaking Change**: `HintSize::new()` now validates that `lower_bound <= upper_bound` and panics if invalid
  - Migration: refactor designs that intentionally provide invalid size hints.

## [0.1.0] - 2022-03-22

- Initial release
