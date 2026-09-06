use super::*;

#[test]
fn test_int() {
    let args = vec![("x".to_string(), "int".to_string(), "42".to_string())];
    let vars = args_to_cel_variables(&args).unwrap();
    assert!(matches!(vars.get("x").unwrap(), CelValue::Int(42)));
}

#[test]
fn test_uint() {
    let args = vec![("x".to_string(), "uint".to_string(), "42".to_string())];
    let vars = args_to_cel_variables(&args).unwrap();
    assert!(matches!(vars.get("x").unwrap(), CelValue::UInt(42)));
}

#[test]
fn test_float() {
    let args = vec![("x".to_string(), "float".to_string(), "1.23".to_string())];
    let vars = args_to_cel_variables(&args).unwrap();
    if let CelValue::Float(f) = vars.get("x").unwrap() {
        assert!((f - 1.23).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_string() {
    let args = vec![("x".to_string(), "string".to_string(), "hello".to_string())];
    let vars = args_to_cel_variables(&args).unwrap();
    if let CelValue::String(s) = vars.get("x").unwrap() {
        assert_eq!(s.as_str(), "hello");
    } else {
        panic!("Expected string");
    }
}

#[test]
fn test_bool() {
    let args = vec![("x".to_string(), "bool".to_string(), "true".to_string())];
    let vars = args_to_cel_variables(&args).unwrap();
    assert!(matches!(vars.get("x").unwrap(), CelValue::Bool(true)));
}

#[test]
fn test_alias_types() {
    let args = vec![
        ("x".to_string(), "i64".to_string(), "42".to_string()),
        ("y".to_string(), "u64".to_string(), "7".to_string()),
        ("z".to_string(), "double".to_string(), "2.5".to_string()),
        ("w".to_string(), "boolean".to_string(), "false".to_string()),
        ("s".to_string(), "str".to_string(), "hello".to_string()),
    ];
    let vars = args_to_cel_variables(&args).unwrap();

    assert!(matches!(vars.get("x").unwrap(), CelValue::Int(42)));
    assert!(matches!(vars.get("y").unwrap(), CelValue::UInt(7)));
    assert!(matches!(vars.get("z").unwrap(), CelValue::Float(f) if (*f - 2.5).abs() < 0.001));
    assert!(matches!(vars.get("w").unwrap(), CelValue::Bool(false)));
    assert!(matches!(vars.get("s").unwrap(), CelValue::String(s) if s.as_str() == "hello"));
}

#[test]
fn test_duplicate_args_last_wins() {
    let args = vec![
        ("x".to_string(), "int".to_string(), "1".to_string()),
        ("x".to_string(), "int".to_string(), "2".to_string()),
    ];
    let vars = args_to_cel_variables(&args).unwrap();
    assert_eq!(vars.len(), 1);
    assert!(matches!(vars.get("x").unwrap(), CelValue::Int(2)));
}

#[test]
fn test_multiple_args() {
    let args = vec![
        ("x".to_string(), "int".to_string(), "10".to_string()),
        ("y".to_string(), "string".to_string(), "test".to_string()),
        ("z".to_string(), "bool".to_string(), "false".to_string()),
    ];
    let vars = args_to_cel_variables(&args).unwrap();
    assert_eq!(vars.len(), 3);
    assert!(matches!(vars.get("x").unwrap(), CelValue::Int(10)));
    assert!(matches!(vars.get("z").unwrap(), CelValue::Bool(false)));
}

#[test]
fn test_unsupported_type() {
    let args = vec![("x".to_string(), "list".to_string(), "[]".to_string())];
    let result = args_to_cel_variables(&args);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Unsupported type"));
}

#[test]
fn test_parse_error() {
    let args = vec![(
        "x".to_string(),
        "int".to_string(),
        "not_a_number".to_string(),
    )];
    let result = args_to_cel_variables(&args);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Failed to parse argument 'x'"));
}

#[test]
fn test_parse_errors_for_each_numeric_and_boolean_type() {
    let cases = [
        ("uint", "-1", "cannot parse '-1' as uint"),
        (
            "float",
            "not-a-float",
            "cannot parse 'not-a-float' as float",
        ),
        ("bool", "yes", "cannot parse 'yes' as bool"),
    ];

    for (type_name, value, expected) in cases {
        let args = vec![(
            "value".to_string(),
            type_name.to_string(),
            value.to_string(),
        )];

        let error = args_to_cel_variables(&args).unwrap_err().to_string();
        assert!(
            error.contains(expected),
            "expected {error:?} to contain {expected:?}"
        );
    }
}
