# celq [![Minimum rustc 1.85](https://img.shields.io/badge/rustc-1.85+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)

**celq** is a command-line tool for evaluating [Common Expression Language (CEL)](https://cel.dev/) expressions. It processes JSON input, performs computations, and outputs results. Think of it as if [jq](https://jqlang.org/) supported CEL.

## Quick Start

`celq` reads JSON from the input and lets users process it with CEL:

```bash
echo '["apples", "bananas", "blueberry"]' | celq 'this.filter(x, x.contains("a"))'
# Outputs: ["apples","bananas"]
```

`celq` can also evaluate expressions with arguments, without reading from the input:

```bash
celq -n --arg='fruit:string=apple' 'fruit.contains("a")'
# Outputs: true
```

For detailed usage examples and recipes, see the [manual](docs/manual.md).

## Why?

There are implementations of CEL for [Go](https://github.com/google/cel-go), [C++](https://github.com/google/cel-cpp), [Python](https://github.com/cloud-custodian/cel-python), [JavaScript](https://github.com/marcbachmann/cel-js), [Rust](https://github.com/cel-rust/cel-rust), and possibly more languages.

`celq` brings the same CEL syntax to the command-line. `celq` is not necessarily better than jq, but perhaps it makes it easier to reuse snippets of code accross multiple places.

Moreover, the CEL specification is simpler than the jqlang specification. If you need something less powerful than `jq` or Python, then `celq` might be what you are looking for.

## Installation

```bash
cargo install celq
```

This installs `celq` from source. In the future, pre-built binaries will be provided.

## Limitations

### CEL Implementation Differences

`celq` uses [cel-rust](https://github.com/cel-rust/cel-rust), a community-maintained Rust implementation of CEL, rather than the official Go implementation. 

While `cel-rust` provides excellent compatibility with the CEL specification, there may be edge cases or advanced features where behavior differs from the official implementation. If you find one, feel free to report it at their repository.

### JSON Parsing

`celq` eagerly parses all JSON input into memory before evaluation. This design was made to simplify the code implementation, at the cost of memory and performance.

Currently, there are no benchmarks for `celq`. I believe the tool is "good enough" for my personal use. That might be revisited in the future.

## Non-Goals

### REPL

While conceptually interesting, `celq` does not aim to be a CEL REPL. In the original author's view, that should live on a separate binary.

### YAML Support

`celq` works with JSON. I originally considered supporting YAML as a supported input format. However, the amount of YAML edge cases pushed me back to JSON. Although that might change in the future, please do not open an issue asking for YAML support as of today.

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

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.
