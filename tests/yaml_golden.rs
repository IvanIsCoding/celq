mod base;

#[cfg(feature = "from-yaml")]
use base::test;

// YAML tests

// YAML with nested mappings
#[cfg(feature = "from-yaml")]
test!(
    yaml_nested_mapping,
    &[
        "--from-yaml",
        "this.database.host + ':' + string(this.database.port)"
    ],
    r#"database:
  host: localhost
  port: 5432
  # Database configuration
"#,
    "\"localhost:5432\""
);

#[cfg(feature = "from-yaml")]
test!(
    yaml_array_of_mappings,
    &["--from-yaml", "this.servers.map(s, s.ip)"],
    r#"servers:
  - ip: 192.168.1.1
    name: alpha
  - ip: 192.168.1.2
    name: beta
"#,
    "[\"192.168.1.1\",\"192.168.1.2\"]"
);

#[cfg(feature = "from-yaml")]
test!(
    yaml_nested_keys,
    &[
        "--from-yaml",
        "this.user.name + ' <' + this.user.email + '>'"
    ],
    r#"user:
  name: Alice
  email: alice@example.com
"#,
    "\"Alice <alice@example.com>\""
);

#[cfg(feature = "from-yaml")]
test!(
    yaml_flow_mapping,
    &["--from-yaml", "this.point.x + this.point.y"],
    r#"point: {x: 10, y: 20}
"#,
    "30"
);

// Multi-document YAML
#[cfg(feature = "from-yaml")]
test!(
    yaml_multi_document_with_separator,
    &["--from-yaml", "this.map(doc, doc.name)"],
    r#"name: Alice
age: 30
---
name: Bob
age: 25
---
name: Charlie
age: 35
"#,
    "[\"Alice\",\"Bob\",\"Charlie\"]"
);

#[cfg(feature = "from-yaml")]
test!(
    yaml_multi_document_with_terminator,
    &["--from-yaml", "this.map(doc, doc.value)"],
    r#"value: first
...
---
value: second
...
---
value: third
"#,
    "[\"first\",\"second\",\"third\"]"
);

#[cfg(feature = "from-yaml")]
test!(
    yaml_multi_document_single_doc_stays_scalar,
    &["--from-yaml", "this.name"],
    r#"name: Alice
age: 30
"#,
    "\"Alice\""
);
