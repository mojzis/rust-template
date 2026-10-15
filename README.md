# rust-template

A Rust CLI project template: strict clippy, cargo-deny, CI, and a tag-driven
release that ships the binary as a Python wheel (via maturin) to PyPI and GitHub
Releases.

## Using the template

```sh
gh repo create <name> --template mojzis/rust-template --public --clone
cd <name>
```

Then rename `rust-template` / `rust_template` to the new project name:

```sh
grep -rl --exclude-dir=target --exclude-dir=.git -e rust-template -e rust_template . \
  | xargs sed -i -e "s/rust-template/<name>/g" -e "s/rust_template/<name_snake>/g"
make setup        # install the prek commit hook
make review-quick
```

Set up a [PyPI trusted publisher](https://docs.pypi.org/trusted-publishers/)
(workflow `release.yml`, environment `pypi`) before the first release.

## What's included

| Area | Files |
| --- | --- |
| Crate | `src/main.rs` (thin entry, exit codes), `src/cli.rs` (clap), `src/lib.rs`, `tests/cli.rs` (assert_cmd) |
| Lints | `[lints]` in `Cargo.toml` (pedantic + nursery + selected restriction), `clippy.toml`, `rustfmt.toml` |
| Toolchain | `rust-toolchain.toml` (pinned) |
| Supply chain | `deny.toml`, `audit.yml`, `cargo-deny.yml`, `codeql.yml`, `dependabot.yml` |
| CI | `review.yml` — fmt, clippy `-D warnings`, tests, rustdoc `-D warnings` |
| Release | `release.toml` (cargo-release), `release.yml` (wheels for linux/macos/windows → GitHub Release + PyPI) |
| Commit hook | `prek.toml` — fmt, clippy, tests on commits touching Rust |
| Claude Code | `CLAUDE.md`, `.claude/skills/rust-review` |

## Commands

```sh
make review-quick   # fmt-check, clippy, test, rustdoc
make review         # + cargo audit, cargo deny
make coverage       # cargo-llvm-cov HTML report
make mutants        # cargo-mutants on the last commit
make wheel          # maturin build --release
cargo release patch # bump, tag vX.Y.Z, push → triggers release.yml
```

## License

MIT
