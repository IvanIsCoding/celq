// Feature inspired by the gron CLI
// Source: https://github.com/tomnomnom/gron
// gron is licensed under the MIT License
// Copyright (c) 2016 Tom Hudson
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
use resast::prelude::*;
use ressa::Parser;
use serde_json::Value as JsonValue;
use std::fmt::Write;

/// Convert a JSON value to gron format
pub fn json_to_gron(value: &JsonValue) -> String {
    let mut output = String::new();
    let mut path = String::from("json");
    gron_recursive(value, &mut path, &mut output);
    output
}

fn gron_recursive(value: &JsonValue, path: &mut String, output: &mut String) {
    match value {
        JsonValue::Null => {
            writeln!(output, "{} = null;", path).unwrap();
        }
        JsonValue::Bool(b) => {
            writeln!(output, "{} = {};", path, b).unwrap();
        }
        JsonValue::Number(n) => {
            writeln!(output, "{} = {};", path, n).unwrap();
        }
        JsonValue::String(s) => {
            writeln!(output, "{} = {};", path, escape_string(s)).unwrap();
        }
        JsonValue::Array(arr) => {
            writeln!(output, "{} = [];", path).unwrap();
            for (i, item) in arr.iter().enumerate() {
                let prefix_len = path.len();
                write!(path, "[{}]", i).unwrap();
                gron_recursive(item, path, output);
                path.truncate(prefix_len);
            }
        }
        JsonValue::Object(obj) => {
            writeln!(output, "{} = {{}};", path).unwrap();
            for (key, val) in obj.iter() {
                let prefix_len = path.len();
                append_path_segment(path, key);
                gron_recursive(val, path, output);
                path.truncate(prefix_len);
            }
        }
    }
}

/// Check if a string is a valid JavaScript identifier
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // 1. Quick check: identifiers cannot contain whitespace or newlines
    if s.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return false;
    }

    let test_code = format!("json.{}", s);

    // If the parser errors, we should escape in quotes
    if let Ok(mut parser) = Parser::new(&test_code) {
        // This is a good sign, but we need to ensure the entire input was consumed
        if let Some(Ok(ProgramPart::Stmt(Stmt::Expr(Expr::Member(_))))) = parser.next() {
            // Guard against tokens such as newline
            return parser.next().is_none();
        }
    }

    false
}

/// Append a path segment to the existing path buffer, using dot notation for valid identifiers,
/// bracket notation with quotes for everything else
fn append_path_segment(path: &mut String, key: &str) {
    if is_valid_identifier(key) {
        write!(path, ".{}", key).unwrap();
    } else {
        write!(path, "[{}]", escape_string(key)).unwrap();
    }
}

/// Escape a string for use in gron output (both for keys and values)
fn escape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 2);
    result.push('"');

    for ch in s.chars() {
        match ch {
            '"' => result.push_str(r#"\""#),
            '\\' => result.push_str(r"\\"),
            '\n' => result.push_str(r"\n"),
            '\r' => result.push_str(r"\r"),
            '\t' => result.push_str(r"\t"),
            '\x08' => result.push_str(r"\b"),
            '\x0C' => result.push_str(r"\f"),
            c if c.is_control() => {
                write!(result, "\\u{:04x}", c as u32).unwrap();
            }
            c => result.push(c),
        }
    }

    result.push('"');
    result
}

#[cfg(test)]
#[path = "gron_test.rs"]
mod tests;
