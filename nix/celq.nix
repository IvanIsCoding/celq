{
  lib,
  rustPlatform,
  fetchCrate,
  versionCheckHook,
  runCommand,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "celq";
  version = "0.2.0";

  # Fetch from crates.io
  src = fetchCrate {
    pname = finalAttrs.pname;
    version = finalAttrs.version;
    sha256 = "sha256-C8IayTYJ1i2/ZOHS2tohASpL8uOPTYrQCyOJTbcYJ14=";
  };

  cargoHash = "sha256-Gk0C1LHRP+xWP1sTJrnyJSSPwKo7ZPh5/CP8is2/t9A=";

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
