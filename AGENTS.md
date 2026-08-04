# AGENTS.md

Guidelines for agents working on this repository.

## Project overview

`remend` is a pure Rust port of the [`remend`](https://www.npmjs.com/package/remend) npm package. It completes incomplete Markdown syntax during streaming output, for example `**bold text` becomes `**bold text**`.

The `streamdown/` git submodule contains the upstream TypeScript source and is the behavioral source of truth. The current port tracks upstream package version `1.3.0`.

## Upstream parity rule

Keep the Rust implementation behaviorally 1:1 with `streamdown/packages/remend/src/` and its tests. Do not add handlers, change priorities, rename options, or alter completion behavior unless the upstream implementation changes first.

The source layout intentionally follows upstream filenames and responsibility:

```text
streamdown/packages/remend/src/     src/
code-block-utils.ts              -> code-block-utils.rs
comparison-operator-handler.ts   -> comparison-operator-handler.rs
emphasis-handlers.ts             -> emphasis-handlers.rs
html-tag-handler.ts              -> html-tag-handler.rs
index.ts                         -> index.rs
inline-code-handler.ts           -> inline-code-handler.rs
katex-handler.ts                -> katex-handler.rs
link-image-handler.ts           -> link-image-handler.rs
patterns.ts                     -> patterns.rs
setext-heading-handler.ts       -> setext-heading-handler.rs
single-tilde-handler.ts         -> single-tilde-handler.rs
strikethrough-handler.ts        -> strikethrough-handler.rs
utils.ts                        -> utils.rs
```

`lib.rs` is the unavoidable Rust crate entrypoint and only wires those modules with `#[path]`. Integration tests use the corresponding upstream stems under `tests/` (`bold.rs`, `links.rs`, `underscore-bug.rs`, etc.). Keep new upstream files in the same location and use the same kebab-case filename.

Rust-specific adaptations are limited to ownership and UTF-8-safe string handling, `Option`/`Result` where required, traits for custom handlers, and `LinkMode` as an enum. Public utility positions are UTF-8 byte offsets and must be valid `str` boundaries.

## Public API

Exported from `remend`:

- `remend(&str) -> String`
- `remend_with_options(&str, &RemendOptions) -> String`
- `RemendHandler` and `FnRemendHandler`
- `RemendOptions`
- `LinkMode::{Protocol, TextOnly}`
- context utilities from `utils.rs`
- public emphasis counters and incomplete-emphasis handlers used by tests and custom integrations

Options default to enabled, except `inline_katex`, which is opt-in. Built-in priorities must remain: single tilde `0`, comparison operators `5`, HTML `10`, setext headings `15`, links `20`, bold-italic `30`, bold `35`, italic `40–42`, inline code `50`, strikethrough `60`, block KaTeX `70`, and inline KaTeX `75`. Custom handlers default to priority `100`.

## Upstream sync workflow

When the submodule advances:

1. Record the old submodule commit.
2. Run `git submodule update --remote streamdown`.
3. Compare changed files with `git diff <old-commit> HEAD -- streamdown/packages/remend/src/ streamdown/packages/remend/__tests__/`.
4. Port each changed TypeScript/test file to the matching Rust file, keeping comments, test cases, handler names, option names, and priorities aligned.
5. If `packages/remend/package.json` changes version, update `version` in `Cargo.toml` exactly.
6. Update `CHANGELOG.md` and any meaningful upstream documentation changes.
7. Run the complete Rust validation below.

Do not delete or replace the submodule; it is required for conformance review.

## Development commands

Run before every commit:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo doc --no-deps
cargo package --list
cargo publish --dry-run
git diff --check
```

The GitHub Actions CI runs formatting, Clippy, tests, documentation, and code coverage on pushes and pull requests to `main`. It uploads the LCOV coverage report as the `coverage-lcov` artifact without enforcing a threshold yet. The release workflow additionally checks the package and publishes version tags through crates.io Trusted Publishing.

## Commit style

Use lowercase conventional commits and do not add AI coauthor lines:

```text
feat:     new feature
fix:      bug fix
docs:     documentation only
chore:    maintenance (deps, config)
refactor: code change without behavior change
ci:       CI/CD changes
```
