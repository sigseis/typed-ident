# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Versions `0.0.x` are considered early alpha. Breaking changes may occur in any
release during this time, including patch releases.

## [Unreleased]

### Added

- Added the ability to iterate over words of a fragment; `Words`, `StrWords`, `WordIndices`, `StrWordIndices`.
- Added `Owned*Segment` variant type aliases for presets.
- Added `TryFrom<String>` for owned types (such as `Box<Chunk>`, `Box<Fragment>`, and `Box<Ident>`).
- Loosened the argument requirements for `new` and `new_boxed`.

### Breaking Changes

- Changed `Fragment` to also return `Box<Fragment>` where appropriate (for consistency with `Ident`).
- Changed the result types for `To*` conversion traits to support various different result type bindings.
- Hid the `SliceIndex` type so that it cannot be named by the end-user.
- Removed `ConvertibleIdentifier` and `FormattableIdentifier`, now you can just use `Identifier`.
- Removed `UNICODE_*` constants other than `UNICODE_VERSION`, changed reported version calculation.
- Removed the `_opts` formatting variants from the `As*` and `To*` traits.
- Removed the `presets` feature flag (presets are now always enabled).
- Removed the plain-form formatter variants that were added in v0.0.2.
- Renamed `Chunk`'s `WordStrs` and `WordStrIndices` to `StrWords` and `StrWordIndices`.
- Renamed `into_owned` to `to_owned` to better match other similar methods/traits.
- The `alloc`, `core`, and `error` modules are now private, re-exporting their contents into their parents.

## [0.0.2] - 2026-09-18

### Added

- Added profile variants of `CasedCamelIdent`, `CasedHybridIdent`, `CasedKebabIdent`, `CasedSnakeIdent`.
- Added generated functions for titlecase and combining character mappings.
- Added plain-form formatter variants.

### Breaking Changes

- Removed `IdentBuf`.
- Changed existing `IdentBuf` usage to `FragmentBuf`.
- Removed `Ident` functions that returned `Option<&Ident>`.
- Removed or consolidated redundant `FragmentBuf` and `IdentBuf` function
  variants.

### Changed

- Updated and clarified the documentation.

## [0.0.1] - 2026-09-07

### Added

- Published the initial version of the crate.

[Unreleased]: https://github.com/sigseis/typed-ident/compare/v0.0.2...HEAD
[0.0.2]: https://github.com/sigseis/typed-ident/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/sigseis/typed-ident/releases/tag/v0.0.1
