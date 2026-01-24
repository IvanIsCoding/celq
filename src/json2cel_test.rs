use super::*;

const ROOT_VAR: &str = "this";
const DEFAULT_PARALLELISM: i32 = 1;

#[test]
fn test_null() {
    let vars =
        json_to_cel_variables("null", ROOT_VAR, InputFormat::Json, DEFAULT_PARALLELISM).unwrap();
    assert!(matches!(vars.get("this").unwrap(), CelValue::Null));
}

#[test]
fn test_number() {
    let vars =
        json_to_cel_variables("42", ROOT_VAR, InputFormat::Json, DEFAULT_PARALLELISM).unwrap();
    assert!(matches!(vars.get("this").unwrap(), CelValue::Int(42)));
}

#[test]
fn test_string() {
    let vars = json_to_cel_variables(
        r#""hello""#,
        ROOT_VAR,
        InputFormat::Json,
        DEFAULT_PARALLELISM,
    )
    .unwrap();
    if let CelValue::String(s) = vars.get("this").unwrap() {
        assert_eq!(s.as_str(), "hello");
    } else {
        panic!("Expected string");
    }
}

#[test]
fn test_bool() {
    let vars =
        json_to_cel_variables("true", ROOT_VAR, InputFormat::Json, DEFAULT_PARALLELISM).unwrap();
    assert!(matches!(vars.get("this").unwrap(), CelValue::Bool(true)));
}

