Changelog
=========

[v0.7.0](https://github.com/IvanIsCoding/celq/releases/tag/v0.7.0) - 2026-09-17
------------------------------------------------------------------------

### Added

* Added support for JSON-encoded lists and maps as `--arg` values.
* Release archives are now signed with minisign. The installer can verify signatures with `--verify-minisign`, and `cargo-binstall` can verify them with `cargo binstall celq --only-signed`.
* Release archives for several targets now use zstd compression. If you use Homebrew, `cargo-binstall`, or have `zstd` available, you will be able to download smaller files to install celq
* Significantly improved the documentation's organization, formatting, navigation, installation guidance, examples, and coverage of release verification.

### Miscellaneous

* Split celq's XML and greppable JSON implementations into the new `two_xml2json` and `serde_greppable` crates. celq now consumes those parsers as optional dependencies. Future releases of those two crates will polish their public surface.
* Bumped the CEL interpreter (`cel` -> 0.14.5), YAML parser (`serde-saphyr` -> 1.2.0), and TOML parser (`toml` -> 1.1.6).
* This is the first release that tries to provide reproducible builds. Independent rebuilds compare selected release archives and publish attestations when they match. This is a best-effort initiative, if the builds fail to reproduce the release will still be considered a success.
* There are now pre-built Linux PowerPC64LE binaries for both GNU and musl environments.
* There are now NPM packages for FreeBSD x86-64 and Linux PowerPC64LE.

[v0.6.0](https://github.com/IvanIsCoding/celq/releases/tag/v0.6.0) - 2026-08-28
------------------------------------------------------------------------

### Added

* celq binaries now can be audited with [cargo-audit](https://crates.io/crates/cargo-audit)!
* Added pre-built Linux RISC-V 64 binaries across GitHub releases, PyPI, NPM.
* Added Windows ARM64 to the test matrix.

### Miscellaneous

* Bumped the CEL interpreter (`cel` -> 0.14.4). This could theoretically be a breaking change as the new version of the parser is more compliant with the CEL standard and could reject some existing queries. In practice, this is a quality-of-life improvement by rejecting buggy queries.
* Bumped the YAML parser (`serde-saphyr` -> 1.1.0).
* Bumped the toml parser (`toml` -> 1.1.4).
* Raised the MSRV to Rust 1.94
* celq binaries are not built with [cargo-auditable](https://crates.io/crates/cargo-auditable). This embeds informations about
celq's dependencies in the binary. Should there be a vulnerability affecting one of celq's parsers or CEL interpreter, external tools
will be able to flag it.

[v0.5.0](https://github.com/IvanIsCoding/celq/releases/tag/v0.5.0) - 2026-05-25
------------------------------------------------------------------------

### Miscellaneous

* Switched XML parsing from `xml2json-rs` to `quick-xml`. `xml2json-rs` was based on an older version of `quick-xml` that was not up-to-date
* The gron parser and serializer has been completely rewritten using `serde_json`. The `ressa` and `resast` JavaScript parsers were dropped. This leads to smaller binaries.
* Bumped the YAML parser (`serde-saphyr` -> 0.0.26).
* Bumped the toml parser (`toml` -> 1.1.2).


[v0.4.0](https://github.com/IvanIsCoding/celq/releases/tag/v0.4.0) - 2026-03-28
------------------------------------------------------------------------

### Miscellaneous

* Bumped the CEL interpreter (`cel` -> 0.13.0). This could theoretically be a breaking change as the new version of the parser is more compliant with the CEL standard and could reject some existing queries. In practice, this is a quality-of-life improvement by rejecting buggy queries.
* Bumped the YAML parser (`serde-saphyr` -> 0.0.22).

[v0.3.4](https://github.com/IvanIsCoding/celq/releases/tag/v0.3.4) - 2026-03-07
------------------------------------------------------------------------

### Fixed

* Fixed conflicting input-format flags so combinations such as `--from-xml --from-yaml` are rejected by the CLI.

### Miscellaneous

* Bumped the TOML parser (`toml` -> 1.0.6).
* Bumped the YAML parser (`serde-saphyr` -> 0.0.21).

[v0.3.3](https://github.com/IvanIsCoding/celq/releases/tag/v0.3.1) - 2026-02-05
------------------------------------------------------------------------


### Added

* Added XML support via the `--from-xml` flag.

[v0.3.2](https://github.com/IvanIsCoding/celq/releases/tag/v0.3.2) - 2026-01-31
------------------------------------------------------------------------


### Miscellaneous

* Downgraded the MSRV to Rust 1.90

[v0.3.1](https://github.com/IvanIsCoding/celq/releases/tag/v0.3.1) - 2026-01-27
------------------------------------------------------------------------


### Added

* Added the `--verify-checksum` flag to `install.sh`
* `celq` now returns the input if no expression is passed


[v0.3.0](https://github.com/IvanIsCoding/celq/releases/tag/v0.3.0) - 2026-01-24
------------------------------------------------------------------------

### Added

* Added pre-compiled FreeBSD aarch64 binaries
* Added support for parallelism when using the `--slurp` flag with the `-j` flag

### Miscellaneous

* Bumped the YAML parser to incorporate fixes (`serde-saphyr` -> 0.0.16)
* Bumped the JSON5 parser to incorporate fixes (`json5` -> 1.3.0)
* Bumped the MSRV to Rust 1.91

[v0.2.0](https://github.com/IvanIsCoding/celq/releases/tag/v0.2.0) - 2026-01-17
------------------------------------------------------------------------

### Added

* Added support for array slicing (e.g. `this.slice(a, b)`)
* Added support for TOML inputs with `--from-toml`
* Added support for YAML inputs with `--from-yaml`
* Added support for greppable json inspired by `gron`
* Added pre-compiled FreeBSD x86-64 binaries
* Added pre-built binaries via Scoop for Windows

### Miscellaneous

* Live Playground: https://celq-playground.github.io/
* Switched PyPI releases to use Zig for linking
* Switched pre-built binaries to prettified names (e.g. `celq-macos-aarch64.tar.gz` instead of `celq-aarch64-apple-darwin.tar.gz`)

[v0.1.1](https://github.com/IvanIsCoding/celq/releases/tag/v0.1.1) - 2026-01-06
------------------------------------------------------------------------

### Added

* Added pre-built binaries via GitHub releases
* Added pre-built binaries via PyPI
* Added pre-built binaries via NPM
* Added pre-built binaries via Chocolatey for Windows
* Added pre-built binaries via Homebrew for Mac
* Published Nix support with and without flakes

### Miscellaneous

* Many documentation updates.

[v0.1.0](https://github.com/IvanIsCoding/celq/releases/tag/v0.1.0) - 2026-01-04
------------------------------------------------------------------------

Initial release with support for JSON and JSON5!
