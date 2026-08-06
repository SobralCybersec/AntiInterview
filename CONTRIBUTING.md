# Contributing

## Scope

Keep changes inspection-first and minimal. Preserve Windows and Linux behavior,
platform gates, configuration compatibility, and the existing
domain/application/infrastructure/presentation boundaries.

Read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md),
[docs/DEPENDENCIES.md](docs/DEPENDENCIES.md), [SECURITY.md](SECURITY.md), and
[PRIVACY.md](PRIVACY.md) before changing runtime behavior.

## Setup

1. Install the Rust toolchain declared in `rust-toolchain.toml`.
2. Check out the repository with `Cargo.lock`.
3. Install platform requirements needed by the feature under test.
4. Windows C++ hook builds require CMake, a Windows compiler, and Microsoft
   Detours discoverable by `hook/CMakeLists.txt`.

No JavaScript, Node, database, or backend service setup is required.

## Required checks

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo audit
cargo build --release
clang-format --dry-run --Werror hook/*.cpp hook/*.h
bash linux/test_anti-interview-wayland.sh
```

Run Windows-specific tests and the native Detours C++ build on Windows.
Use `cargo build --profile profiling` when collecting performance traces.

## Change rules

- Add a regression test for each fixed bug when the behavior is testable.
- Keep platform-specific dependencies and code behind `cfg` boundaries.
- Prefer the standard library and existing dependencies.
- Do not add architecture layers, design patterns, metrics, or services without
  a demonstrated requirement.
- Document configuration or privacy changes in the same pull request.
- Do not commit credentials, private logs, screenshots, build directories, or
  generated binaries.

Report security issues privately as described in [SECURITY.md](SECURITY.md).
