use super::parse_xml;
use serde_json::json;

#[test]
fn parses_nested_elements_without_single_item_arrays() {
    let actual = parse_xml(
        r#"<database>
  <host>localhost</host>
  <port>5432</port>
</database>"#,
    )
    .unwrap();

    assert_eq!(
        actual,
        json!({
            "database": {
                "host": "localhost",
                "port": "5432"
            }
        })
    );
}

#[test]
fn repeated_sibling_elements_become_arrays() {
    let actual = parse_xml(
        r#"<servers>
  <server><ip>192.168.1.1</ip></server>
  <server><ip>192.168.1.2</ip></server>
</servers>"#,
    )
    .unwrap();

    assert_eq!(
        actual,
        json!({
            "servers": {
                "server": [
                    { "ip": "192.168.1.1" },
                    { "ip": "192.168.1.2" }
                ]
            }
        })
    );
}

#[test]
fn preserves_attributes_under_dollar_key() {
    let actual = parse_xml(r#"<user id="42"><name>Alice</name></user>"#).unwrap();

    assert_eq!(
        actual,
        json!({
            "user": {
                "$": { "id": "42" },
                "name": "Alice"
            }
        })
    );
}

#[test]
fn preserves_text_for_elements_with_attributes() {
    let actual = parse_xml(r#"<message lang="en">Hello &amp; welcome</message>"#).unwrap();

    assert_eq!(
        actual,
        json!({
            "message": {
                "$": { "lang": "en" },
                "_": "Hello & welcome"
            }
        })
    );
}

#[test]
fn rejects_mismatched_tags() {
    let err = parse_xml("<foo>bar</baz>").unwrap_err();
    let err = err.to_string();

    assert!(err.contains("mismatched") || (err.contains("foo") && err.contains("baz")));
}
