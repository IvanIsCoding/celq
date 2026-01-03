
use super::*;
use cel::Program;

#[test]
fn test_handle_json_null_input() {
    let program = Program::compile("2 + 3").unwrap();
    let args = BTreeMap::new();

    let (output, is_truthy) = handle_json(&program, &args, None).unwrap();

    assert!(output.contains("5"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_with_json() {
    let program = Program::compile(".x + .y").unwrap();
    let args = BTreeMap::new();
    let json = r#"{"x": 10, "y": 20}"#;

    let (output, is_truthy) = handle_json(&program, &args, Some(json)).unwrap();

    assert!(output.contains("30"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_with_args() {
    let program = Program::compile("x + y").unwrap();
    let mut args = BTreeMap::new();
    args.insert("x".to_string(), CelValue::Int(5));
    args.insert("y".to_string(), CelValue::Int(7));

    let (output, is_truthy) = handle_json(&program, &args, None).unwrap();

    assert!(output.contains("12"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_args_and_json() {
    let program = Program::compile("x + .value").unwrap();
    let mut args = BTreeMap::new();
    args.insert("x".to_string(), CelValue::Int(100));
    let json = r#"{"value": 50}"#;

    let (output, is_truthy) = handle_json(&program, &args, Some(json)).unwrap();

    assert!(output.contains("150"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_boolean_false() {
    let program = Program::compile("2 > 5").unwrap();
    let args = BTreeMap::new();

    let (output, is_truthy) = handle_json(&program, &args, None).unwrap();

    assert!(output.contains("false"));
    assert!(!is_truthy);
}

#[test]
fn test_handle_json_boolean_true() {
    let program = Program::compile("5 > 2").unwrap();
    let args = BTreeMap::new();

    let (output, is_truthy) = handle_json(&program, &args, None).unwrap();

    assert!(output.contains("true"));
    assert!(is_truthy);
}

#[test]
fn test_handle_json_truthiness_zero() {
    let program = Program::compile("0").unwrap();
    let args = BTreeMap::new();

    let (_output, is_truthy) = handle_json(&program, &args, None).unwrap();

    assert!(!is_truthy);
}

#[test]
fn test_handle_json_truthiness_empty_string() {
    let program = Program::compile(r#""""#).unwrap();
    let args = BTreeMap::new();

    let (_output, is_truthy) = handle_json(&program, &args, None).unwrap();

    assert!(!is_truthy);
}

#[test]
fn test_handle_json_invalid_json() {
    let program = Program::compile(".x").unwrap();
    let args = BTreeMap::new();
    let json = r#"not valid json"#;

    let result = handle_json(&program, &args, Some(json));

    assert!(result.is_err());
}

#[test]
fn test_handle_json_missing_variable() {
    let program = Program::compile("missing_var").unwrap();
    let args = BTreeMap::new();

    let result = handle_json(&program, &args, None);

    assert!(result.is_err());
}

#[test]
fn test_handle_buffer_single_line() {
    let program = Program::compile(".x").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 42}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);

    let results = handle_buffer(&program, &args, reader, false, -1).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].0.contains("42"));
    assert!(results[0].1);
}

#[test]
fn test_handle_buffer_multiple_lines() {
    let program = Program::compile(".x").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 1}
{"x": 2}
{"x": 3}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);

    let results = handle_buffer(&program, &args, reader, false, -1).unwrap();

    assert_eq!(results.len(), 3);
    assert!(results[0].0.contains("1"));
    assert!(results[1].0.contains("2"));
    assert!(results[2].0.contains("3"));
}

#[test]
fn test_handle_buffer_slurp() {
    let program = Program::compile(".x + .y").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 10,
"y": 20}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);

    let results = handle_buffer(&program, &args, reader, true, -1).unwrap();

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

    let results = handle_buffer(&program, &args, reader, false, -1).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].0.contains("5"));
    assert!(results[0].1);
}

#[test]
fn test_handle_input_null_input() {
    let program = Program::compile("2 + 3").unwrap();
    let args = BTreeMap::new();

    let results = handle_input(&program, &args, true, false, -1).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].0.contains("5"));
    assert!(results[0].1);
}

#[test]
fn test_handle_buffer_skip_empty_lines() {
    let program = Program::compile(".x").unwrap();
    let args = BTreeMap::new();
    let input = r#"{"x": 1}

{"x": 2}
   
{"x": 3}"#;
    let cursor = Cursor::new(input.as_bytes());
    let reader = BufReader::new(cursor);

    let results = handle_buffer(&program, &args, reader, false, -1).unwrap();

    assert_eq!(results.len(), 3);
    assert!(results[0].0.contains("1"));
    assert!(results[1].0.contains("2"));
    assert!(results[2].0.contains("3"));
}
