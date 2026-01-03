
use super::*;

#[test]
fn test_null() {
    let vars = json_to_cel_variables("null").unwrap();
    assert!(matches!(vars.get("this").unwrap(), CelValue::Null));
}

#[test]
fn test_number() {
    let vars = json_to_cel_variables("42").unwrap();
    assert!(matches!(vars.get("this").unwrap(), CelValue::Int(42)));
}

#[test]
fn test_string() {
    let vars = json_to_cel_variables(r#""hello""#).unwrap();
    if let CelValue::String(s) = vars.get("this").unwrap() {
        assert_eq!(s.as_str(), "hello");
    } else {
        panic!("Expected string");
    }
}

#[test]
fn test_bool() {
    let vars = json_to_cel_variables("true").unwrap();
    assert!(matches!(vars.get("this").unwrap(), CelValue::Bool(true)));
}

#[test]
fn test_array() {
    let vars = json_to_cel_variables("[1, 2, 3]").unwrap();
    if let CelValue::List(list) = vars.get("this").unwrap() {
        assert_eq!(list.len(), 3);
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_object() {
    let vars = json_to_cel_variables(r#"{"x": 10, "y": 20}"#).unwrap();

    // Should have "this"
    assert_eq!(vars.len(), 1);

    // Check "this" contains the full object
    assert!(matches!(vars.get("this").unwrap(), CelValue::Map(_)));
}

#[test]
fn test_nested_object() {
    let vars = json_to_cel_variables(r#"{"outer": {"inner": 42}}"#).unwrap();

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
