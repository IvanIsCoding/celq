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
mod args2cel_test;
