use super::*;
use cel::Program;
use cel::objects::Key;
use std::collections::HashMap;
use std::sync::Arc;

fn default_params() -> InputParameters {
    InputParameters {
        root_var: "this".to_string(),
        null_input: false,
        input_format: InputFormat::Json,
        parallelism: -1,
        sort_keys: false,
        pretty_print: false,
        raw_output: false,
        greppable: false,
        no_extensions: false,
    }
}

#[test]
fn test_handle_json_null_input() {
    let program = Program::compile("2 + 3").unwrap();
    let args = BTreeMap::new();
    let params = default_params();

    let (output, is_truthy) = handle_json(&program, &args, &params, None).unwrap();

    assert!(output.contains("5"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_with_json() {
    let program = Program::compile("this.x + this.y").unwrap();
    let args = BTreeMap::new();
    let json = r#"{"x": 10, "y": 20}"#;
    let params = default_params();

    let (output, is_truthy) = handle_json(&program, &args, &params, Some(json)).unwrap();

    assert!(output.contains("30"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_with_args() {
    let program = Program::compile("x + y").unwrap();
    let mut args = BTreeMap::new();
    args.insert("x".to_string(), CelValue::Int(5));
    args.insert("y".to_string(), CelValue::Int(7));
    let params = default_params();

    let (output, is_truthy) = handle_json(&program, &args, &params, None).unwrap();

    assert!(output.contains("12"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_input_overrides_arg_with_same_name() {
    let program = Program::compile("this.value").unwrap();
    let mut args = BTreeMap::new();
    args.insert("this".to_string(), CelValue::Int(999));
    let json = r#"{"value": 50}"#;
    let params = default_params();

    let (output, is_truthy) = handle_json(&program, &args, &params, Some(json)).unwrap();

    assert!(output.contains("50"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_args_and_json() {
    let program = Program::compile("x + this.value").unwrap();
    let mut args = BTreeMap::new();
    args.insert("x".to_string(), CelValue::Int(100));
    let json = r#"{"value": 50}"#;
    let params = default_params();

    let (output, is_truthy) = handle_json(&program, &args, &params, Some(json)).unwrap();

    assert!(output.contains("150"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_boolean_false() {
    let program = Program::compile("2 > 5").unwrap();
    let args = BTreeMap::new();
    let params = default_params();

    let (output, is_truthy) = handle_json(&program, &args, &params, None).unwrap();

    assert!(output.contains("false"));
    assert!(!is_truthy);
}

#[test]
fn test_handle_json_boolean_true() {
    let program = Program::compile("5 > 2").unwrap();
    let args = BTreeMap::new();
    let params = default_params();

    let (output, is_truthy) = handle_json(&program, &args, &params, None).unwrap();

    assert!(output.contains("true"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_truthiness_zero() {
    let program = Program::compile("0").unwrap();
    let args = BTreeMap::new();
    let params = default_params();

    let (_output, is_truthy) = handle_json(&program, &args, &params, None).unwrap();

    assert!(!is_truthy);
}

#[test]
fn test_handle_json_truthiness_empty_string() {
    let program = Program::compile(r#""""#).unwrap();
    let args = BTreeMap::new();
    let params = default_params();

    let (_output, is_truthy) = handle_json(&program, &args, &params, None).unwrap();

    assert!(!is_truthy);
}

#[test]
fn test_uint_truthiness() {
    assert!(!is_cel_value_truthy(&CelValue::UInt(0)));
    assert!(is_cel_value_truthy(&CelValue::UInt(1)));
}

#[test]
fn test_float_truthiness() {
    assert!(!is_cel_value_truthy(&CelValue::Float(0.0)));
    assert!(!is_cel_value_truthy(&CelValue::Float(f64::NAN)));
    assert!(is_cel_value_truthy(&CelValue::Float(1.0)));
}

#[test]
fn test_list_truthiness() {
    assert!(!is_cel_value_truthy(&CelValue::List(Arc::new(vec![]))));
    assert!(is_cel_value_truthy(&CelValue::List(Arc::new(vec![
        CelValue::Null,
    ]))));
}

#[test]
fn test_map_and_null_truthiness() {
    let empty_map = HashMap::<Key, CelValue>::new().into();
    assert!(!is_cel_value_truthy(&CelValue::Map(empty_map)));

    let mut populated_map = HashMap::new();
    populated_map.insert(Key::String(Arc::new("key".to_string())), CelValue::Null);
    assert!(is_cel_value_truthy(&CelValue::Map(populated_map.into())));

    assert!(!is_cel_value_truthy(&CelValue::Null));
}

#[test]
fn test_bytes_truthiness() {
    assert!(is_cel_value_truthy(&CelValue::Bytes(Arc::new(vec![]))));
    assert!(is_cel_value_truthy(&CelValue::Bytes(Arc::new(vec![0]))));
}

#[test]
fn test_handle_json_raw_output_for_string() {
    let program = Program::compile(r#""hello""#).unwrap();
    let args = BTreeMap::new();
    let mut params = default_params();
    params.raw_output = true;

    let (output, is_truthy) = handle_json(&program, &args, &params, None).unwrap();

    assert_eq!(output, "hello");
    assert!(is_truthy);
}

#[test]
fn test_handle_json_sorted_pretty_output() {
    let program = Program::compile(r#"{"z": {"b": 2, "a": 1}, "a": 0}"#).unwrap();
    let args = BTreeMap::new();
    let mut params = default_params();
    params.sort_keys = true;
    params.pretty_print = true;

    let (output, is_truthy) = handle_json(&program, &args, &params, None).unwrap();

    assert_eq!(
        output,
        "{\n  \"a\": 0,\n  \"z\": {\n    \"a\": 1,\n    \"b\": 2\n  }\n}"
    );
    assert!(is_truthy);
}

#[test]
fn test_sort_keys_recursive_descends_into_arrays() {
    let mut value = serde_json::json!({
        "items": [
            { "b": 2, "a": 1 },
            [{ "d": 4, "c": 3 }]
        ]
    });

    sort_keys_recursive(&mut value);

    assert_eq!(
        value.to_string(),
        r#"{"items":[{"a":1,"b":2},[{"c":3,"d":4}]]}"#
    );
}

#[test]
fn test_handle_json_invalid_json() {
    let program = Program::compile("this.x").unwrap();
    let args = BTreeMap::new();
    let json = r#"not valid json"#;
    let params = default_params();

    let result = handle_json(&program, &args, &params, Some(json));

    assert!(result.is_err());
}

#[test]
fn test_handle_json_missing_variable() {
    let program = Program::compile("missing_var").unwrap();
    let args = BTreeMap::new();
    let params = default_params();

    let result = handle_json(&program, &args, &params, None);

    assert!(result.is_err());
}

#[test]
fn test_handle_buffer_single_line() {
    let program = Program::compile("this.x").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 42}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);
    let params = default_params();

    let results = handle_buffer(&program, &args, &params, reader).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].0.contains("42"));
    assert!(results[0].1);
}

#[test]
fn test_handle_buffer_multiple_lines() {
    let program = Program::compile("this.x").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 1}
{"x": 2}
{"x": 3}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);
    let params = default_params();

    let results = handle_buffer(&program, &args, &params, reader).unwrap();

    assert_eq!(results.len(), 3);
    assert!(results[0].0.contains("1"));
    assert!(results[1].0.contains("2"));
    assert!(results[2].0.contains("3"));
}

#[test]
fn test_handle_buffer_slurp() {
    let program = Program::compile("this[0].x + this[1].x").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 10}
{"x": 20}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);
    let mut params = default_params();
    params.input_format = InputFormat::SlurpJson;

    let results = handle_buffer(&program, &args, &params, reader).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].0.contains("30"));
    assert!(results[0].1);
}

#[test]
fn test_handle_buffer_empty_input() {
    let program = Program::compile("2 + 3").unwrap();
    let args = BTreeMap::new();
    let cursor = Cursor::new(Vec::<u8>::new());
    let reader = BufReader::new(cursor);
    let params = default_params();

    let results = handle_buffer(&program, &args, &params, reader).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].0.contains("5"));
    assert!(results[0].1);
}

#[test]
fn test_handle_input_null_input() {
    let program = Program::compile("2 + 3").unwrap();
    let args = BTreeMap::new();
    let mut params = default_params();
    params.null_input = true;

    let results = handle_input(&program, &args, &params).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].0.contains("5"));
    assert!(results[0].1);
}

#[test]
fn test_handle_buffer_skip_empty_lines() {
    let program = Program::compile("this.x").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 1}

{"x": 2}
   
{"x": 3}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);
    let params = default_params();

    let results = handle_buffer(&program, &args, &params, reader).unwrap();

    assert_eq!(results.len(), 3);
    assert!(results[0].0.contains("1"));
    assert!(results[1].0.contains("2"));
    assert!(results[2].0.contains("3"));
}

#[test]
fn test_handle_buffer_custom_root_var() {
    let program = Program::compile("request.x").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 42}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);
    let mut params = default_params();
    params.root_var = "request".to_string();

    let results = handle_buffer(&program, &args, &params, reader).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, "42");
    assert!(results[0].1);
}

#[test]
fn test_handle_buffer_rejects_zero_parallelism() {
    let program = Program::compile("this.x").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 42}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);
    let mut params = default_params();
    params.parallelism = 0;

    let err = handle_buffer(&program, &args, &params, reader)
        .unwrap_err()
        .to_string();

    assert!(err.contains("Parallelism level cannot be 0"));
}

#[test]
#[cfg(not(feature = "greppable"))]
fn test_handle_json_greppable_disabled() {
    let program = Program::compile("this").unwrap();
    let args = BTreeMap::new();
    let mut params = default_params();
    params.greppable = true;

    let err = handle_json(&program, &args, &params, Some(r#"{"x": 42}"#))
        .unwrap_err()
        .to_string();

    assert!(err.contains("Binary was compiled without greppable support"));
}
