{
  lib,
  rustPlatform,
  fetchCrate,
  versionCheckHook,
  runCommand,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "celq";
  version = "0.4.0";

  # Fetch from crates.io
  src = fetchCrate {
    pname = finalAttrs.pname;
    version = finalAttrs.version;
    sha256 = "sha256-54lMkgGch7GwpuOmH2y67VNLTC1hMbhkqjLf94DoekA=";
  };

  cargoHash = "sha256-cUpf9/h95MD36xQaRPwg2Fz9lQVxK+q8ftP4B4gJEOc=";

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
