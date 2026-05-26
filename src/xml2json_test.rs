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

#[test]
fn rejects_closing_tag_without_open_element() {
    let err = parse_xml("</foo>").unwrap_err().to_string();

    assert!(err.contains("does not match any open tag"));
}

#[test]
fn rejects_unclosed_elements_at_end_of_input() {
    let err = parse_xml("<foo><bar>").unwrap_err().to_string();

    assert!(err.contains("unexpected end of input"));
}

#[test]
fn rejects_multiple_root_elements() {
    let err = parse_xml("<foo>one</foo><bar>two</bar>")
        .unwrap_err()
        .to_string();

    assert!(err.contains("multiple root elements"));
}

#[test]
fn rejects_text_outside_root_element() {
    let err = parse_xml("text<foo>bar</foo>").unwrap_err().to_string();

    assert!(err.contains("text outside root element"));
}

#[test]
fn rejects_unknown_entity_references() {
    let err = parse_xml("<foo>&unknown;</foo>").unwrap_err().to_string();

    assert!(err.contains("unknown XML entity reference: &unknown;"));
}

#[test]
fn rejects_invalid_character_references() {
    let err = parse_xml("<foo>&#xnothex;</foo>").unwrap_err().to_string();

    assert!(err.contains("invalid digit") || err.contains("Invalid"));
}

#[test]
fn rejects_malformed_attributes() {
    let err = parse_xml("<foo bar></foo>").unwrap_err().to_string();

    assert!(err.contains("attribute") || err.contains("error at position"));
}

#[test]
fn rejects_malformed_xml_events() {
    let err = parse_xml("<foo><bar></foo></bar>").unwrap_err().to_string();

    assert!(err.contains("mismatched") || err.contains("error at position"));
}
