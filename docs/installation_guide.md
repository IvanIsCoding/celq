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

To force the installer to overwrite a version instead of failing, pass the `--force` flag:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --force
```

To pin a specific version, change the URL to include the version. For example:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/v0.3.0/install.sh | bash
```

Will always install the same version, 0.3.0.

The `--target` option can be specified to avoid guessing the architecture. For example:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --target x86_64-unknown-linux-gnu
```

Will always install the binary for `x86_64-unknown-linux-gnu`. See [Rust's target triples](https://doc.rust-lang.org/beta/rustc/platform-support.html) for a list of possible options.

To prevent rate limits from GitHub, set the `$GITHUB_TOKEN` with a valid token. The limit for logged in users is considerably higher. You might also find the [GitHub Actions](#github-actions) section valuable if running in that environment.

If you are interested in the checksums and the attestations for the pre-built binaries and the installer, [see the Integrity and Authenticity section](#integrity-and-authenticity).

Lastly, see [the quirks for the shell script installer](#shell-script-installer-quirks) for how it chooses what binary to install on Linux and the path it chooses.

### Homebrew (macOS)

If you are a [macOS Homebrew](https://brew.sh/) user, then you can install celq with:

```bash
brew install get-celq/tap/celq
```

The formula also works for [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux).

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

### Nix

`celq` is available for [Nix](https://github.com/NixOS/nix). To run it as a flake:

```bash
nix run github:IvanIsCoding/celq -- -n '"Hello World"'
```

By default, Nix fetches the stable version from crates.io. If you want to run the code from HEAD, use the `dev` derivation:

```bash
nix run github:IvanIsCoding/celq#dev -- -n '"Hello World"'
```

We also include a `default.nix` for non-Flake users:

```bash
git clone https://github.com/IvanIsCoding/celq
cd celq
nix-build
./result/bin/celq -n '"Hello World"'
```

### FreeBSD

FreeBSD builds are tested in [Cirrus CI](https://cirrus-ci.org/) and cross-compiled with [Zig](https://github.com/rust-cross/cargo-zigbuild). Although `celq` is not yet in the ports tree, it does publish pre-built binaries that can be installed manually:

```bash
VERSION=v0.2.0
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

## Integrity and Authenticity

`celq` publishes a `SHA256SUMS` file for each of its release in the [GitHub Releases page](https://github.com/IvanIsCoding/celq/releases). The checksum can be used to verify integrity of the downloaded files.

The `celq` installer supports the `--verify-checksum` flag to ensure the integrity of the pre-built binaries:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --verify-checksum
```

`celq` also generates [artifact attestations](https://github.com/IvanIsCoding/celq/attestations) for each file in the Releases page, including the installer. To verify the authenticity of a file, use the [GitHub CLI](https://cli.github.com/) with the following command:

```bash
gh attestation verify <path_to_file> --repo IvanIsCoding/celq
```

Because `install.sh` is published with each release, that means it can also be verified:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh > install.sh
gh attestation verify intall.sh --repo IvanIsCoding/celq
```

The installer also provides the `--verify-attestation` flag. After verifying the installer, run:

```bash
bash install.sh --verify-attestation
```

This way, you can guarantee that both the installer and the downloaded binaries are authentic.

Running the installer with the `--verify-checksum` requires either `sha256sum` or `shasum` to be available. If none of these tools is available, the installer will fail. 

Running the installer with the `--verify-attestation` requires the GitHub CLI (`gh`). If `gh` is not found, the script will fail. If the user is not authenticated (`gh auth login`), the option will also fail. For scripts and non-interactive environments like CI, `gh auth login --with-token $GITHUB` is an option for authenticaitng when using this installer feature.

## Shell Script Installer Quirks

By default, the installer always chooses Linux binaries that are the most portable (i.e. `musl`). It does not check the `glibc`. The `--target` flag can be convenient for those cases. Pass `--target x86_64-unknown-linux-gnu` or `aarch64-unknown-linux-gnu` if you need the glibc version.

It is worth highlighting that if no `--to` flag is specified, the installer tries to write `$CARGO_HOME/bin/celq`, `$HOME/.cargo/bin/celq`, `$HOME/.local/bin/celq` in that order. If a directory does not exist, the installer moves to the next guess. `$HOME/bin` is the final destination if none of directories exist. If the directory that `celq` was installed is not in the path, the installer will warn the user.

Although unusual, the installer probably works for Windows in Git Bash (MSYS2) and Cygwin. It will detect the platform correctly and download the binaries. As of today, we do not have a Power Shell installer yet, so this option could be interesting for Windows users that do not have Chocolatey/Scoop available.

## Acknowledgments

Special thanks to the maintainers of:
- **[just](https://github.com/casey/just)** for providing the shell script installer that was forked by us
- **[git-cliff](https://github.com/orhun/git-cliff)** for their fantastic blueprint for the NPM release
- **[maturin](https://github.com/PyO3/maturin)** for providing the code to help us build for the Python Package Index
- **[vidmerger](https://github.com/tgotwig/vidmerger)** for providing details on how to package for Chocolatey ([including this blog post](https://dev.to/tgotwig/publish-a-simple-executable-from-rust-on-chocolatey-2pbl))

Thanks also go to [quentinmit@](https://github.com/quentinmit) for guidance on packaging for Nix.