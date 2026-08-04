## 1.3.0 (Rust)

- Provide a dependency-free Rust crate while preserving the upstream remend 1.3.0 handler contract and priorities.
- Keep the upstream kebab-case source and test layout to make future npm syncs direct and reviewable.
- Add a stable Rust API with `RemendOptions`, `LinkMode`, trait-based custom handlers, and a closure adapter.
- Add Cargo CI, docs.rs metadata, and tag-based crates.io publishing through Trusted Publishing.
- Add a reproducible `cargo-llvm-cov` CI job that uploads an LCOV coverage report as an artifact.
