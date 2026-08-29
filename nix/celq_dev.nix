{
  lib,
  rustPlatform,
  fetchCrate,
  versionCheckHook,
  runCommand,
}:

let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "celq";
  version = cargoToml.package.version;

  # Fetch from HEAD
  src = lib.cleanSource ../.;

  postPatch = ''
    # importCargoLock normalizes crates.io entries to the static CDN above;
    # keep the lockfile in the unpacked source identical to the vendored one.
    substituteInPlace Cargo.lock \
      --replace-fail \
      'registry+https://github.com/rust-lang/crates.io-index' \
      'sparse+https://static.crates.io/crates'
  '';

  cargoLock = {
    lockFileContents = builtins.replaceStrings
      [ "registry+https://github.com/rust-lang/crates.io-index" ]
      [ "sparse+https://static.crates.io/crates" ]
      (builtins.readFile ../Cargo.lock);
    allowBuiltinFetchGit = true;
    extraRegistries = {
      "sparse+https://static.crates.io/crates" = "https://static.crates.io/crates";
    };
  };

  nativeInstallCheckInputs = [
    versionCheckHook
  ];

  passthru = {
    tests.simple =
      runCommand "celq-test"
        {
          nativeBuildInputs = [ finalAttrs.finalPackage ];
        }
        ''
          set -o pipefail

          # Test the command `celq -n "1 > 0"`, which should return exit code 0
          if ! celq -n "1 > 0"; then
            echo "Test failed: celq can't execute simple expression"
            exit 1
          else
            echo "Test passed: celq executed simple expression successfully"
          fi
        '';
  };

  meta = {
    description = "celq - A Common Expression Language (CEL) CLI Tool";
    homepage = "https://github.com/IvanIsCoding/celq";
    license = lib.licenses.mit;
    mainProgram = "celq";
    platforms = lib.platforms.unix;
  };
})
