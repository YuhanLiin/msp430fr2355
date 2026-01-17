# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [v0.6.0]

- (Breaking) Regenerate with svd2rust 0.37.1
  - (Breaking) Peripheral names have changed from `SCREAMING_SNAKE` to `snake_case`, and type names have changed to `PascalCase`, dropping any underscores.
  - (Breaking) All registers are now accessed through methods. Previously registers were sometimes fields and sometimes methods (based on whether the hardware address was aliased by multiple registers).
- Add missing eUSCI UCBUSY bits

## [v0.5.2] - 2022-12-24

- Bump `portable-atomic` to v0.3.16 to reverse a previous regression that disabled single-instruction atomic operations

## [v0.5.1] - 2022-10-29

- Replace `msp430-atomic` with `portable-atomic` to fix non-MSP430 builds

## [v0.5.0] - 2022-10-26

- Regenerate all file using `svd2rust` 0.26
- Add CI pipeline
