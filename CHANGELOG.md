# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0] - 2021-05-20
### Removed
- Access trait and volatile box array initialization are removed.

## [1.0.1] - 2021-05-20
### Changed
- Volatile boxes can now be initialized in constant expressions.
### Deprecated
- Access trait and volatile box array initialization are deprecated.

## [1.0.0] - 2021-05-19
### Added
- Volatile box abstraction.
### Removed
- Register traits and volatile cell abstraction.

## [0.1.0] - 2020-09-24
### Changed
- Register trait's volatile cell access API.

## [0.0.0] - 2020-09-21
### Added
- Volatile cell abstraction over memory locations suitable for MMIO.
- Trait for MMIO registers implemented with volatile cells.
- Traits for typesafe register reads and writes.

[2.0.0]: https://github.com/akiekintveld/mmio/releases/tag/2.0.0
[1.0.1]: https://github.com/akiekintveld/mmio/releases/tag/1.0.1
[1.0.0]: https://github.com/akiekintveld/mmio/releases/tag/1.0.0
[0.1.0]: https://github.com/akiekintveld/mmio/releases/tag/0.1.0
[0.0.0]: https://github.com/akiekintveld/mmio/releases/tag/0.0.0
