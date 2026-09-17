use crate::json2cel::json_value_to_cel_value;
use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use cel::objects::Value as CelValue;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Convert CLI arguments into a BTreeMap of CEL values.
/// Supports int, uint, float, string, bool, and JSON-encoded lists and maps.
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

            json_type @ ("list" | "map") => parse_json_argument(name, json_type, value_str)?,

            _ => {
                bail!(
                    "Unsupported type: '{}'. Supported types are int, uint, float, string, bool, list, and map.",
                    type_name
                );
            }
        };

        variables.insert(name.clone(), cel_value);
    }

    Ok(variables)
}

fn parse_json_argument(name: &str, json_type: &str, value: &str) -> Result<CelValue> {
    let parsed: JsonValue = serde_json::from_str(value).map_err(|error| {
        anyhow::anyhow!(
            "Failed to parse argument '{}': invalid JSON for {}: {}",
            name,
            json_type,
            error
        )
    })?;

    let (has_expected_type, expected_json_type) = match json_type {
        "list" => (parsed.is_array(), "array"),
        "map" => (parsed.is_object(), "object"),
        _ => unreachable!("JSON argument type must be list or map"),
    };

    if !has_expected_type {
        bail!(
            "Failed to parse argument '{}': expected a JSON {} for {}",
            name,
            expected_json_type,
            json_type
        );
    }

    Ok(json_value_to_cel_value(&parsed))
}

#[cfg(test)]
#[path = "args2cel_test.rs"]
mod test;
