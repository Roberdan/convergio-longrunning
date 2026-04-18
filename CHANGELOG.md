# Changelog

## [0.4.5](https://github.com/Roberdan/convergio-longrunning/compare/v0.4.4...v0.4.5) (2026-04-18)


### Bug Fixes

* close TOCTOU races in budget propagation and reaper, log SSE serialization errors ([26b983b](https://github.com/Roberdan/convergio-longrunning/commit/26b983b57804694f41fd06ae41e049ea25d02154))
* **deps:** bump rustls-webpki 0.103.11 -&gt; 0.103.12 (RUSTSEC-2026-0099) ([cc4b371](https://github.com/Roberdan/convergio-longrunning/commit/cc4b371240c8d1b504d63b3dab77f0802708dd90))
* security and quality audit pass 2 ([e4509e9](https://github.com/Roberdan/convergio-longrunning/commit/e4509e9937fe4d3c22f49dbf47c91cb3e982296d))

## [0.4.4](https://github.com/Roberdan/convergio-longrunning/compare/v0.4.3...v0.4.4) (2026-04-13)


### Bug Fixes

* pass CARGO_REGISTRY_TOKEN to release workflow ([a5f6cf7](https://github.com/Roberdan/convergio-longrunning/commit/a5f6cf796038cd37745a4b790415a13300cc7fe7))

## [0.4.3](https://github.com/Roberdan/convergio-longrunning/compare/v0.4.2...v0.4.3) (2026-04-13)


### Bug Fixes

* add crates.io publishing metadata (description, repository) ([258bec5](https://github.com/Roberdan/convergio-longrunning/commit/258bec56bea6e714f0ecf7e99855d2bcc41c1bdb))

## [0.4.2](https://github.com/Roberdan/convergio-longrunning/compare/v0.4.1...v0.4.2) (2026-04-13)


### Bug Fixes

* fix malformed convergio-ipc dependency in Cargo.toml ([#8](https://github.com/Roberdan/convergio-longrunning/issues/8)) ([ae6c06d](https://github.com/Roberdan/convergio-longrunning/commit/ae6c06de2b8f556c00589b796c0b492d34561d57))

## [0.4.1](https://github.com/Roberdan/convergio-longrunning/compare/v0.4.0...v0.4.1) (2026-04-13)


### Bug Fixes

* **deps:** update convergio-ipc to v0.1.6 (SDK v0.1.9 aligned) ([59deca6](https://github.com/Roberdan/convergio-longrunning/commit/59deca6aaf84f8b4416e08b07c9d961ff3e40448))

## [0.4.0](https://github.com/Roberdan/convergio-longrunning/compare/v0.3.0...v0.4.0) (2026-04-13)


### ⚠ BREAKING CHANGES

* LongRunError enum has new InvalidInput variant.

### Features

* adapt convergio-longrunning for standalone repo ([45b2af9](https://github.com/Roberdan/convergio-longrunning/commit/45b2af9c8583eaf008cf2d855091dee6afc8254d))


### Bug Fixes

* **release:** use vX.Y.Z tag format (remove component) ([2443cf0](https://github.com/Roberdan/convergio-longrunning/commit/2443cf04bddfb7823253533a37b4d7218a362ac5))
* security audit — race conditions, input validation, error disclosure ([#2](https://github.com/Roberdan/convergio-longrunning/issues/2)) ([66e7e0e](https://github.com/Roberdan/convergio-longrunning/commit/66e7e0eb154e71aedfe25d6dbba09a5391fbc5ee))

## [0.3.0](https://github.com/Roberdan/convergio-longrunning/compare/convergio-longrunning-v0.2.0...convergio-longrunning-v0.3.0) (2026-04-12)


### ⚠ BREAKING CHANGES

* LongRunError enum has new InvalidInput variant.

### Features

* adapt convergio-longrunning for standalone repo ([45b2af9](https://github.com/Roberdan/convergio-longrunning/commit/45b2af9c8583eaf008cf2d855091dee6afc8254d))


### Bug Fixes

* security audit — race conditions, input validation, error disclosure ([#2](https://github.com/Roberdan/convergio-longrunning/issues/2)) ([66e7e0e](https://github.com/Roberdan/convergio-longrunning/commit/66e7e0eb154e71aedfe25d6dbba09a5391fbc5ee))

## 0.1.0 (Initial Release)

### Features

- Initial extraction from convergio monorepo
