use cel::objects::{Key, Value as CelValue};
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

/// Convert a JSON string into a BTreeMap of CEL values.
/// The top-level JSON object is placed under the "this" key.
pub fn json_to_cel_variables(
    json_str: &str,
) -> Result<BTreeMap<String, CelValue>, serde_json::Error> {
    let json_value: JsonValue = serde_json::from_str(json_str)?;

    let mut variables = BTreeMap::new();

    // Convert the entire JSON value and place it under "this"
    let cel_value = json_value_to_cel_value(&json_value);
    variables.insert("this".to_string(), cel_value);

    Ok(variables)
}

/// Convert a serde_json::Value to a cel::objects::Value
fn json_value_to_cel_value(value: &JsonValue) -> CelValue {
    match value {
        JsonValue::Null => CelValue::Null,

        JsonValue::Bool(b) => CelValue::Bool(*b),

        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                CelValue::Int(i)
            } else if let Some(u) = n.as_u64() {
                CelValue::UInt(u)
            } else if let Some(f) = n.as_f64() {
                CelValue::Float(f)
            } else {
                // Fallback, should not happen
                CelValue::Null
            }
        }

        JsonValue::String(s) => CelValue::String(Arc::new(s.clone())),

        JsonValue::Array(arr) => {
            let cel_vec: Vec<CelValue> = arr.iter().map(json_value_to_cel_value).collect();
            CelValue::List(Arc::new(cel_vec))
        }

        JsonValue::Object(map) => {
            let mut cel_map = HashMap::new();
            for (key, val) in map {
                let cel_key = Key::String(Arc::new(key.clone()));
                let cel_val = json_value_to_cel_value(val);
                cel_map.insert(cel_key, cel_val);
            }
            CelValue::Map(cel_map.into())
        }
    }
}

#[cfg(test)]
mod tests {
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
}
