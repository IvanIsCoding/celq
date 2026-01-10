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

Lastly, to prevent rate limits from GitHub, set the `$GITHUB_TOKEN` with a valid token. The limit for logged in users is considerably higher. You might also find the [GitHub Actions](#github-actions) section valuable if running in that environment.

### Homebrew (macOS)

If you are a [macOS Homebrew](https://brew.sh/) user, then you can install celq with:

```bash
brew install get-celq/tap/celq
```

The formula also works for [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux), but it will install from source instead of using bottles.

### Chocolatey (Windows)

If you are a [Chocolatey](https://community.chocolatey.org/) user on Windows, you can install `celq` with:

```bash
choco install celq
```

### Scoop (Windows)

If you are a [Scoop](https://scoop.sh/) user on Windows, you can install `celq` with:

```bash
scoop bucket add get-celq https://github.com/get-celq/scoop-bucket
scoop install get-celq/celq
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

The best practice for GitHub Actions is to select both the version for the tool:
* The tool version is specified by the optional `version` parameter
* The action version is specified `celq-action@actionVersion`

For example:
```yaml
- name: Example Celq Action
  id: exampleID
  uses: get-celq/celq-action@v0.1
  with:
    version: '0.1.2'
    cmd: celq 'this.exampleID' < example.json

- name: Reuse a variable obtained in another step
  run: echo ${{ steps.exampleID.outputs.result }}
```

If you are going to use `celq` in scripts or for multiple calls, we recommend using [taiki-e/install-action](https://github.com/taiki-e/install-action):

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: celq
```

## Acknowledgments

Special thanks to the maintainers of:
- **[just](https://github.com/casey/just)** for providing the shell script installer that was forked by us
- **[git-cliff](https://github.com/orhun/git-cliff)** for their fantastic blueprint for the NPM release
- **[maturin](https://github.com/PyO3/maturin)** for providing the code to help us build for the Python Package Index
- **[vidmerger](https://github.com/tgotwig/vidmerger)** for providing details on how to package for Chocolatey ([including this blog post](https://dev.to/tgotwig/publish-a-simple-executable-from-rust-on-chocolatey-2pbl))