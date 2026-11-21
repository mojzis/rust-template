# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

`rust-template` is a Rust CLI, packaged as a Python wheel via maturin
(`bindings = "bin"`) and published to PyPI. Replace this paragraph with what the
project does.

## Common Commands

```sh
# Pre-commit checks (always run before committing)
cargo fmt --all -- --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features

make review        # the above, plus rustdoc, audit and deny
make review-quick  # skip the network checks
make setup         # install the prek commit hook (once per clone)
```

If formatting fails, fix it with `cargo fmt --all` and re-run.

## Stack

clap (derive), anyhow, tracing; assert_cmd + predicates for CLI tests;
cargo-deny / cargo-audit for supply chain; prek for the commit hook;
cargo-release + maturin for releases.

## Development Workflow

All features and bug fixes follow TDD (red-green-refactor). No implementation
code without a failing test first. Bug fixes must include a regression test that
fails without the fix.

## Test Changes Require Deliberation

When a test fails during implementation:

1. **Stop and diagnose.** Understand WHY it fails before changing anything.
2. **Default assumption: the test is right.** Fix the implementation first.
3. **If the test genuinely needs updating**, explain what changed and why the
   old assertion is no longer correct before modifying it.
4. **Never weaken an assertion just to make it pass.**
5. **If uncertain, ask.**

## Key Invariants

Add to this list as the design settles — an invariant the code no longer
upholds is a bug in one of the two.

- **`main.rs` stays thin.** Argument parsing, tracing setup, exit-code mapping.
  Command bodies belong in `cli.rs`; logic in library modules.
- **Exit codes are part of the contract.** `0` success, `2` the run could not
  complete (clap also uses `2` for usage errors). If the tool grows a verdict
  ("findings"), give it `1` and never collapse it into `2`.
- **Logs go to stderr, output to stdout.** Output must stay pipeable with
  `--verbose` on.
- **No `unwrap`/`expect` outside tests** — clippy warns and `-D warnings` makes
  it fatal. Use `anyhow::Context` on every `?` that crosses an I/O, parsing or
  subprocess boundary; a bare `?` loses which operation failed.
- **Every test asserts on a value.** An integration test that asserts on stdout
  also asserts on the exit code.

## Releasing

`cargo release patch` (or `gh-monitor release`) bumps `Cargo.toml`, commits,
tags `vX.Y.Z` and pushes. The tag triggers `release.yml`: wheels for
linux/macos/windows + sdist → GitHub Release → PyPI (trusted publisher,
environment `pypi`). `publish = false` in `release.toml` — nothing goes to
crates.io. Release workflows are skipped while the repo is still marked as a
template.

## Release notes

A change a user would notice adds one file to `release-notes/` in the same PR
(`YYYY-MM-DD-short-slug.md`, frontmatter `kind: added|changed|fixed|removed`, one or two
plain sentences). Run `uvx --from git+https://github.com/mojzis/relnotes relnotes guide` before writing one: it has the wording rules.
Dependency bumps, CI, refactors and tests get no note. `uvx --from git+https://github.com/mojzis/relnotes relnotes check` validates them.
The release workflow turns them into the GitHub release, and the aesop site shows them.
