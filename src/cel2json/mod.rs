use cel::objects::{Key, Value as CelValue};
use serde_json::Value as JsonValue;

/// Convert a CEL value to a serde_json::Value
pub fn cel_value_to_json_value(value: &CelValue) -> JsonValue {
    match value {
        CelValue::Null => JsonValue::Null,

        CelValue::Bool(b) => JsonValue::Bool(*b),

        CelValue::Int(i) => JsonValue::Number((*i).into()),

        CelValue::UInt(u) => JsonValue::Number((*u).into()),

        CelValue::Float(f) => {
            // serde_json::Number doesn't support NaN or infinity
            // Handle these edge cases appropriately
            if f.is_finite() {
                serde_json::Number::from_f64(*f)
                    .map(JsonValue::Number)
                    .unwrap_or(JsonValue::Null)
            } else {
                JsonValue::Null
            }
        }

        CelValue::String(s) => JsonValue::String(s.to_string()),

        CelValue::List(list) => {
            let json_array: Vec<JsonValue> = list.iter().map(cel_value_to_json_value).collect();
            JsonValue::Array(json_array)
        }

        CelValue::Map(map) => {
            let mut json_map = serde_json::Map::new();
            for (key, val) in map.map.iter() {
                // Convert Key to string
                let key_string = match key {
                    Key::String(s) => s.to_string(),
                    Key::Int(i) => i.to_string(),
                    Key::Uint(u) => u.to_string(),
                    Key::Bool(b) => b.to_string(),
                };
                json_map.insert(key_string, cel_value_to_json_value(val));
            }
            JsonValue::Object(json_map)
        }

        // Handle any other CEL value types by converting to string representation
        _ => JsonValue::String(format!("{:?}", value)),
    }
}
