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
use std::env;
use std::fs;
use std::io;
use std::process;
use std::str;
use tempfile::NamedTempFile;

fn golden_test(args: &[&str], input: &str, out_ex: &str) -> io::Result<()> {
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

macro_rules! test {
    ($name:ident, $args:expr, $input:expr, $output:expr) => {
        #[test]
        fn $name() -> io::Result<()> {
            golden_test($args, $input, $output)
        }
    };
}

// Basic arithmetic and literal tests
test!(literal_int, &["42"], "{}", "42");
test!(literal_string, &[r#""hello""#], "{}", r#""hello""#);
test!(literal_bool_true, &["true"], "{}", "true");
test!(literal_bool_false, &["false"], "{}", "false");
test!(arithmetic_add, &["2 + 3"], "{}", "5");
test!(arithmetic_subtract, &["10 - 4"], "{}", "6");
test!(arithmetic_multiply, &["6 * 7"], "{}", "42");
test!(arithmetic_divide, &["20 / 4"], "{}", "5");
test!(arithmetic_modulo, &["17 % 5"], "{}", "2");

// String operations
test!(
    string_concat,
    &[r#""hello" + " " + "world""#],
    "{}",
    r#""hello world""#
);
test!(string_size, &[r#"size("hello")"#], "{}", "5");
test!(
    string_contains,
    &[r#""hello".contains("ell")"#],
    "{}",
    "true"
);
test!(
    string_startswith,
    &[r#""hello".startsWith("hel")"#],
    "{}",
    "true"
);
test!(
    string_endswith,
    &[r#""hello".endsWith("lo")"#],
    "{}",
    "true"
);

// Raw output
test!(
    string_raw_output,
    &["--raw-output", r#""hello" + " " + "world""#],
    "{}",
    r#"hello world"#
);

// Logical operations
test!(logical_and_true, &["true && true"], "{}", "true");
test!(logical_and_false, &["true && false"], "{}", "false");
test!(logical_or_true, &["false || true"], "{}", "true");
test!(logical_or_false, &["false || false"], "{}", "false");
test!(logical_not, &["!false"], "{}", "true");

// Comparison operations
test!(compare_eq, &["5 == 5"], "{}", "true");
test!(compare_neq, &["5 != 3"], "{}", "true");
test!(compare_lt, &["3 < 5"], "{}", "true");
test!(compare_lte, &["5 <= 5"], "{}", "true");
test!(compare_gt, &["7 > 3"], "{}", "true");
test!(compare_gte, &["5 >= 5"], "{}", "true");

// List operations
test!(list_literal, &["[1, 2, 3]"], "{}", "[1,2,3]");
test!(list_size, &["size([1, 2, 3, 4])"], "{}", "4");
test!(list_in, &["2 in [1, 2, 3]"], "{}", "true");
test!(list_index, &["[10, 20, 30][1]"], "{}", "20");
test!(
    list_no_key_sorting,
    &["-S", "[30, 20, 10]"],
    "{}",
    "[30,20,10]"
);

// Map operations
test!(
    map_literal,
    &["-S", r#"{"a": 1, "b": 2}"#],
    "{}",
    r#"{"a":1,"b":2}"#
);
test!(
    map_nested_sorted,
    &["-S", r#"{"person": {"name": "Alice", "age": 30}, "id": 1}"#],
    "{}",
    r#"{"id":1,"person":{"age":30,"name":"Alice"}}"#
);
test!(map_access, &[r#"{"a": 1, "b": 2}["a"]"#], "{}", "1");
test!(
    map_dot_access,
    &[r#"{"name": "Alice"}.name"#],
    "{}",
    r#""Alice""#
);
test!(map_size, &[r#"size({"a": 1, "b": 2, "c": 3})"#], "{}", "3");
test!(map_in, &[r#""a" in {"a": 1, "b": 2}"#], "{}", "true");

// Conditional (ternary) operator
test!(ternary_true, &["true ? 1 : 2"], "{}", "1");
test!(ternary_false, &["false ? 1 : 2"], "{}", "2");
test!(
    ternary_expr,
    &["5 > 3 ? \"yes\" : \"no\""],
    "{}",
    r#""yes""#
);

// Arguments: string type
test!(
    arg_string,
    &["--arg", "name:string=Alice", "name"],
    "{}",
    r#""Alice""#
);

test!(
    arg_string_concat,
    &[
        "--arg",
        "first:string=Hello",
        "--arg",
        "second:string=World",
        "first + \" \" + second"
    ],
    "{}",
    r#""Hello World""#
);

// Arguments: int type
test!(arg_int, &["--arg", "x:int=42", "x * 2"], "{}", "84");

test!(
    arg_int_math,
    &["--arg", "a:int=10", "--arg", "b:int=5", "a + b"],
    "{}",
    "15"
);

// Arguments: bool type
test!(
    arg_bool_true,
    &["--arg", "flag:bool=true", "flag ? 1 : 0"],
    "{}",
    "1"
);

test!(
    arg_bool_false,
    &["--arg", "flag:bool=false", "flag || true"],
    "{}",
    "true"
);

// Arguments: float type
test!(
    arg_float,
    &["--arg", "pi:float=3.14159", "pi * 2.0"],
    "{}",
    "6.28318"
);

// JSON input - accessing fields
test!(
    json_input_field,
    &["this.name"],
    r#"{"name":"Alice","age":30}"#,
    r#""Alice""#
);

test!(
    json_input_nested,
    &["this.person.name"],
    r#"{"person":{"name":"Bob","age":25}}"#,
    r#""Bob""#
);

test!(
    json_input_array,
    &["this.items[0]"],
    r#"{"items":[1,2,3]}"#,
    "1"
);

test!(
    json_input_expression,
    &["this.x + this.y"],
    r#"{"x":10,"y":20}"#,
    "30"
);

// Newline-Delimited JSON (NDJSON)
test!(
    ndjson_multi_line,
    &["this.value * 2"],
    r#"{"value":1}
{"value":2}
{"value":3}"#,
    "2\n4\n6"
);

test!(
    ndjson_filter,
    &["this.age > 25"],
    r#"{"name":"Alice","age":30}
{"name":"Bob","age":20}
{"name":"Charlie","age":35}"#,
    "true\nfalse\ntrue"
);

// Newline-Delimited JSON (NDJSON) + Parallelism
test!(
    ndjson_multi_line_parallel,
    &["-j", "2", "this.value * 2"],
    r#"{"value":1}
{"value":2}
{"value":3}"#,
    "2\n4\n6"
);

test!(
    ndjson_filter_parallel,
    &["-j", "2", "this.age > 25"],
    r#"{"name":"Alice","age":30}
{"name":"Bob","age":20}
{"name":"Charlie","age":35}"#,
    "true\nfalse\ntrue"
);

// Multi-line JSON (pretty-printed)
test!(
    multiline_json_object,
    &["this.a + ' and ' + string(this.c)"],
    r#"{
  "a": "b",
  "c": "d"
}"#,
    "\"b and d\""
);

test!(
    multiline_json_array,
    &["this.map(x, x * 2)"],
    r#"[
  1,
  2,
  3,
  4,
  5
]"#,
    "[2,4,6,8,10]"
);

test!(
    multiline_json_empty_first_line,
    &["this.x + this.y"],
    r#"
{"x": 5, "y": 10}"#,
    "15"
);

test!(
    multiline_json_closing_brace_last,
    &["this.foo"],
    r#"{"foo": "bar"
}"#,
    "\"bar\""
);

// Null input mode
test!(null_input_literal, &["-n", "42"], "", "42");

test!(null_input_computation, &["-n", "5 * 5 + 3"], "", "28");

test!(
    null_input_string,
    &["-n", r#""computed value""#],
    "",
    r#""computed value""#
);

// Null input with arguments
test!(
    null_input_with_args,
    &["-n", "--arg", "x:int=10", "--arg", "y:int=20", "x + y"],
    "",
    "30"
);

// Slurp mode - treat all input as single JSON array
test!(
    slurp_mode,
    &["-s", "size(this)"],
    r#"{"id":1}
{"id":2}
{"id":3}"#,
    "3"
);

test!(
    slurp_sum,
    &["-s", "this[0].value + this[1].value + this[2].value"],
    r#"{"value":10}
{"value":20}
{"value":30}"#,
    "60"
);

// Combining arguments with JSON input
test!(
    args_and_json,
    &["--arg", "threshold:int=25", "this.age > threshold"],
    r#"{"name":"Alice","age":30}"#,
    "true"
);

test!(
    args_and_json_string,
    &["--arg", "prefix:string=Hello, ", "prefix + this.name"],
    r#"{"name":"World"}"#,
    r#""Hello, World""#
);

// Complex expressions
test!(
    complex_nested,
    &["(this.a + this.b) * this.c"],
    r#"{"a":2,"b":3,"c":4}"#,
    "20"
);

test!(
    complex_ternary_with_json,
    &["this.score >= 60 ? \"pass\" : \"fail\""],
    r#"{"score":75}"#,
    r#""pass""#
);

test!(
    complex_list_comprehension,
    &["[1, 2, 3].map(x, x * 2)"],
    "{}",
    "[2,4,6]"
);

// has() macro for checking field existence
test!(
    has_field_true,
    &["has(this.name)"],
    r#"{"name":"Alice"}"#,
    "true"
);

test!(
    has_field_false,
    &["has(this.missing)"],
    r#"{"name":"Alice"}"#,
    "false"
);

// Duration operations (if supported)
test!(
    duration_seconds,
    &["duration(\"1h\") > duration(\"30m\")"],
    "{}",
    "true"
);

// Combining multiple features
test!(
    multi_feature_combo,
    &[
        "-n",
        "--arg",
        "multiplier:int=3",
        "--arg",
        "offset:int=10",
        "multiplier * 5 + offset"
    ],
    "",
    "25"
);

// Renaming root variable
test!(
    rename_root_variable,
    &["-R=request", "(request.a + request.b) * request.c"],
    r#"{"a":2,"b":3,"c":4}"#,
    "20"
);

test!(
    rename_root_variable_with_args,
    &[
        "--root-var=request",
        "--arg=c:int=4",
        "(request.a + request.b) * c"
    ],
    r#"{"a":2,"b":3}"#,
    "20"
);

// Multi-line JSON5 (with trailing comma and comment)
test!(
    multiline_json5_object,
    &["--from-json5", "this.a + ' and ' + string(this.c)"],
    r#"{
  // This is a comment
  "a": "b",
  "c": "d",
}"#,
    "\"b and d\""
);

test!(
    multiline_json5_array,
    &["--from-json5", "this.map(x, x * 2)"],
    r#"[
  1,
  2,
  3,
  4,
  5, // trailing comma
]"#,
    "[2,4,6,8,10]"
);

test!(
    multiline_json5_empty_first_line,
    &["--from-json5", "this.x + this.y"],
    r#"
{x: 5, y: 10}"#, // unquoted keys
    "15"
);

test!(
    multiline_json5_closing_brace_last,
    &["--from-json5", "this.foo"],
    r#"{foo: "bar", // comment and trailing comma
}"#,
    "\"bar\""
);

// TOML tests

// TOML with nested tables
#[cfg(feature = "from-toml")]
test!(
    toml_nested_table,
    &[
        "--from-toml",
        "this.database.host + ':' + string(this.database.port)"
    ],
    r#"[database]
host = "localhost"
port = 5432
# Database configuration
"#,
    "\"localhost:5432\""
);

#[cfg(feature = "from-toml")]
test!(
    toml_array_of_tables,
    &["--from-toml", "this.servers.map(s, s.ip)"],
    r#"[[servers]]
ip = "192.168.1.1"
name = "alpha"

[[servers]]
ip = "192.168.1.2"
name = "beta"
"#,
    "[\"192.168.1.1\",\"192.168.1.2\"]"
);

#[cfg(feature = "from-toml")]
test!(
    toml_dotted_keys,
    &[
        "--from-toml",
        "this.user.name + ' <' + this.user.email + '>'"
    ],
    r#"user.name = "Alice"
user.email = "alice@example.com"
"#,
    "\"Alice <alice@example.com>\""
);

#[cfg(feature = "from-toml")]
test!(
    toml_inline_table,
    &["--from-toml", "this.point.x + this.point.y"],
    r#"point = { x = 10, y = 20 }
"#,
    "30"
);

// Pretty-printing output
test!(
    map_nested_pretty,
    &[
        "-S",
        "-p",
        r#"{"person": {"name": "Alice", "age": 30}, "id": 1}"#
    ],
    "{}",
    r#"{
  "id": 1,
  "person": {
    "age": 30,
    "name": "Alice"
  }
}"#
);

test!(
    json5_to_pretty_sorted_json,
    &["--from-json5", "--sort-keys", "--pretty-print", "this"],
    r#"{
  // Input with JSON5 features
  person: {
    name: "Alice",
    age: 30,
  },
  id: 1, // trailing comma
}"#,
    r#"{
  "id": 1,
  "person": {
    "age": 30,
    "name": "Alice"
  }
}"#
);

// YAML tests

// YAML with nested mappings
#[cfg(feature = "from-yaml")]
test!(
    yaml_nested_mapping,
    &[
        "--from-yaml",
        "this.database.host + ':' + string(this.database.port)"
    ],
    r#"database:
  host: localhost
  port: 5432
  # Database configuration
"#,
    "\"localhost:5432\""
);

#[cfg(feature = "from-yaml")]
test!(
    yaml_array_of_mappings,
    &["--from-yaml", "this.servers.map(s, s.ip)"],
    r#"servers:
  - ip: 192.168.1.1
    name: alpha
  - ip: 192.168.1.2
    name: beta
"#,
    "[\"192.168.1.1\",\"192.168.1.2\"]"
);

#[cfg(feature = "from-yaml")]
test!(
    yaml_nested_keys,
    &[
        "--from-yaml",
        "this.user.name + ' <' + this.user.email + '>'"
    ],
    r#"user:
  name: Alice
  email: alice@example.com
"#,
    "\"Alice <alice@example.com>\""
);

#[cfg(feature = "from-yaml")]
test!(
    yaml_flow_mapping,
    &["--from-yaml", "this.point.x + this.point.y"],
    r#"point: {x: 10, y: 20}
"#,
    "30"
);

// Multi-document YAML
#[cfg(feature = "from-yaml")]
test!(
    yaml_multi_document_with_separator,
    &["--from-yaml", "this.map(doc, doc.name)"],
    r#"name: Alice
age: 30
---
name: Bob
age: 25
---
name: Charlie
age: 35
"#,
    "[\"Alice\",\"Bob\",\"Charlie\"]"
);

#[cfg(feature = "from-yaml")]
test!(
    yaml_multi_document_with_terminator,
    &["--from-yaml", "this.map(doc, doc.value)"],
    r#"value: first
...
---
value: second
...
---
value: third
"#,
    "[\"first\",\"second\",\"third\"]"
);

// Void test
test!(void_mode, &["--void", "-n", "2 + 2"], "", "");

// From file tests
#[test]
fn from_file_simple_expression() -> io::Result<()> {
    let file = NamedTempFile::new()?;
    fs::write(file.path(), "2 + 3 * 4")?;

    let path = file.path().to_str().expect("non-utf8 temp path");

    golden_test(&["-f", path, "-n"], "", "14")
}

#[test]
fn from_file_multiline_expression() -> io::Result<()> {
    let file = NamedTempFile::new()?;
    fs::write(
        file.path(),
        r#"
            (
                this.a +
                this.b
            ) * this.c
        "#,
    )?;

    let path = file.path().to_str().expect("non-utf8 temp path");

    golden_test(&["--from-file", path], r#"{"a":1, "b":2, "c":3}"#, "9")
}

// Greppable output tests
#[cfg(feature = "greppable")]
test!(
    greppable_simple,
    &["-g", "-S", "this"],
    r#"{"name":"Alice","age":30}"#,
    r#"json = {};
json.age = 30;
json.name = "Alice";
"#
);

#[cfg(feature = "greppable")]
test!(
    greppable_nested,
    &["--greppable", "-S", "this"],
    r#"{"person":{"name":"Bob","city":"NYC"},"id":1}"#,
    r#"json = {};
json.id = 1;
json.person = {};
json.person.city = "NYC";
json.person.name = "Bob";
"#
);

#[cfg(feature = "greppable")]
test!(
    greppable_json5,
    &["--greppable", "-S", "--from-json5", "this"],
    r#"{
  // Comment
  x: 10,
  y: 20,
}"#,
    r#"json = {};
json.x = 10;
json.y = 20;
"#
);

// Ungron (from-gron) tests
#[cfg(feature = "greppable")]
test!(
    from_gron_simple,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.age = 30;
json.name = "Alice";
"#,
    r#"{"age":30,"name":"Alice"}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_nested,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.id = 1;
json.person = {};
json.person.city = "NYC";
json.person.name = "Bob";
"#,
    r#"{"id":1,"person":{"city":"NYC","name":"Bob"}}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_arrays,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.items = [];
json.items[0] = "first";
json.items[1] = "second";
json.items[2] = "third";
"#,
    r#"{"items":["first","second","third"]}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_sparse_array,
    &["--from-gron", "-S", "this"],
    r#"json.likes = [];
json.likes[0] = "code";
json.likes[2] = "meat";
"#,
    r#"{"likes":["code",null,"meat"]}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_mixed_types,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.integer = 42;
json.float = 3.14;
json.negative = -5;
json.isTrue = true;
json.isFalse = false;
json.nothing = null;
"#,
    r#"{"float":3.14,"integer":42,"isFalse":false,"isTrue":true,"negative":-5,"nothing":null}"#
);

// Tricky edge cases
#[cfg(feature = "greppable")]
test!(
    from_gron_special_keys,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json["key-with-dashes"] = "value1";
json["key.with.dots"] = "value2";
json["key with spaces"] = "value3";
json["123numeric"] = "value4";
"#,
    r#"{"123numeric":"value4","key with spaces":"value3","key-with-dashes":"value1","key.with.dots":"value2"}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_deeply_nested,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.a = {};
json.a.b = {};
json.a.b.c = {};
json.a.b.c.d = [];
json.a.b.c.d[0] = {};
json.a.b.c.d[0].e = "deep";
"#,
    r#"{"a":{"b":{"c":{"d":[{"e":"deep"}]}}}}"#
);

// Slice extension tests
test!(
    slice_basic,
    &["this.items.slice(1, 3)"],
    r#"{"items":[1,2,3,4,5]}"#,
    "[2,3]"
);

test!(
    slice_negative_indices,
    &["this.items.slice(-3, -1)"],
    r#"{"items":[10,20,30,40,50]}"#,
    "[30,40]"
);

test!(
    slice_from_start,
    &["this.items.slice(0, 2)"],
    r#"{"items":["a","b","c","d"]}"#,
    r#"["a","b"]"#
);

test!(
    slice_beyond_bounds,
    &["this.items.slice(2, 100)"],
    r#"{"items":[1,2,3]}"#,
    "[3]"
);

test!(
    slice_negative_to_positive,
    &["this.items.slice(-5, 4)"],
    r#"{"items":[5,10,15,20,25,30]}"#,
    "[10,15,20]"
);

// Default expr test
test!(
    default_expression,
    &[],
    r#"{"value":100}"#,
    r#"{"value":100}"#
);

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
