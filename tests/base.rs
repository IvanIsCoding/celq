// Test structure inspired by jaq's golden tests
// Source: https://github.com/01mf02/jaq/blob/3cf97ec33ccc4c6ca7c5bd29599a537bd5db2a70/jaq/tests/golden.rs
// jaq is licensed under the MIT License
// Copyright (c) 2021 Michael Färber
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Shared helpers and the `test!` macro used by the per-format golden test files.
// This module is intentionally free of any `#[cfg(feature = ...)]` gates so that it
// compiles under every feature set; the gates live on the individual tests instead.
//
// Because `base.rs` is compiled once as its own (test-less) binary and again as a
// module inside each per-format file, individual helpers are unused in some of those
// compilations. Suppress those expected warnings rather than gating the helpers.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
use std::io;
use std::process;
use std::str;

pub(crate) fn golden_test(args: &[&str], input: &str, out_ex: &str) -> io::Result<()> {
    let mut child = process::Command::new(env!("CARGO_BIN_EXE_celq"))
        .args(args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()?;

    use io::Write;
    // Write input and explicitly drop stdin to close it
    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(input.as_bytes())?;
        // stdin is dropped here, closing the pipe
        drop(stdin);
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        eprintln!("Process failed with status: {}", output.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Test failed");
    }

    let out_act = str::from_utf8(&output.stdout).expect("invalid UTF-8 in output");
    // remove '\r' from output for compatibility with Windows
    let out_act = out_act.replace('\r', "");
    if out_ex.trim() != out_act.trim() {
        println!("Expected output:\n{}\n---", out_ex);
        println!("Actual output:\n{}\n---", out_act);
        panic!("Output mismatch");
    }
    Ok(())
}

pub(crate) fn golden_test_failure(args: &[&str], input: &str, err_ex: &str) -> io::Result<()> {
    let mut child = process::Command::new(env!("CARGO_BIN_EXE_celq"))
        .args(args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()?;

    use io::Write;
    {
        let mut stdin = child.stdin.take().unwrap();
        // Some failures happen during argument parsing before stdin is read.
        // In that case the child may exit early and close the pipe first.
        match stdin.write_all(input.as_bytes()) {
            Ok(()) => {}
            Err(err) if err.kind() == io::ErrorKind::BrokenPipe => {}
            Err(err) => return Err(err),
        }
        drop(stdin);
    }

    let output = child.wait_with_output()?;

    assert!(
        !output.status.success(),
        "Expected failure, but process succeeded with stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let err_act = str::from_utf8(&output.stderr).expect("invalid UTF-8 in stderr");
    let err_act = err_act.replace('\r', "");
    assert!(
        err_act.contains(err_ex),
        "Expected stderr to contain:\n{}\n---\nActual stderr:\n{}\n---",
        err_ex,
        err_act
    );

    Ok(())
}

// Drives a golden test through `crate::base::golden_test`. The body uses absolute
// paths so each consumer only needs `use base::test;` to invoke it.
macro_rules! test {
    ($name:ident, $args:expr, $input:expr, $output:expr) => {
        #[test]
        fn $name() -> std::io::Result<()> {
            $crate::base::golden_test($args, $input, $output)
        }
    };
}
pub(crate) use test;
