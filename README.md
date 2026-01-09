# celq
 [![Crates.io](https://img.shields.io/crates/v/celq.svg)](https://crates.io/crates/celq) [![Documentation](https://docs.rs/celq/badge.svg)](https://docs.rs/celq) [![Minimum rustc 1.90](https://img.shields.io/badge/rustc-1.90+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)

**celq** is a command-line tool for evaluating [Common Expression Language (CEL)](https://cel.dev/) expressions. It processes JSON input, performs computations, and outputs results. Think of it as if [jq](https://jqlang.org/) supported CEL.

## Quick Start

`celq` reads JSON from the input and lets users process it with CEL:

```bash
echo '["apples", "bananas", "blueberry"]' | celq 'this.filter(s, s.contains("a"))'
# Outputs: ["apples","bananas"]
```

`celq` can also evaluate expressions with arguments, without reading from the input:

```bash
celq -n --arg='fruit:string=apple' 'fruit.contains("a")'
# Outputs: true
```

Closely related formats such as NDJSON and JSON5 are also supported.

For detailed usage examples and recipes, see the [manual](https://docs.rs/celq/latest/celq/).

## Why?

There are implementations of CEL for [Go](https://github.com/google/cel-go), [C++](https://github.com/google/cel-cpp), [Python](https://github.com/cloud-custodian/cel-python), [JavaScript](https://github.com/marcbachmann/cel-js), [Rust](https://github.com/cel-rust/cel-rust), and possibly more languages.

`celq` brings the same CEL syntax to the command-line. `celq` is not necessarily better than jq, but perhaps it makes it easier to reuse snippets of code accross multiple places.

Moreover, the CEL specification is simpler than the jqlang specification. If you need something less powerful than `jq` or Python, then `celq` might be what you are looking for.

## Installation

### Pre-built Binaries

We publish pre-built binaries for Linux, macOS, and Windows in celq's [GitHub Releases page](https://github.com/IvanIsCoding/celq/releases). To install the current version for Linux or macOS, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | bash
```

Notice that the installer tries not to be clever and doesn't modify `$PATH` or overwrite existing files. To specify a destination, use the `--to` flag:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --to DESTINATION
```

See the the [installation guide](https://docs.rs/celq/latest/celq/) for more details on the installer such as `--force` to replace existing binaries, `--target` to specify which binary to download, versioned URLs, GitHub tokens, and more.

### Homebrew (macOS)

If you are a [macOS Homebrew](https://brew.sh/) user, then you can install celq with:

```bash
brew install get-celq/tap/celq
```
#### Installing From Source 

If you want to install from source, celq publishes to [crates.io](https://crates.io/crates/celq).

```bash
cargo install celq
```

#### Installing With cargo-binstall

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installed, you can install pre-built binaries directly:

```bash
cargo binstall celq
```

### Python

celq is packaged for [PyPI](https://pypi.org/project/celq/). Python users can install it with `pip`:

```bash
pip install celq
```

If you have [uv](https://github.com/astral-sh/uv) installed, `celq` can be used as a tool:
```bash
uvx celq -n '"Hello World"'
```

### NPM (Node.js/JavaScript)

Node.js users can install celq in their project with:

```bash
npm install celq
```

This adds celq to `package.json` and makes it available for scripts. It's also possible to run single commands with [npx](https://docs.npmjs.com/cli/v8/commands/npx):

```bash
npx celq -n '"Hello World"'
```

## Limitations

### JSON Parsing

`celq` eagerly parses all JSON input into memory before evaluation. This design was made to simplify the code implementation, at the cost of memory and performance.

Currently, there are no benchmarks for `celq`. I believe the tool is "good enough" for my personal use. That might be revisited in the future. In the meantime, expect `celq` to be slower than `jq`.

### CEL Implementation Differences

`celq` uses [cel-rust](https://github.com/cel-rust/cel-rust), a community-maintained Rust implementation of CEL, rather than the official Go implementation. 

While `cel-rust` provides excellent compatibility with the CEL specification, there may be edge cases or advanced features where behavior differs from the official implementation. If you find one, feel free to report it at their repository.

### List and Map Arguments

Currently, the `--arg` syntax only supports `int`, `bool`, `float`, and `string`. Support for other CEL types will be added in the future.

## Non-Goals

### REPL

While conceptually interesting, `celq` does not aim to be a CEL REPL. In the original author's view, that should live on a separate binary.

### Full YAML Support

`celq` works with JSON. YAML will be supported in the future as a best-effort. Full YAML support is out-of-scope, as the specification has too many edge cases.

## Acknowledgments

Special thanks to the maintainers of:
- **[cel-rust](https://github.com/cel-rust/cel-rust)** for providing the CEL evaluation engine that powers `celq`
- **[cel-python](https://github.com/cloud-custodian/cel-python)** for publishing their CLI. `celq` has heavily drawn from their interface
- **[jaq](https://github.com/01mf02/jaq)** for giving an excellent blueprint on how to test a Rust CLI

## Large Language Models Disclosure

Many commits in this repository were co-authored by LLMs. All commits were guided and reviewed by a human. The original author has significantly refactored the AI output to conform to his opinionated view.

All the documentation in the manual has been hand-crafted. That was done to keep the tone of the original author. If you find a typo or a grammar mistake, please send a pull request.

## License

This project is dual-licensed under the MIT License and Apache 2.0 licenses.  See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) file for details.

The `install.sh` published with each GitHub release and its template at the root of the repository (`template_install.sh`) are licensed independently. Those files are under the CC0-1.0 license. They are the original work of Casey Rodarmor from [just](https://github.com/casey/just) and have been adapted for celq.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.
