# celq
 [![Crates.io](https://img.shields.io/crates/v/celq.svg)](https://crates.io/crates/celq) [![Documentation](https://docs.rs/celq/badge.svg)](https://docs.rs/celq) [![Playground](https://img.shields.io/badge/playground-live-brightgreen)](https://celq-playground.github.io/) [![Minimum rustc 1.90](https://img.shields.io/badge/rustc-1.90+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html) 

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

Popular configuration formats such as JSON5, YAML, and TOML are supported. The closely related format NDJSON is also supported.

For detailed usage examples and recipes, see the [manual](https://docs.rs/celq/latest/celq/).

### Interactive Playground
Want to try `celq` without installing anything? Visit the [celq-playground](https://celq-playground.github.io/) to try it in your browser!

## Why?

There are implementations of CEL for [Go](https://github.com/google/cel-go), [Java](https://github.com/google/cel-java), [C++](https://github.com/google/cel-cpp), [JavaScript](https://github.com/marcbachmann/cel-js), [Python](https://github.com/cloud-custodian/cel-python), [C#](https://github.com/telus-oss/cel-net), [Rust](https://github.com/cel-rust/cel-rust), [Ruby](https://gitlab.com/os85/cel-ruby), and possibly more languages.

`celq` brings the same CEL syntax to the command-line. `celq` is not necessarily better than jq, but perhaps it makes it easier to reuse snippets of code across multiple places.

Moreover, the CEL specification is simpler than the jqlang specification. If you need something less powerful than `jq` or Python, then `celq` might be what you are looking for.

Check our [comparison with other tools](https://docs.rs/celq/latest/celq/comparison_with_other_tools) for more details.

## Installation

### Pre-built Binaries

We publish pre-built binaries for Linux, macOS, FreeBSD, and Windows in celq's [GitHub Releases page](https://github.com/IvanIsCoding/celq/releases). To install the current version for Linux or macOS, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | bash
```

Notice that the installer tries not to be clever and doesn't modify `$PATH` or overwrite existing files. To specify a destination, use the `--to` flag:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --to DESTINATION
```

See the [installation guide](https://docs.rs/celq/latest/celq/installation_guide) for more details on the installer such as `--force` to replace existing binaries, `--target` to specify which binary to download, versioned URLs, GitHub tokens, attestations, and more.

### Homebrew (macOS)

If you are a [macOS Homebrew](https://brew.sh/) user, then you can install celq with:

```bash
brew install get-celq/tap/celq
```

### Scoop (Windows)

If you are a [Scoop](https://scoop.sh/) user on Windows, you can install `celq` with:

```bash
scoop bucket add get-celq https://github.com/get-celq/scoop-bucket
scoop install get-celq/celq
```

### Chocolatey (Windows)

If you are a [Chocolatey](https://community.chocolatey.org/) user on Windows, you can install `celq` with:

```bash
choco install celq
```

### Cargo

#### Installing From Source 

If you want to install from source, celq publishes to [crates.io](https://crates.io/crates/celq).

```bash
cargo install celq --locked
```

#### Installing With cargo-binstall

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installed, you can install pre-built binaries directly:

```bash
cargo binstall celq
```

### GitHub Actions

`celq` can be used in GitHub actions. For one-off commands, the [get-celq/celq-action](https://github.com/get-celq/celq-action) is the quickest way:

```yaml
- name: Example Celq Action
  id: exampleID
  uses: get-celq/celq-action@main
  with:
    cmd: celq 'this.exampleID' < example.json

- name: Reuse a variable obtained in another step
  run: echo ${{ steps.exampleID.outputs.result }}
```

See the [installation guide](https://docs.rs/celq/latest/celq/installation_guide) for more details on GitHub actions such as pinning the `celq` version and the Action itself.

If you are going to use `celq` in scripts or for multiple calls, we recommend using [taiki-e/install-action](https://github.com/taiki-e/install-action):

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: celq
```

### Nix

`celq` is available for [Nix](https://github.com/NixOS/nix). To run it as a Flake:

```bash
nix run github:IvanIsCoding/celq -- -n '"Hello World"'
```

By default, Nix fetches the stable version from crates.io. If you want to run the code from HEAD, use the `dev` derivation:

```bash
nix run github:IvanIsCoding/celq#dev -- -n '"Hello World"'
```

See the [installation guide](https://docs.rs/celq/latest/celq/installation_guide) for other Nix setups.

### FreeBSD

FreeBSD builds are tested in [Cirrus CI](https://cirrus-ci.org/) and cross-compiled with [Zig](https://github.com/rust-cross/cargo-zigbuild). Although `celq` is not yet in the ports tree, it does publish pre-built binaries:

```bash
VERSION=v0.3.0
RELEASE_URL=https://github.com/IvanIsCoding/celq/releases/download/${VERSION}
PLATFORM=x86_64 # or aarch64

fetch ${RELEASE_URL}/celq-freebsd-${PLATFORM}.tar.gz

tar xzf celq-freebsd-${PLATFORM}.tar.gz
su root -c 'install -m 755 celq /usr/local/bin/'
```

`celq` can also be installed from source following the [Cargo](#cargo) section. We strive to always compile with the Rust version provided in the ports tree.

### OpenBSD

OpenBSD builds are tested in CI using the latest stable release. `celq` strives to always compile with the Rust version provided in the ports tree. Refer to the [Cargo](#cargo) section for instructions.

### NPM (Node.js/JavaScript)

`celq` is packaged for [NPM](https://www.npmjs.com/package/celq). Node.js users can install celq in their project with:

```bash
npm install celq
```

This adds celq to `package.json` and makes it available for scripts. It's also possible to run single commands with [npx](https://docs.npmjs.com/cli/v8/commands/npx):

```bash
npx celq -n '"Hello World"'
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

## Limitations

### Eager JSON Parsing

`celq` eagerly parses all JSON input into memory before evaluation. This design was made to simplify the code implementation, at the cost of memory and performance.

Currently, there are no benchmarks for `celq`. I believe the tool is "good enough" for my personal use. That might be revisited in the future. In the meantime, expect `celq` to be slower than `jq`. With that being said, `celq` feels snappy in practice. `celq` glues together Rust parsers that are performant with a CEL engine that strives to be fast.

### CEL Implementation Differences

`celq` uses [cel-rust](https://github.com/cel-rust/cel-rust), a community-maintained Rust implementation of CEL, rather than the official Go implementation. 

There may be edge cases or advanced features where behavior differs from the official implementation. If you find one, open an issue at the celq repository and we'll triage the issue before sending it to cel-rust.

### List and Map Arguments

Currently, the `--arg` syntax only supports `int`, `bool`, `float`, and `string`. Support for other CEL types will be added in the future.

### Malformed expressions

Currently, the error messages for CEL expressions that are invalid are cryptic. We will enhance them in future releases.

## Non-Goals

### REPL

While conceptually interesting, `celq` does not aim to be a CEL REPL. In the original author's view, that should live on a separate binary.

### Full YAML Support

`celq` works with JSON. YAML is supported as a best-effort. If the ingested YAML can be translated to JSON, `celq` most likely works fine. Full YAML support is out-of-scope, as the specification has too many edge cases.

## Acknowledgments

Special thanks to the maintainers of:
- **[cel-rust](https://github.com/cel-rust/cel-rust)** for providing the CEL evaluation engine that powers `celq`
- **[cel-python](https://github.com/cloud-custodian/cel-python)** for publishing their CLI. `celq` has heavily drawn from their interface
- **[jaq](https://github.com/01mf02/jaq)** for giving an excellent blueprint on how to test a Rust CLI
- **[gron](https://github.com/tomnomnom/gron)** for greppable JSON

## Large Language Models Disclosure

Many commits in this repository were co-authored by LLMs. All commits were guided and reviewed by a human. I tried my best to keep things simple and auditable.

All the documentation in the manual has been hand-crafted. That was done to keep the tone of the original author. If you find a typo or a grammar mistake, please send a pull request.

## License

This project is dual-licensed under the MIT License and Apache 2.0 licenses.  See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) file for details.

The `install.sh` published with each GitHub release and its template at the root of the repository (`template_install.sh`) are licensed independently. Those files are under the CC0-1.0 license. They are the original work of Casey Rodarmor from [just](https://github.com/casey/just) and have been adapted for celq.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in `celq` by you shall be dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache 2.0 license](LICENSE-APACHE). Contributions to `template_install.sh` shall be dedicated to the public domain. Any additional terms or conditions shall not apply.
