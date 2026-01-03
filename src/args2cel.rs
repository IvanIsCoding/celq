use cel::objects::Value as CelValue;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug)]
pub enum ArgConversionError {
    UnsupportedType(String),
    ParseError(String, String), // field name, error message
}

impl std::fmt::Display for ArgConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgConversionError::UnsupportedType(type_name) => {
                write!(
                    f,
                    "Unsupported type: '{}'. Only simple types (int, uint, float, string, bool) are supported.",
                    type_name
                )
            }
            ArgConversionError::ParseError(name, msg) => {
                write!(f, "Failed to parse argument '{}': {}", name, msg)
            }
        }
    }
}

impl std::error::Error for ArgConversionError {}

/// Convert CLI arguments into a BTreeMap of CEL values.
/// Only supports simple types: int, uint, float, string, bool
pub fn args_to_cel_variables(
    args: &[(String, String, String)], // (name, type_name, value)
) -> Result<BTreeMap<String, CelValue>, ArgConversionError> {
    let mut variables = BTreeMap::new();

    for (name, type_name, value_str) in args {
        let cel_value = match type_name.to_lowercase().as_str() {
            "int" | "i64" => {
                let parsed = value_str.parse::<i64>().map_err(|e| {
                    ArgConversionError::ParseError(
                        name.clone(),
                        format!("cannot parse '{}' as int: {}", value_str, e),
                    )
                })?;
                CelValue::Int(parsed)
            }

            "uint" | "u64" => {
                let parsed = value_str.parse::<u64>().map_err(|e| {
                    ArgConversionError::ParseError(
                        name.clone(),
                        format!("cannot parse '{}' as uint: {}", value_str, e),
                    )
                })?;
                CelValue::UInt(parsed)
            }

            "float" | "f64" | "double" => {
                let parsed = value_str.parse::<f64>().map_err(|e| {
                    ArgConversionError::ParseError(
                        name.clone(),
                        format!("cannot parse '{}' as float: {}", value_str, e),
                    )
                })?;
                CelValue::Float(parsed)
            }

            "string" | "str" => CelValue::String(Arc::new(value_str.clone())),

            "bool" | "boolean" => {
                let parsed = value_str.parse::<bool>().map_err(|e| {
                    ArgConversionError::ParseError(
                        name.clone(),
                        format!("cannot parse '{}' as bool: {}", value_str, e),
                    )
                })?;
                CelValue::Bool(parsed)
            }

            _ => {
                return Err(ArgConversionError::UnsupportedType(type_name.clone()));
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
        let args = vec![("x".to_string(), "int".to_string(), Some("42".to_string()))];
        let vars = args_to_cel_variables(&args).unwrap();
        assert!(matches!(vars.get("x").unwrap(), CelValue::Int(42)));
    }

    #[test]
    fn test_uint() {
        let args = vec![("x".to_string(), "uint".to_string(), Some("42".to_string()))];
        let vars = args_to_cel_variables(&args).unwrap();
        assert!(matches!(vars.get("x").unwrap(), CelValue::UInt(42)));
    }

    #[test]
    fn test_float() {
        let args = vec![(
            "x".to_string(),
            "float".to_string(),
            Some("3.14".to_string()),
        )];
        let vars = args_to_cel_variables(&args).unwrap();
        if let CelValue::Float(f) = vars.get("x").unwrap() {
            assert!((f - 3.14).abs() < 0.001);
        } else {
            panic!("Expected float");
        }
    }

    #[test]
    fn test_string() {
        let args = vec![(
            "x".to_string(),
            "string".to_string(),
            Some("hello".to_string()),
        )];
        let vars = args_to_cel_variables(&args).unwrap();
        if let CelValue::String(s) = vars.get("x").unwrap() {
            assert_eq!(s.as_str(), "hello");
        } else {
            panic!("Expected string");
        }
    }

    #[test]
    fn test_bool() {
        let args = vec![(
            "x".to_string(),
            "bool".to_string(),
            Some("true".to_string()),
        )];
        let vars = args_to_cel_variables(&args).unwrap();
        assert!(matches!(vars.get("x").unwrap(), CelValue::Bool(true)));
    }

    #[test]
    fn test_multiple_args() {
        let args = vec![
            ("x".to_string(), "int".to_string(), Some("10".to_string())),
            (
                "y".to_string(),
                "string".to_string(),
                Some("test".to_string()),
            ),
            (
                "z".to_string(),
                "bool".to_string(),
                Some("false".to_string()),
            ),
        ];
        let vars = args_to_cel_variables(&args).unwrap();
        assert_eq!(vars.len(), 3);
        assert!(matches!(vars.get("x").unwrap(), CelValue::Int(10)));
        assert!(matches!(vars.get("z").unwrap(), CelValue::Bool(false)));
    }

    #[test]
    fn test_skip_args_without_values() {
        let args = vec![
            ("x".to_string(), "int".to_string(), Some("10".to_string())),
            ("y".to_string(), "string".to_string(), None),
        ];
        let vars = args_to_cel_variables(&args).unwrap();
        assert_eq!(vars.len(), 1);
        assert!(vars.get("x").is_some());
        assert!(vars.get("y").is_none());
    }

    #[test]
    fn test_unsupported_type() {
        let args = vec![("x".to_string(), "list".to_string(), Some("[]".to_string()))];
        let result = args_to_cel_variables(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ArgConversionError::UnsupportedType(_)
        ));
    }

    #[test]
    fn test_parse_error() {
        let args = vec![(
            "x".to_string(),
            "int".to_string(),
            Some("not_a_number".to_string()),
        )];
        let result = args_to_cel_variables(&args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ArgConversionError::ParseError(_, _)
        ));
    }
}
