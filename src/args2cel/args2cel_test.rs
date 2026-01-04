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
