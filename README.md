# Remend

[![crates.io](https://img.shields.io/crates/v/remend.svg)](https://crates.io/crates/remend)
[![docs.rs](https://docs.rs/remend/badge.svg)](https://docs.rs/remend)

Remend completes incomplete Markdown syntax while text is being streamed. It is a small, dependency-free Rust crate for AI and other incremental Markdown pipelines.

Remend is a Rust port of the [`remend`](https://www.npmjs.com/package/remend) package maintained by the [Streamdown](https://github.com/vercel/streamdown) team. The `streamdown/` submodule is the behavioral source of truth; this repository keeps the same handler order, option names, priorities, and test coverage while using Rust's ownership, traits, and UTF-8-safe string APIs.

## Features

- Completes bold, italic, bold-italic, inline code, strikethrough, links, images, setext headings, and KaTeX delimiters.
- Escapes single tildes and comparison operators when Markdown would otherwise misinterpret them.
- Preserves code, math, HTML, nested-bracket, list, and streaming context.
- Supports custom handlers through a trait or the `FnRemendHandler` closure adapter.
- Uses no runtime dependencies.

## Installation

```toml
[dependencies]
remend = "1.3"
```

## Usage

```rust
use remend::{remend, remend_with_options, LinkMode, RemendOptions};

let completed = remend("This is **bold text");
assert_eq!(completed, "This is **bold text**");

let options = RemendOptions {
    link_mode: LinkMode::TextOnly,
    ..Default::default()
};
let completed_link = remend_with_options("Read [the docs](https://example", &options);
assert_eq!(completed_link, "Read the docs");
```

The input is plain `&str` and the result is an owned `String`, making the function suitable for each streaming update.

## Options and custom handlers

All completion options default to `true`, except `inline_katex`, which is opt-in because a single `$` may be currency. `LinkMode::Protocol` is the default and emits `streamdown:incomplete-link`; `LinkMode::TextOnly` removes the incomplete link markup.

```rust
use remend::{remend_with_options, FnRemendHandler, RemendOptions};

let joke = FnRemendHandler::with_priority("joke", 80, |text: &str| {
    if text.contains("<<<JOKE>>>") && !text.ends_with("<<</JOKE>>>") {
        format!("{text}<<</JOKE>>>")
    } else {
        text.to_owned()
    }
});

let options = RemendOptions::default().with_handler(joke);
let completed = remend_with_options("<<<JOKE>>>hello", &options);
assert_eq!(completed, "<<<JOKE>>>hello<<</JOKE>>>");
```

Implement `RemendHandler` directly when a named type is more appropriate. Its `priority()` defaults to `100`; lower priorities run first. Built-in priorities are kept aligned with upstream: single tilde `0`, comparison operators `5`, HTML `10`, setext headings `15`, links `20`, bold-italic `30`, bold `35`, italic `40..42`, inline code `50`, strikethrough `60`, block KaTeX `70`, and inline KaTeX `75`.

The exported context helpers use UTF-8 byte offsets, matching Rust indexing conventions. Callers must pass a valid `str` boundary.

## Upstream-aligned layout

Each upstream TypeScript source file has a corresponding Rust file with the same kebab-case name under `src/`:

```text
streamdown/packages/remend/src/     src/
code-block-utils.ts              -> code-block-utils.rs
comparison-operator-handler.ts   -> comparison-operator-handler.rs
emphasis-handlers.ts             -> emphasis-handlers.rs
...
index.ts                         -> index.rs
```

`lib.rs` is only the Rust crate entrypoint and declares those files with `#[path]`. Integration tests use the same upstream stems (`bold.rs`, `links.rs`, `streaming.rs`, and so on). This makes an upstream diff easy to locate and port without introducing a second implementation structure.

## Development

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo doc --no-deps
```

Before releasing, also run `cargo package --list` and `cargo publish --dry-run`. The `streamdown` submodule remains checked in as the conformance reference. When it changes, compare the old and new submodule commits, port each changed source/test file with the same filename, update the crate version from `packages/remend/package.json`, and add a changelog entry.

## Code coverage

Install `cargo-llvm-cov` with the stable toolchain and generate an LCOV report locally:

```bash
cargo +stable install cargo-llvm-cov --locked
rustup component add llvm-tools-preview --toolchain stable
mkdir -p target/llvm-cov
cargo +stable llvm-cov --all-features --workspace --lcov --output-path target/llvm-cov/lcov.info
```

The CI runs the same coverage command and uploads `target/llvm-cov/lcov.info` as the `coverage-lcov` artifact. Coverage is reported but does not enforce a percentage threshold yet.

## Publishing

The release workflow publishes tags matching `vX.Y.Z` after running the same format, lint, test, documentation, and package checks. It uses crates.io Trusted Publishing through GitHub Actions OIDC; configure the repository and workflow as a trusted publisher in the crates.io package settings before the first automated release. The first package publication must be bootstrapped manually because crates.io cannot configure a trusted publisher for a package that does not exist yet.

## License and attribution

The Rust adaptation is distributed under the repository's MIT license. The upstream implementation and its Apache-2.0 attribution are recorded in [`NOTICE`](NOTICE).
