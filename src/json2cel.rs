use crate::InputFormat;
use cel::objects::Key;
use cel::objects::Value as CelValue;
use rayon::prelude::*;
use serde::de::Error as _;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

/// Convert a JSON string into a BTreeMap of CEL values.
/// The top-level JSON object is placed under the root variable key.
pub fn json_to_cel_variables(
    json_str: &str,
    root_var: &str,
    input_format: InputFormat,
    parallelism: i32,
) -> Result<BTreeMap<String, CelValue>, serde_json::Error> {
    let json_value: JsonValue = match input_format {
        InputFormat::Json => {
            // Regular JSON parsing (NDJSON is handled line-by-line in input_handler)
            serde_json::from_str(json_str)?
        }
        InputFormat::SlurpJson => slurp_json_lines(Some(json_str), parallelism)?,
        InputFormat::Json5 => json5::from_str(json_str).map_err(serde_json::Error::custom)?,
        InputFormat::Toml => {
            #[cfg(feature = "from-toml")]
            {
                toml::from_str(json_str).map_err(serde_json::Error::custom)?
            }

            #[cfg(not(feature = "from-toml"))]
            {
                return Err(serde_json::Error::custom(
                    "Binary was compiled without TOML support",
                ));
            }
        }
        InputFormat::Yaml => {
            #[cfg(feature = "from-yaml")]
            {
                // Try parsing as a single YAML document first
                match serde_saphyr::from_str::<JsonValue>(json_str) {
                    Ok(value) => value,
                    Err(single_err) => {
                        // Try multi-document as a fallback
                        match serde_saphyr::from_multiple::<JsonValue>(json_str) {
                            Ok(values) => serde_json::Value::Array(values),
                            Err(_multi_err) => {
                                // Both attempts failed, invalid YAML input
                                return Err(serde_json::Error::custom(single_err));
                            }
                        }
                    }
                }
            }

            #[cfg(not(feature = "from-yaml"))]
            {
                return Err(serde_json::Error::custom(
                    "Binary was compiled without YAML support",
                ));
            }
        }
        InputFormat::Gron => {
            #[cfg(feature = "greppable")]
            {
                crate::gron_to_json(json_str).map_err(serde_json::Error::custom)?
            }

            #[cfg(not(feature = "greppable"))]
            {
                return Err(serde_json::Error::custom(
                    "Binary was compiled without greppable support",
                ));
            }
        }
    };

    let mut variables = BTreeMap::new();

    // Convert the entire JSON value and place it under the root variable
    let cel_value = json_value_to_cel_value(&json_value);
    variables.insert(root_var.to_string(), cel_value);

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

fn slurp_json_lines(
    json_str: Option<&str>,
    parallelism: i32,
) -> Result<JsonValue, serde_json::Error> {
    if let Some(s) = json_str {
        let lines: Vec<&str> = s.lines().filter(|line| !line.trim().is_empty()).collect();

        let values: Result<Vec<JsonValue>, serde_json::Error> = if parallelism == 1 {
            lines
                .iter()
                .map(|line| serde_json::from_str(line))
                .collect()
        } else {
            let num_threads = if parallelism <= -1 {
                std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(1)
            } else {
                parallelism as usize
            };
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .map_err(|e| serde_json::Error::custom(e.to_string()))?
                .install(|| {
                    lines
                        .par_iter()
                        .map(|line| serde_json::from_str(line))
                        .collect()
                })
        };

        values.map(JsonValue::Array)
    } else {
        Ok(JsonValue::Array(Vec::new()))
    }
}

#[cfg(test)]
#[path = "json2cel_test.rs"]
mod test;
