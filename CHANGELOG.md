# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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