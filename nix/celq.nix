{
  lib,
  rustPlatform,
  fetchCrate,
  versionCheckHook,
  runCommand,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "celq";
  version = "0.3.0";

  # Fetch from crates.io
  src = fetchCrate {
    pname = finalAttrs.pname;
    version = finalAttrs.version;
    sha256 = "sha256-c6m1Ekc8vmJPuL+OTfARKRTGUb1cIihC/WMSszdgnbI=";
  };

  cargoHash = "sha256-PZtrLT4NDPvrQ7py2LrmpQp1FpbXGmBNmkbF7kubpgo=";

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
