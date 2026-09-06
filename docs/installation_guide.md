### Pre-built Binaries

We publish pre-built binaries for Linux, macOS, FreeBSD, and Windows on celq's [GitHub Releases page](https://github.com/IvanIsCoding/celq/releases).

To install the current version on Linux or macOS:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | bash
```

#### Installer options

<div class="installation-tabs" style="--arity: 4">
<details name="installer-option" style="--n: 1" open>
<summary><h5>Destination</h5></summary>
<div class="installation-tab-content">
<p>The installer doesn't modify <code>$PATH</code> or overwrite existing files. Use <code>--to</code> to install celq at a specific destination:</p>

<pre><code class="language-bash">curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --to DESTINATION</code></pre>
</div>
</details>
<details name="installer-option" style="--n: 2">
<summary><h5>Overwrite</h5></summary>
<div class="installation-tab-content">
<p>The installer doesn't modify <code>$PATH</code> or overwrite existing files. Use <code>--force</code> to overwrite an existing celq binary instead of failing:</p>

<pre><code class="language-bash">curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --force</code></pre>
</div>
</details>
<details name="installer-option" style="--n: 3">
<summary><h5>Version</h5></summary>
<div class="installation-tab-content">
<p>Include a version in the URL to pin the installation. This example always installs version 0.6.0:</p>

<pre><code class="language-bash">curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/v0.6.0/install.sh | bash</code></pre>
</div>
</details>
<details name="installer-option" style="--n: 4">
<summary><h5>Target</h5></summary>
<div class="installation-tab-content">
<p>Use <code>--target</code> to select a platform instead of detecting it. This example always installs <code>x86_64-unknown-linux-gnu</code>:</p>

<pre><code class="language-bash">curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --target x86_64-unknown-linux-gnu</code></pre>

<p>See <a href="https://doc.rust-lang.org/beta/rustc/platform-support.html">Rust's target triples</a> for more information about target triples.</p>
</div>
</details>
</div>

#### Rate limitting

To prevent rate limits from GitHub, set the `$GITHUB_TOKEN` with a valid token. The limit for logged in users is considerably higher. You might also find the [GitHub Actions](#github-actions) section valuable if running in that environment.

If you are interested in the checksums and the attestations for the pre-built binaries and the installer, [see the Integrity and Authenticity section](#integrity-and-authenticity).

Lastly, see [the quirks for the shell script installer](#shell-script-installer-quirks) for how it chooses what binary to install on Linux and the path it chooses.

### Homebrew (macOS)

If you are a [macOS Homebrew](https://brew.sh/) user, then you can install celq with:

```bash
brew install get-celq/tap/celq
```

The formula also works for [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux).

### Windows

celq is available for multiple Windows package managers:

<div class="installation-tabs" style="--arity: 3">
<details name="windows-installation" style="--n: 1" open>
<summary><h4>Scoop</h4></summary>
<div class="installation-tab-content">
<p>Add the <a href="https://scoop.sh/">Scoop</a> bucket, then install celq:</p>

<pre><code class="language-powershell">scoop bucket add get-celq https://github.com/get-celq/scoop-bucket
scoop install get-celq/celq</code></pre>
</div>
</details>
<details name="windows-installation" style="--n: 2">
<summary><h4>WinGet</h4></summary>
<div class="installation-tab-content">
<p>Install celq with <a href="https://learn.microsoft.com/windows/package-manager/winget/">WinGet</a>:</p>

<pre><code class="language-powershell">winget install IvanIsCoding.celq</code></pre>
</div>
</details>
<details name="windows-installation" style="--n: 3">
<summary><h4>Chocolatey</h4></summary>
<div class="installation-tab-content">
<p>Install celq with <a href="https://community.chocolatey.org/">Chocolatey</a>:</p>

<pre><code class="language-powershell">choco install celq</code></pre>
</div>
</details>
</div>

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

<div class="installation-tabs" style="--arity: 2">
<details name="github-action" style="--n: 1" open>
<summary><h4>get-celq/celq-action</h4></summary>
<div class="installation-tab-content">
<p><a href="https://github.com/get-celq/celq-action">get-celq/celq-action</a> is the quickest option for one-off commands and exposes the result as an output:</p>

<pre><code class="language-yaml">- name: Example Celq Action
  id: exampleID
  uses: get-celq/celq-action@main
  with:
    cmd: celq 'this.exampleID' &lt; example.json

- name: Reuse a variable obtained in another step
  run: echo ${{ steps.exampleID.outputs.result }}</code></pre>

<p>For reproducible workflows, select both versions:</p>

<ul>
<li>Set the celq version with the optional <code>version</code> parameter.</li>
<li>Set the action version in <code>get-celq/celq-action@actionVersion</code>.</li>
</ul>

<pre><code class="language-yaml">- name: Example Celq Action
  id: exampleID
  uses: get-celq/celq-action@v0.1
  with:
    version: '0.6.0'
    cmd: celq 'this.exampleID' &lt; example.json

- name: Reuse a variable obtained in another step
  run: echo ${{ steps.exampleID.outputs.result }}</code></pre>
</div>
</details>
<details name="github-action" style="--n: 2">
<summary><h4>taiki-e/install-action</h4></summary>
<div class="installation-tab-content">
<p>Use <a href="https://github.com/taiki-e/install-action">taiki-e/install-action</a> when a workflow runs celq in scripts or invokes it multiple times:</p>

<pre><code class="language-yaml">- uses: taiki-e/install-action@v2
  with:
    tool: celq</code></pre>
</div>
</details>
</div>

### Nix

`celq` is available for [Nix](https://github.com/NixOS/nix) with and without flakes:

<div class="installation-tabs" style="--arity: 2">
<details name="nix-installation" style="--n: 1" open>
<summary><h4>Flakes</h4></summary>
<div class="installation-tab-content">
<p>Run the stable version fetched from crates.io:</p>

<pre><code class="language-bash">nix run github:IvanIsCoding/celq -- -n '"Hello World"'</code></pre>

<p>To run the code from HEAD, use the <code>dev</code> package:</p>

<pre><code class="language-bash">nix run github:IvanIsCoding/celq#dev -- -n '"Hello World"'</code></pre>
</div>
</details>
<details name="nix-installation" style="--n: 2">
<summary><h4>Without flakes</h4></summary>
<div class="installation-tab-content">
<p>Clone the repository and build its <code>default.nix</code>:</p>

<pre><code class="language-bash">git clone https://github.com/IvanIsCoding/celq
cd celq
nix-build
./result/bin/celq -n '"Hello World"'</code></pre>
</div>
</details>
</div>

### BSDs

<div class="installation-tabs" style="--arity: 3">
<details name="bsd-installation" style="--n: 1" open>
<summary><h4>FreeBSD</h4></summary>
<div class="installation-tab-content">
<p>FreeBSD builds are tested in <a href="https://cirrus-ci.org/">Cirrus CI</a> and cross-compiled with <a href="https://github.com/rust-cross/cargo-zigbuild">Zig</a>. Although celq is not yet in the ports tree, pre-built binaries can be installed manually:</p>

<pre><code class="language-bash">VERSION=v0.2.0
RELEASE_URL=https://github.com/IvanIsCoding/celq/releases/download/${VERSION}
PLATFORM=x86_64 # or aarch64

fetch ${RELEASE_URL}/celq-freebsd-${PLATFORM}.tar.gz

tar xzf celq-freebsd-${PLATFORM}.tar.gz
su root -c 'install -m 755 celq /usr/local/bin/'</code></pre>

<p>celq can also be installed from source following the <a href="#cargo">Cargo</a> section. We strive to always compile with the Rust version provided in the ports tree.</p>
</div>
</details>
<details name="bsd-installation" style="--n: 2">
<summary><h4>OpenBSD</h4></summary>
<div class="installation-tab-content">
<p>OpenBSD builds are tested in CI using the latest stable release. celq strives to always compile with the Rust version provided in the ports tree. Refer to the <a href="#cargo">Cargo</a> section for installation instructions.</p>
</div>
</details>
<details name="bsd-installation" style="--n: 3">
<summary><h4>NetBSD</h4></summary>
<div class="installation-tab-content">
<p>NetBSD builds are tested in CI against the latest stable release. celq aims to remain compatible with the Rust version provided by the NetBSD pkgsrc quarterly branch. See the <a href="#cargo">Cargo</a> section for installation instructions.</p>
</div>
</details>
</div>

### NPM (Node.js/JavaScript)

`celq` is packaged for [NPM](https://www.npmjs.com/package/celq). Node.js users can install celq in their project with:

```bash
npm install celq
```

This adds celq to `package.json` and makes it available for scripts. It is also possible to run single commands with [npx](https://docs.npmjs.com/cli/v8/commands/npx):

```bash
npx celq -n '"Hello World"'
```

### Python

<div class="installation-tabs" style="--arity: 2">
<details name="python-installation" style="--n: 1" open>
<summary><h4>PyPI</h4></summary>
<div class="installation-tab-content">
<p>Install celq from <a href="https://pypi.org/project/celq/">PyPI</a> with <code>pip</code>:</p>

<pre><code class="language-bash">pip install celq</code></pre>

<p>With <a href="https://github.com/astral-sh/uv">uv</a>, run celq as a tool without installing it permanently:</p>

<pre><code class="language-bash">uvx celq -n '"Hello World"'</code></pre>
</div>
</details>
<details name="python-installation" style="--n: 2">
<summary><h4>Conda Forge</h4></summary>
<div class="installation-tab-content">
<p>The <a href="https://anaconda.org/channels/conda-forge/packages/celq/overview">conda-forge package</a> works with <code>conda</code>, <code>mamba</code>, <code>micromamba</code>, and <code>pixi</code>:</p>

<pre><code class="language-bash">conda install -c conda-forge celq</code></pre>

<p>With <a href="https://pixi.prefix.dev/latest/">pixi</a>, run celq in a temporary environment:</p>

<pre><code class="language-bash">pixi exec celq -n '"Hello World"'</code></pre>
</div>
</details>
</div>

### Mise

celq can be used with [mise](https://mise.jdx.dev/). To install celq, use the Conda back-end:

```bash
mise use -g conda:celq
```

Alternatively, add this to `mise.toml`:

```toml
[tools]
"conda:celq" = "latest"
```

## Integrity and Authenticity

`celq` publishes a `SHA256SUMS` file for each of its releases on the [GitHub Releases page](https://github.com/IvanIsCoding/celq/releases). The checksum can be used to verify the integrity of the downloaded files.

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
