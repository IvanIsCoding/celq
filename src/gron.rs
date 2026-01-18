use resast::prelude::*;
use ressa::Parser;
use serde_json::Value as JsonValue;
use std::fmt::Write;

/// Convert a JSON value to gron format
pub fn json_to_gron(value: &JsonValue) -> String {
    let mut output = String::new();
    gron_recursive(value, "json", &mut output);
    output
}

fn gron_recursive(value: &JsonValue, path: &str, output: &mut String) {
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
                let new_path = format!("{}[{}]", path, i);
                gron_recursive(item, &new_path, output);
            }
        }
        JsonValue::Object(obj) => {
            writeln!(output, "{} = {{}};", path).unwrap();
            for (key, val) in obj.iter() {
                let new_path = format_path_segment(path, key);
                gron_recursive(val, &new_path, output);
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

/// Format a path segment, using dot notation for valid identifiers,
/// bracket notation with quotes for everything else
fn format_path_segment(current_path: &str, key: &str) -> String {
    if is_valid_identifier(key) {
        format!("{}.{}", current_path, key)
    } else {
        format!("{}[{}]", current_path, escape_string(key))
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
