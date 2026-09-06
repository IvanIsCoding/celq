mod base;

#[cfg(feature = "greppable")]
use base::test;

// Greppable output tests
#[cfg(feature = "greppable")]
test!(
    greppable_simple,
    &["-g", "-S", "this"],
    r#"{"name":"Alice","age":30}"#,
    r#"json = {};
json.age = 30;
json.name = "Alice";
"#
);

#[cfg(feature = "greppable")]
test!(
    greppable_nested,
    &["--greppable", "-S", "this"],
    r#"{"person":{"name":"Bob","city":"NYC"},"id":1}"#,
    r#"json = {};
json.id = 1;
json.person = {};
json.person.city = "NYC";
json.person.name = "Bob";
"#
);

#[cfg(feature = "greppable")]
test!(
    greppable_ndjson_prints_only_last_result,
    &["--greppable", "-S", "this"],
    r#"{"a":1}
{"b":2}"#,
    r#"json = {};
json.b = 2;
"#
);

#[cfg(feature = "greppable")]
test!(
    greppable_json5,
    &["--greppable", "-S", "--from-json5", "this"],
    r#"{
  // Comment
  x: 10,
  y: 20,
}"#,
    r#"json = {};
json.x = 10;
json.y = 20;
"#
);

// Ungron (from-gron) tests
#[cfg(feature = "greppable")]
test!(
    from_gron_simple,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.age = 30;
json.name = "Alice";
"#,
    r#"{"age":30,"name":"Alice"}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_nested,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.id = 1;
json.person = {};
json.person.city = "NYC";
json.person.name = "Bob";
"#,
    r#"{"id":1,"person":{"city":"NYC","name":"Bob"}}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_arrays,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.items = [];
json.items[0] = "first";
json.items[1] = "second";
json.items[2] = "third";
"#,
    r#"{"items":["first","second","third"]}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_sparse_array,
    &["--from-gron", "-S", "this"],
    r#"json.likes = [];
json.likes[0] = "code";
json.likes[2] = "meat";
"#,
    r#"{"likes":["code",null,"meat"]}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_mixed_types,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.integer = 42;
json.float = 3.14;
json.negative = -5;
json.isTrue = true;
json.isFalse = false;
json.nothing = null;
"#,
    r#"{"float":3.14,"integer":42,"isFalse":false,"isTrue":true,"negative":-5,"nothing":null}"#
);

// Tricky edge cases
#[cfg(feature = "greppable")]
test!(
    from_gron_special_keys,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json["key-with-dashes"] = "value1";
json["key.with.dots"] = "value2";
json["key with spaces"] = "value3";
json["123numeric"] = "value4";
"#,
    r#"{"123numeric":"value4","key with spaces":"value3","key-with-dashes":"value1","key.with.dots":"value2"}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_identifier_edge_cases,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.$dollar = 1;
json._underscore = 2;
json.alpha$beta_gamma9 = 3;
"#,
    r#"{"$dollar":1,"_underscore":2,"alpha$beta_gamma9":3}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_single_quoted_path_segment,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json['User-Agent'] = "gron/0.1";
"#,
    r#"{"User-Agent":"gron/0.1"}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_single_quoted_value,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.value = 'single quoted';
"#,
    r#"{"value":"single quoted"}"#
);

#[cfg(feature = "greppable")]
test!(
    from_gron_deeply_nested,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json.a = {};
json.a.b = {};
json.a.b.c = {};
json.a.b.c.d = [];
json.a.b.c.d[0] = {};
json.a.b.c.d[0].e = "deep";
"#,
    r#"{"a":{"b":{"c":{"d":[{"e":"deep"}]}}}}"#
);

#[cfg(feature = "greppable")]
test!(
    greppable_round_trip_special_keys,
    &["--from-gron", "-S", "this"],
    r#"json = {};
json["key with spaces"] = "value";
json["key-with-dashes"] = "other";
"#,
    r#"{"key with spaces":"value","key-with-dashes":"other"}"#
);
