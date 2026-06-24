mod base;

#[cfg(all(feature = "from-xml", feature = "from-yaml"))]
use base::golden_test_failure;

use std::io;
use std::process;

// Boolean mode: a `false` result exits with code 1 by design.
#[test]
fn test_boolean_false_exit_code() -> io::Result<()> {
    let mut child = process::Command::new(env!("CARGO_BIN_EXE_celq"))
        .args(["-n", "-b", "1 < 0"])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()?;

    // Close stdin
    drop(child.stdin.take());

    let output = child.wait_with_output()?;

    // Check that the exit code is 1 for false
    assert_eq!(
        output.status.code(),
        Some(1),
        "Expected exit code 1 for false boolean result, got {:?}",
        output.status.code()
    );

    Ok(())
}

#[test]
fn test_slice_fails_without_extensions() -> io::Result<()> {
    let mut child = process::Command::new(env!("CARGO_BIN_EXE_celq"))
        .args(["--no-extensions", "this.items.slice(0, 2)"])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()?;

    // Write test input
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(br#"{"items":[1,2,3,4,5]}"#)?;
    }

    let output = child.wait_with_output()?;

    // Check that the command failed (non-zero exit code)
    assert!(
        !output.status.success(),
        "Expected command to fail without extensions, but it succeeded"
    );

    Ok(())
}

#[cfg(all(feature = "from-xml", feature = "from-yaml"))]
#[test]
fn test_conflicting_input_format_flags_fail() -> io::Result<()> {
    golden_test_failure(
        &["--from-xml", "--from-yaml", "this"],
        "<a>1</a>",
        "the argument '--from-xml' cannot be used with '--from-yaml'",
    )
}
