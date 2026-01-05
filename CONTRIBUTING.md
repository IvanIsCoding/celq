# Contributing

## Building

`celq` is built with Rust. To contribute, you'll need to have `cargo` installed. If you don't, check
[rustup](https://rustup.rs/) for the most popular way of installing.

Once you have `cargo`, simply run:

```bash
cargo build
```

To use the local binary, `cargo run -- <ARGS>` is your friend.

## Bug Fixes and Features

If `celq` has a bug or is missing a feature, please:
1. Open an issue
2. Discuss the proposed bug fix/ feature design
3. Send a PR with tests

## Tests

To run the tests:

```bash
cargo test
```

`celq` has two kind of tests: unit tests and integration tests.

Unit tests live very close to the implementation. Next to the `mod.rs`, there's generally a `module_test.rs`. It's fine to use unit tests for smaller details.

Integration tests live in the `test/` folder, mostly in `tests/golden.rs`. When fixing a bug or adding a feature, please try to add a test covering multiple combinations to that file.

## Documentation

The `celq` manual lives in docs.rs. To build it locally, run:

```bash
cargo doc --open --no-deps
```

Despite `celq` being a binary, docs.rs reads from `src/lib.rs`. Our current documentation lives in the `docs/` folder, so `src/lib.rs` should exclusively include the markdown from the docs folder.

## Packaging

Packaging `celq` for Linux, macOS, and Windows is a welcome contribution. If you add `celq` to a packaging repository, feel free to send a Pull Request updating the README to list that installation method.

## MSRV

The Minimum Supported Rust Version (MSRV) of `celq` trys to align with the MSRV of Debian stable. To see their current MSRV, search for [rustc](https://packages.debian.org/search?keywords=rustc) and see the version listed for testing.

Notice that eventually that may lag behind the latest of rustc by a considerable margin. However, that guarantees that `celq` can be packaged by many Linux distributions.