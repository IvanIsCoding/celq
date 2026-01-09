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

To force the installer to overwrite a version instead of failing, pass the `--force` flag:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --force
```

To pin a specific version, change the URL to include the version. For example:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/v0.1.2/install.sh | bash
```

Will always install the same version, 0.1.2.

By default, the installer always chooses Linux binaries that are the most portable (i.e. `musl`). It does not check the `glibc`. The `--target` flag can be convenient for those cases. For example:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --target x86_64-unknown-linux-gnu
```

Will install the version that links against the glibc version.

Lastly, to prevent rate limits from GitHub, set the `$GITHUB_TOKEN` with a valid token. The limit for logged in users is considerably higher.

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

## Acknowledgments

Special thanks to the maintainers of:
- **[just](https://github.com/casey/just)** for providing the shell script installer that was forked by us
- **[git-cliff](https://github.com/orhun/git-cliff)** for their fantastic blueprint for the NPM release
- **[maturin](https://github.com/PyO3/maturin)** for providing the code to help us build for the Python Package Index