#[test]
fn test_array() {
    let vars = json_to_cel_variables(
        "[1, 2, 3]",
        ROOT_VAR,
        InputFormat::Json,
        DEFAULT_PARALLELISM,
    )
    .unwrap();
    if let CelValue::List(list) = vars.get("this").unwrap() {
        assert_eq!(list.len(), 3);
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_object() {
    let vars = json_to_cel_variables(
        r#"{"x": 10, "y": 20}"#,
        ROOT_VAR,
        InputFormat::Json,
        DEFAULT_PARALLELISM,
    )
    .unwrap();

    // Should have "this"
    assert_eq!(vars.len(), 1);

    // Check "this" contains the full object
    assert!(matches!(vars.get("this").unwrap(), CelValue::Map(_)));
}

#[test]
fn test_nested_object() {
    let vars = json_to_cel_variables(
        r#"{"outer": {"inner": 42}}"#,
        ROOT_VAR,
        InputFormat::Json,
        DEFAULT_PARALLELISM,
    )
    .unwrap();

    // Should have "this"
    assert_eq!(vars.len(), 1);

    // Check "this" is a map
    if let CelValue::Map(map) = vars.get("this").unwrap() {
        let outer_key = Key::String(Arc::new("outer".to_string()));
        if let CelValue::Map(inner_map) = map.get(&outer_key).unwrap() {
            let inner_key = Key::String(Arc::new("inner".to_string()));
            assert!(matches!(
                inner_map.get(&inner_key).unwrap(),
                CelValue::Int(42)
            ));
        } else {
            panic!("Expected inner map");
        }
    } else {
        panic!("Expected map");
    }
}

#[test]
fn test_json5_with_comment() {
    let json5_input = r#"
    {
        // This is a comment
        "x": 42
    }
    "#;
    let vars = json_to_cel_variables(
        json5_input,
        ROOT_VAR,
        InputFormat::Json5,
        DEFAULT_PARALLELISM,
    )
    .unwrap();

    if let CelValue::Map(map) = vars.get("this").unwrap() {
        let x_key = Key::String(Arc::new("x".to_string()));
        assert!(matches!(map.get(&x_key).unwrap(), CelValue::Int(42)));
    } else {
        panic!("Expected map");
    }
}

#[test]
fn test_slurp_single_threaded() {
    let json_lines = r#"{"x": 1}
{"x": 2}
{"x": 3}"#;

    let vars = json_to_cel_variables(
        json_lines,
        ROOT_VAR,
        InputFormat::SlurpJson,
        1, // parallelism = 1
    )
    .unwrap();

    if let CelValue::List(list) = vars.get("this").unwrap() {
        assert_eq!(list.len(), 3);
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_slurp_parallel() {
    let json_lines = r#"{"x": 1}
{"x": 2}
{"x": 3}"#;

    let vars = json_to_cel_variables(
        json_lines,
        ROOT_VAR,
        InputFormat::SlurpJson,
        2, // parallelism = 2
    )
    .unwrap();

    if let CelValue::List(list) = vars.get("this").unwrap() {
        assert_eq!(list.len(), 3);
    } else {
        panic!("Expected list");
    }
}

#[test]
#[cfg(feature = "from-toml")]
fn test_toml_format() {
    let toml_input = r#"
    [package]
    name = "example"
    version = "1.0.0"
    "#;

    let vars = json_to_cel_variables(toml_input, ROOT_VAR, InputFormat::Toml, DEFAULT_PARALLELISM)
        .unwrap();

    if let CelValue::Map(map) = vars.get("this").unwrap() {
        let package_key = Key::String(Arc::new("package".to_string()));
        assert!(matches!(map.get(&package_key).unwrap(), CelValue::Map(_)));
    } else {
        panic!("Expected map");
    }
}

#[test]
#[cfg(not(feature = "from-toml"))]
fn test_toml_format_disabled() {
    let toml_input = r#"
    [package]
    name = "example"
    version = "1.0.0"
    "#;

    let vars = json_to_cel_variables(toml_input, ROOT_VAR, InputFormat::Toml, DEFAULT_PARALLELISM);

    assert!(vars.is_err());
}

#[test]
#[cfg(feature = "from-yaml")]
fn test_yaml_format() {
    let yaml_input = r#"
    name: example
    version: 1.0.0
    dependencies:
      - dep1
      - dep2
    "#;

    let vars = json_to_cel_variables(yaml_input, ROOT_VAR, InputFormat::Yaml, DEFAULT_PARALLELISM)
        .unwrap();

    if let CelValue::Map(map) = vars.get("this").unwrap() {
        let name_key = Key::String(Arc::new("name".to_string()));
        if let CelValue::String(name) = map.get(&name_key).unwrap() {
            assert_eq!(name.as_str(), "example");
        } else {
            panic!("Expected string for name");
        }
    } else {
        panic!("Expected map");
    }
}

#[test]
#[cfg(not(feature = "from-yaml"))]
fn test_yaml_format_disabled() {
    let yaml_input = r#"
    name: example
    version: 1.0.0
    dependencies:
      - dep1
      - dep2
    "#;

    let vars = json_to_cel_variables(yaml_input, ROOT_VAR, InputFormat::Yaml, DEFAULT_PARALLELISM);

    assert!(vars.is_err());
}

#[test]
#[cfg(feature = "greppable")]
fn test_gron_format() {
    let gron_input = r#"
    json = {};
    json.name = "example";
    json.value = 42;
    "#;

    let vars = json_to_cel_variables(gron_input, ROOT_VAR, InputFormat::Gron, DEFAULT_PARALLELISM)
        .unwrap();

    assert!(matches!(vars.get("this").unwrap(), CelValue::Map(_)));
}

#[test]
#[cfg(not(feature = "greppable"))]
fn test_gron_format_disabled() {
    let gron_input = r#"
    json = {};
    json.name = "example";
    json.value = 42;
    "#;

    let vars = json_to_cel_variables(gron_input, ROOT_VAR, InputFormat::Gron, DEFAULT_PARALLELISM);

    assert!(vars.is_err());
}

#[test]
fn test_custom_root_var() {
    let vars =
        json_to_cel_variables("42", "myvar", InputFormat::Json, DEFAULT_PARALLELISM).unwrap();

    assert!(vars.contains_key("myvar"));
    assert!(!vars.contains_key("this"));
    assert!(matches!(vars.get("myvar").unwrap(), CelValue::Int(42)));
}
