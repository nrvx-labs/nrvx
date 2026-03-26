# Changelog

All notable changes to this project are documented in this file.

The format follows Keep a Changelog and this project adheres to Semantic
Versioning.

## [Unreleased]

## [0.3.1] - 2026-04-10
### Added
- Stable disclosure predicate API for SDK consumers
- Example workflow demonstrating whale-tier gating
- CI matrix across stable and beta Rust toolchains

### Changed
- Prover commitment format moved to little-endian serialisation
- SDK types module reorganised; deprecated aliases removed

### Fixed
- Account sizing for disclosure metadata when using nested predicates
- Flaky unit test in ```commitment::tests::mix_preimage```

## [0.3.0] - 2026-03-18
### Added
- Token-2022 confidential transfer wrapper instructions
- Proof verification CPI from consumer programs
- TypeScript helpers for proof serialisation

## [0.2.0] - 2026-02-09
### Added
- Initial SP1-style prover skeleton
- Disclosure type registry and predicate builder
- SDK scaffolding with Jest tests

## [0.1.0] - 2025-12-20
### Added
- Workspace layout
- Anchor program entrypoint
- Placeholder state accounts and errors
