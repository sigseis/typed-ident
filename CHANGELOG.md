# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Versions `0.0.x` are considered early alpha. Breaking changes may occur in any
release during this time, including patch releases.

## [Unreleased]

## [0.0.2] - 2026-09-18

### Added

- Added cased variants of `SnakeIdent`, `KebabIdent`, and `CamelIdent`.
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
