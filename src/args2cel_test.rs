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
fn test_list() {
    let args = vec![(
        "items".to_string(),
        "list".to_string(),
        r#"[1,"two",true,null,[3],{"four":4}]"#.to_string(),
    )];
    let vars = args_to_cel_variables(&args).unwrap();

    let CelValue::List(items) = vars.get("items").unwrap() else {
        panic!("Expected list");
    };

    assert_eq!(items.len(), 6);
    assert!(matches!(items[0], CelValue::Int(1)));
    assert!(matches!(&items[1], CelValue::String(value) if value.as_str() == "two"));
    assert!(matches!(items[2], CelValue::Bool(true)));
    assert!(matches!(items[3], CelValue::Null));
    assert!(matches!(&items[4], CelValue::List(values) if matches!(values[0], CelValue::Int(3))));
    assert!(matches!(items[5], CelValue::Map(_)));
}

#[test]
fn test_map() {
    let args = vec![(
        "config".to_string(),
        "map".to_string(),
        r#"{"name":"celq","enabled":true,"items":[1,2],"nested":{"value":3}}"#.to_string(),
    )];
    let vars = args_to_cel_variables(&args).unwrap();

    let CelValue::Map(config) = vars.get("config").unwrap() else {
        panic!("Expected map");
    };

    assert_eq!(config.map.len(), 4);
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
    let args = vec![("x".to_string(), "bytes".to_string(), "value".to_string())];
    let result = args_to_cel_variables(&args);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Unsupported type"));
}

#[test]
fn test_list_invalid_json() {
    let args = vec![("items".to_string(), "list".to_string(), "[1,".to_string())];
    let error = args_to_cel_variables(&args).unwrap_err().to_string();
    assert!(error.contains("Failed to parse argument 'items': invalid JSON for list"));
}

#[test]
fn test_list_requires_json_array() {
    let args = vec![(
        "items".to_string(),
        "list".to_string(),
        r#"{"one":1}"#.to_string(),
    )];
    let error = args_to_cel_variables(&args).unwrap_err().to_string();
    assert_eq!(
        error,
        "Failed to parse argument 'items': expected a JSON array for list"
    );
}

#[test]
fn test_map_invalid_json() {
    let args = vec![(
        "config".to_string(),
        "map".to_string(),
        "{\"one\":".to_string(),
    )];
    let error = args_to_cel_variables(&args).unwrap_err().to_string();
    assert!(error.contains("Failed to parse argument 'config': invalid JSON for map"));
}

#[test]
fn test_map_requires_json_object() {
    let args = vec![("config".to_string(), "map".to_string(), "[1,2]".to_string())];
    let error = args_to_cel_variables(&args).unwrap_err().to_string();
    assert_eq!(
        error,
        "Failed to parse argument 'config': expected a JSON object for map"
    );
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
