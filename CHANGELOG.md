# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2023-08-25

### Changed
- Update dependencies of `anyhow`, `clap`, `flate2`, and `iso9660`.

### Fixed
- Point `cpio-archive` dependency to the upstream repository. (Will need to update again when new release
  enters crates.io)

## [0.1.0] - 2023-02-02

### Added
- Extraction of firmware files from an iso file.
- Option to execute the updator after the extraction.

[0.1.0]: https://github.com/johnmave126/extract-samsung-firmware/releases/tag/v0.1.0
[0.1.1]: https://github.com/johnmave126/extract-samsung-firmware/releases/tag/v0.1.1
