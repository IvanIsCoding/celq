Changelog
=========

[v0.3.2](https://github.com/IvanIsCoding/celq/releases/tag/v0.3.2) - 2026-01-31

### Miscellaneous

* Downgraded the MSRV to Rust 1.90

[v0.3.1](https://github.com/IvanIsCoding/celq/releases/tag/v0.3.1) - 2026-01-27

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