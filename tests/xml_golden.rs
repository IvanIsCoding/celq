mod base;

#[cfg(feature = "from-xml")]
use base::test;

// XML tests

// XML with nested elements
#[cfg(feature = "from-xml")]
test!(
    xml_nested_elements,
    &[
        "--from-xml",
        "this.database.host + ':' + string(this.database.port)"
    ],
    r#"<database>
  <host>localhost</host>
  <port>5432</port>
</database>"#,
    "\"localhost:5432\""
);

#[cfg(feature = "from-xml")]
test!(
    xml_array_of_elements,
    &["--from-xml", "this.servers.server.map(s, s.ip)"],
    r#"<servers>
  <server>
    <ip>192.168.1.1</ip>
    <name>alpha</name>
  </server>
  <server>
    <ip>192.168.1.2</ip>
    <name>beta</name>
  </server>
</servers>"#,
    "[\"192.168.1.1\",\"192.168.1.2\"]"
);

#[cfg(feature = "from-xml")]
test!(
    xml_three_repeated_elements_append_to_array,
    &["--from-xml", "this.servers.server.map(s, s.ip)"],
    r#"<servers>
  <server><ip>192.168.1.1</ip></server>
  <server><ip>192.168.1.2</ip></server>
  <server><ip>192.168.1.3</ip></server>
</servers>"#,
    "[\"192.168.1.1\",\"192.168.1.2\",\"192.168.1.3\"]"
);

#[cfg(feature = "from-xml")]
test!(
    xml_with_attributes,
    &[
        "--from-xml",
        "this.user.name + ' <' + this.user.email + '>'"
    ],
    r#"<user>
  <name>Alice</name>
  <email>alice@example.com</email>
</user>"#,
    "\"Alice <alice@example.com>\""
);

#[cfg(feature = "from-xml")]
test!(
    xml_element_attributes,
    &["--from-xml", "this.user.name + ' #' + this.user['$'].id"],
    r#"<user id="42">
  <name>Alice</name>
</user>"#,
    "\"Alice #42\""
);

#[cfg(feature = "from-xml")]
test!(
    xml_simple_structure,
    &["--from-xml", "int(this.point.x) + int(this.point.y)"],
    r#"<point>
  <x>10</x>
  <y>20</y>
</point>"#,
    "30"
);

#[cfg(feature = "from-xml")]
test!(
    xml_text_content,
    &["--from-xml", "this.message"],
    r#"<message>Hello, World!</message>"#,
    r#""Hello, World!""#
);

#[cfg(feature = "from-xml")]
test!(
    xml_mixed_content,
    &[
        "--from-xml",
        "this.person.name + ' is ' + string(this.person.age)"
    ],
    r#"<person>
  <name>Bob</name>
  <age>25</age>
</person>"#,
    "\"Bob is 25\""
);

#[cfg(feature = "from-xml")]
test!(
    xml_to_pretty_sorted_json,
    &["--from-xml", "--sort-keys", "--pretty-print", "this"],
    r#"<root>
  <person>
    <name>Alice</name>
    <age>30</age>
  </person>
  <id>1</id>
</root>"#,
    r#"{
  "root": {
    "id": "1",
    "person": {
      "age": "30",
      "name": "Alice"
    }
  }
}"#
);

#[cfg(feature = "from-xml")]
test!(
    xml_empty_root_element,
    &["--from-xml", "this.empty == ''"],
    r#"<empty/>"#,
    "true"
);

#[cfg(feature = "from-xml")]
test!(
    xml_empty_child_element,
    &[
        "--from-xml",
        "this.config.enabled == '' && this.config.name == 'celq'"
    ],
    r#"<config>
  <enabled/>
  <name>celq</name>
</config>"#,
    "true"
);

#[cfg(feature = "from-xml")]
test!(
    xml_cdata_and_references,
    &["--from-xml", "this.message"],
    r#"<message><![CDATA[Use <tags>]]> &amp; &#65;</message>"#,
    r#""Use <tags> & A""#
);

#[cfg(feature = "from-xml")]
test!(
    xml_mixed_text_and_child_preserves_text_key,
    &["--from-xml", "this.article['_'] + '|' + this.article.title"],
    r#"<article>Intro <title>XML</title> outro</article>"#,
    r#""Intro  outro|XML""#
);

#[cfg(feature = "from-xml")]
test!(
    xml_declaration_comments_and_processing_instructions,
    &["--from-xml", "this.root.value"],
    r#"<?xml version="1.0"?>
<!-- ignored -->
<root><?format compact?><value>ok</value></root>"#,
    r#""ok""#
);
