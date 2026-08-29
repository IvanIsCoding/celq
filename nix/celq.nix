{
  lib,
  rustPlatform,
  fetchCrate,
  versionCheckHook,
  runCommand,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "celq";
  version = "0.6.0";

  # Fetch from crates.io
  src = fetchCrate {
    pname = finalAttrs.pname;
    version = finalAttrs.version;
    registryDl = "https://static.crates.io/crates";
    sha256 = "sha256-ldT9TfgZqhnRNmccCIxDFKNWQmVZW+MWR2bDCWGN6Ds=";
  };

  cargoHash = "sha256-X5eNy4Aj/mo6NbeX13NitjkNayR6h4QdO16vfuFlz8M=";

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
