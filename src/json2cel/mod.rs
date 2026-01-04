use cel::objects::{Key, Value as CelValue};
use serde::de::Error as _;
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

/// Convert a JSON string into a BTreeMap of CEL values.
/// The top-level JSON object is placed under the "this" key.
pub fn json_to_cel_variables(
    json_str: &str,
    root_var: &str,
    slurp: bool,
    from_json5: bool,
) -> Result<BTreeMap<String, CelValue>, serde_json::Error> {
    let json_value: JsonValue = if !slurp && !from_json5 {
        serde_json::from_str(json_str)?
    } else if from_json5 {
        json5::from_str(json_str).map_err(serde_json::Error::custom)?
    } else {
        slurp_json_lines(Some(json_str))?
    };

    let mut variables = BTreeMap::new();

    // Convert the entire JSON value and place it under "this"
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

fn slurp_json_lines(json_str: Option<&str>) -> Result<JsonValue, serde_json::Error> {
    let mut values = Vec::new();

    if let Some(s) = json_str {
        for line in s.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let v: JsonValue = serde_json::from_str(line)?;
            values.push(v);
        }
    }

    Ok(JsonValue::Array(values))
}

#[cfg(test)]
mod json2cel_test;
