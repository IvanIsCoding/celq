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

#[test]
fn test_rejects_malformed_gron_inputs() {
    let cases = [
        ("missing assignment", "json.foo", "Expected assignment"),
        (
            "invalid root",
            "root.foo = 1;",
            "Path root must be 'json', found 'root'",
        ),
        ("missing root", ".foo = 1;", "Path root must be 'json'"),
        (
            "unexpected path character",
            "json? = 1;",
            "Unexpected character in path: '?'",
        ),
        (
            "missing semicolon",
            "json.foo = 1",
            "Expected assignment to end with ';'",
        ),
        (
            "unterminated quoted path",
            r#"json["foo] = 1;"#,
            "Expected assignment",
        ),
        (
            "unterminated quoted value",
            r#"json.foo = "value;"#,
            "Expected assignment to end with ';'",
        ),
        (
            "extra content after single-quoted value",
            "json.foo = 'value' trailing;",
            "Unexpected characters after string value",
        ),
    ];

    for (case, input, expected_error) in cases {
        let error = gron_to_json(input).unwrap_err();
        let full_error = format!("{error:#}");
        assert!(
            full_error.contains(expected_error),
            "{case}: expected error containing {expected_error:?}, got {full_error}"
        );
    }
}

#[test]
fn test_empty_container_assignments_do_not_overwrite_existing_values() {
    let input = r#"json = {};
json.user = {};
json.user.name = "John";
json.user = {};
json.items = [];
json.items[0] = "first";
json.items = [];"#;

    let result = gron_to_json(input).unwrap();
    let expected: JsonValue = serde_json::json!({
        "user": {
            "name": "John"
        },
        "items": ["first"]
    });

    assert_eq!(result, expected);
}

#[test]
fn test_parse_escape_sequence() {
    let cases = [
        ("\"", '"'),
        ("'", '\''),
        ("\\", '\\'),
        ("/", '/'),
        ("b", '\u{0008}'),
        ("f", '\u{000c}'),
        ("n", '\n'),
        ("r", '\r'),
        ("t", '\t'),
        ("x", 'x'),
        ("u2764", '\u{2764}'),
    ];

    for (input, expected) in cases {
        let mut parser = PathParser::new(input);
        assert_eq!(parser.parse_escape_sequence().unwrap(), expected);
        assert!(parser.is_done());
    }

    let error = PathParser::new("").parse_escape_sequence().unwrap_err();
    assert_eq!(error.to_string(), "Unclosed escape sequence");
}

#[test]
fn test_parse_unicode_escape_errors() {
    let error = PathParser::new("12x4").parse_unicode_escape().unwrap_err();
    assert_eq!(error.to_string(), "Invalid unicode escape character 'x'");

    let error = PathParser::new("123").parse_unicode_escape().unwrap_err();
    assert_eq!(error.to_string(), "Unclosed unicode escape sequence");

    let error = PathParser::new("d800").parse_unicode_escape().unwrap_err();
    assert_eq!(error.to_string(), "Invalid unicode escape code point");
}

#[test]
fn test_parse_bracket_segment_variants() {
    let mut parser = PathParser::new(r#"["quoted key"]"#);
    let segment = parser.parse_bracket_segment().unwrap();
    assert!(matches!(
        segment,
        PathSegment::Property(value) if value == "quoted key"
    ));
    assert!(parser.is_done());

    let mut parser = PathParser::new("['single quoted key']");
    let segment = parser.parse_bracket_segment().unwrap();
    assert!(matches!(
        segment,
        PathSegment::Property(value) if value == "single quoted key"
    ));
    assert!(parser.is_done());

    let mut parser = PathParser::new("[42]");
    let segment = parser.parse_bracket_segment().unwrap();
    assert!(matches!(segment, PathSegment::Index(42)));
    assert!(parser.is_done());
}

#[test]
fn test_parse_bracket_segment_errors() {
    let cases = [
        ("key", "Expected '[', found 'k'"),
        ("[", "Unclosed bracket path segment"),
        ("[?]", "Unsupported bracket path segment starting with '?'"),
        ("[1", "Expected ']', found end of path"),
        ("[1}", "Expected ']', found '}'"),
    ];

    for (input, expected) in cases {
        let error = PathParser::new(input).parse_bracket_segment().unwrap_err();
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn test_split_assignment_ignores_equals_inside_quoted_paths() {
    let cases = [
        (r#"json["a=b"] = 1;"#, r#"json["a=b"] "#, " 1;"),
        ("json['a=b'] = 2;", "json['a=b'] ", " 2;"),
        (r#"json["a\"=b"] = 3;"#, r#"json["a\"=b"] "#, " 3;"),
        (r"json['a\'=b'] = 4;", r"json['a\'=b'] ", " 4;"),
    ];

    for (input, expected_left, expected_right) in cases {
        let (left, right) = split_assignment(input).unwrap();
        assert_eq!(left, expected_left);
        assert_eq!(right, expected_right);
    }
}

#[test]
fn test_read_identifier_edge_cases() {
    for identifier in ["alpha", "_private", "$value", "alpha_beta$gamma9"] {
        let mut parser = PathParser::new(identifier);
        assert_eq!(parser.read_identifier().as_deref(), Some(identifier));
        assert!(parser.is_done());
    }

    for invalid in ["", "9lives", "-name"] {
        let mut parser = PathParser::new(invalid);
        assert_eq!(parser.read_identifier(), None);
        assert_eq!(parser.pos, 0);
    }
}

#[test]
fn test_bump_char_advances_by_utf8_length() {
    let mut parser = PathParser::new("éx");

    parser.bump_char();
    assert_eq!(parser.pos, "é".len());
    assert_eq!(parser.peek_char(), Some('x'));
}

#[test]
fn test_strip_trailing_semicolon_ignores_quoted_semicolons() {
    let cases = [
        (r#""a;b";"#, Some(r#""a;b""#)),
        ("'a;b';", Some("'a;b'")),
        (r#""a\";b";"#, Some(r#""a\";b""#)),
        ("42; \t", Some("42")),
        ("42", None),
        ("42; trailing", None),
    ];

    for (input, expected) in cases {
        assert_eq!(strip_trailing_semicolon(input), expected);
    }
}

#[test]
fn test_empty_containers_only_preserve_matching_existing_containers() {
    let input = r#"json.object_to_scalar = {};
json.object_to_scalar = 1;
json.scalar_to_object = 1;
json.scalar_to_object = {};
json.array_to_scalar = [];
json.array_to_scalar = 2;
json.scalar_to_array = 2;
json.scalar_to_array = [];"#;

    assert_eq!(
        gron_to_json(input).unwrap(),
        serde_json::json!({
            "object_to_scalar": 1,
            "scalar_to_object": {},
            "array_to_scalar": 2,
            "scalar_to_array": []
        })
    );
}
