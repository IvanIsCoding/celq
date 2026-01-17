use super::*;

#[test]
fn test_simple_object() {
    let input = r#"json = {};
json.Host = "headers.jsontest.com";
json["User-Agent"] = "gron/0.1";"#;

    let result = gron_to_json(input).unwrap();
    let expected: JsonValue = serde_json::json!({
        "Host": "headers.jsontest.com",
        "User-Agent": "gron/0.1"
    });

    assert_eq!(result, expected);
}

#[test]
fn test_sparse_array() {
    let input = r#"json.likes = [];
json.likes[0] = "code";
json.likes[2] = "meat";"#;

    let result = gron_to_json(input).unwrap();
    let expected: JsonValue = serde_json::json!({
        "likes": ["code", null, "meat"]
    });

    assert_eq!(result, expected);
}

#[test]
fn test_nested_object() {
    let input = r#"json = {};
json.user = {};
json.user.name = "John";
json.user.age = 30;"#;

    let result = gron_to_json(input).unwrap();
    let expected: JsonValue = serde_json::json!({
        "user": {
            "name": "John",
            "age": 30
        }
    });

    assert_eq!(result, expected);
}

#[test]
fn test_numbers() {
    let input = r#"json = {};
json.integer = 42;
json.float = 17.38;
json.negative = -5;"#;

    let result = gron_to_json(input).unwrap();
    let expected: JsonValue = serde_json::json!({
        "integer": 42,
        "float": 17.38,
        "negative": -5
    });

    assert_eq!(result, expected);
}

#[test]
fn test_booleans_and_null() {
    let input = r#"json = {};
json.isTrue = true;
json.isFalse = false;
json.nothing = null;"#;

    let result = gron_to_json(input).unwrap();
    let expected: JsonValue = serde_json::json!({
        "isTrue": true,
        "isFalse": false,
        "nothing": null
    });

    assert_eq!(result, expected);
}
