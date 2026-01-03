use anyhow::{Context, Result, bail};
use cel::objects::Value as CelValue;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Convert CLI arguments into a BTreeMap of CEL values.
/// Only supports simple types: int, uint, float, string, bool
pub fn args_to_cel_variables(
    args: &[(String, String, String)], // (name, type_name, value)
) -> Result<BTreeMap<String, CelValue>> {
    let mut variables = BTreeMap::new();

    for (name, type_name, value_str) in args {
        let cel_value = match type_name.to_lowercase().as_str() {
            "int" | "i64" => {
                let parsed = value_str.parse::<i64>().with_context(|| {
                    format!(
                        "Failed to parse argument '{}': cannot parse '{}' as int",
                        name, value_str
                    )
                })?;
                CelValue::Int(parsed)
            }

            "uint" | "u64" => {
                let parsed = value_str.parse::<u64>().with_context(|| {
                    format!(
                        "Failed to parse argument '{}': cannot parse '{}' as uint",
                        name, value_str
                    )
                })?;
                CelValue::UInt(parsed)
            }

            "float" | "f64" | "double" => {
                let parsed = value_str.parse::<f64>().with_context(|| {
                    format!(
                        "Failed to parse argument '{}': cannot parse '{}' as float",
                        name, value_str
                    )
                })?;
                CelValue::Float(parsed)
            }

            "string" | "str" => CelValue::String(Arc::new(value_str.clone())),

            "bool" | "boolean" => {
                let parsed = value_str.parse::<bool>().with_context(|| {
                    format!(
                        "Failed to parse argument '{}': cannot parse '{}' as bool",
                        name, value_str
                    )
                })?;
                CelValue::Bool(parsed)
            }

            _ => {
                bail!(
                    "Unsupported type: '{}'. Only simple types (int, uint, float, string, bool) are supported.",
                    type_name
                );
            }
        };

        variables.insert(name.clone(), cel_value);
    }

    Ok(variables)
}

#[cfg(test)]
mod tests {
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
        let args = vec![("x".to_string(), "float".to_string(), "3.14".to_string())];
        let vars = args_to_cel_variables(&args).unwrap();
        if let CelValue::Float(f) = vars.get("x").unwrap() {
            assert!((f - 3.14).abs() < 0.001);
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
}
