use super::*;
use serde_json::json;

#[test]
fn test_simple_object() {
    let value = json!({
        "name": "Get Celq",
        "email": "get-celq@proton.me"
    });

    let result = json_to_gron(&value);
    assert!(result.contains(r#"json = {};"#));
    assert!(result.contains(r#"json.name = "Get Celq";"#));
    assert!(result.contains(r#"json.email = "get-celq@proton.me";"#));
}

#[test]
fn test_nested_object() {
    let value = json!({
        "commit": {
            "author": {
                "name": "Get Celq",
                "email": "get-celq@proton.me",
                "date": "2026-01-01T06:00:00Z"
            }
        }
    });

    let result = json_to_gron(&value);
    assert!(result.contains(r#"json.commit.author = {};"#));
    assert!(result.contains(r#"json.commit.author.name = "Get Celq";"#));
    assert!(result.contains(r#"json.commit.author.email = "get-celq@proton.me";"#));
    assert!(result.contains(r#"json.commit.author.date = "2026-01-01T06:00:00Z";"#));
}

#[test]
fn test_array() {
    let value = json!([
        {
            "commit": {
                "author": {
                    "name": "Get Celq"
                }
            }
        }
    ]);

    let result = json_to_gron(&value);
    assert!(result.contains("json = [];"));
    assert!(result.contains(r#"json[0].commit.author.name = "Get Celq";"#));
}

#[test]
fn test_special_keys() {
    let value = json!({
        "x86_64-unknown-linux-musl": {
            "pkg-url": "some-value"
        }
    });

    let result = json_to_gron(&value);
    // Keys with hyphens are not valid identifiers, so they need bracket notation
    assert!(result.contains(r#"json["x86_64-unknown-linux-musl"]["pkg-url"] = "some-value";"#));
}

#[test]
fn test_primitives() {
    let value = json!({
        "null_val": null,
        "bool_val": true,
        "num_val": 42,
        "float_val": 17.38
    });

    let result = json_to_gron(&value);
    assert!(result.contains(r#"json.null_val = null;"#));
    assert!(result.contains(r#"json.bool_val = true;"#));
    assert!(result.contains(r#"json.num_val = 42;"#));
    assert!(result.contains(r#"json.float_val = 17.38;"#));
}

#[test]
fn test_string_escaping() {
    let value = json!({
        "with\"quote": "value with \"quote\"",
        "with\nnewline": "value\nwith\nnewline"
    });

    let result = json_to_gron(&value);
    println!("DEBUG: {}", result);
    assert!(result.contains(r#"json["with\"quote"] = "value with \"quote\"";"#));
    assert!(result.contains(r#"json["with\nnewline"] = "value\nwith\nnewline";"#));
}

#[test]
fn test_your_example() {
    let value = json!({
        "package": {
            "metadata": {
                "binstall": {
                    "overrides": {
                        "x86_64-unknown-linux-musl": {
                            "pkg-url": "{ repo }/releases/download/v{ version }/{ name }-linux-x86_64-musl{ archive-suffix }"
                        }
                    }
                }
            }
        }
    });

    let result = json_to_gron(&value);
    // "pkg-url" and "x86_64-unknown-linux-musl" have hyphens, so they need brackets
    // but package, metadata, binstall, overrides are valid identifiers
    assert!(result.contains(
            r#"json.package.metadata.binstall.overrides["x86_64-unknown-linux-musl"]["pkg-url"] = "{ repo }/releases/download/v{ version }/{ name }-linux-x86_64-musl{ archive-suffix }";"#
        ));
}

#[test]
fn test_mixed_notation() {
    let value = json!({
        "valid_id": {
            "also-valid": "value",
            "nested": {
                "deep-key": "deep-value"
            }
        },
        "123start": "invalid"
    });

    let result = json_to_gron(&value);
    // valid_id is a valid identifier, uses dot
    assert!(result.contains(r#"json.valid_id = {};"#));
    // also-valid has hyphen, needs brackets
    assert!(result.contains(r#"json.valid_id["also-valid"] = "value";"#));
    // nested is valid, uses dot
    assert!(result.contains(r#"json.valid_id.nested = {};"#));
    // deep-key has hyphen, needs brackets
    assert!(result.contains(r#"json.valid_id.nested["deep-key"] = "deep-value";"#));
    // 123start starts with number but has letters, needs quoted brackets
    assert!(result.contains(r#"json["123start"] = "invalid";"#));
}

#[test]
fn test_array_with_objects() {
    let value = json!([
        {
            "123": "starts with number",
            "validKey": "valid"
        }
    ]);

    let result = json_to_gron(&value);
    // Array index is unquoted
    assert!(result.contains(r#"json[0] = {};"#));
    // Numeric string object key uses quoted bracket notation
    assert!(result.contains(r#"json[0]["123"] = "starts with number";"#));
    // Valid identifier after array index uses dot
    assert!(result.contains(r#"json[0].validKey = "valid";"#));
}

#[test]
fn test_serde_saphyr_example() {
    let value = json!({
        "dependencies": {
            "serde-saphyr": {
                "version": "=0.0.14"
            }
        }
    });

    let result = json_to_gron(&value);
    // Should produce: json.dependencies["serde-saphyr"].version = "=0.0.14";
    assert!(result.contains(r#"json.dependencies["serde-saphyr"].version = "=0.0.14";"#));
}

#[test]
fn test_features_array_example() {
    let value = json!({
        "features": {
            "default": [
                "from-yaml",
                "from-toml"
            ]
        }
    });

    let result = json_to_gron(&value);
    // Array indices are unquoted
    assert!(result.contains(r#"json.features.default[0] = "from-yaml";"#));
    assert!(result.contains(r#"json.features.default[1] = "from-toml";"#));
}
