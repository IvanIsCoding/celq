mod base;

#[cfg(feature = "from-toml")]
use base::test;

// TOML tests

// TOML with nested tables
#[cfg(feature = "from-toml")]
test!(
    toml_nested_table,
    &[
        "--from-toml",
        "this.database.host + ':' + string(this.database.port)"
    ],
    r#"[database]
host = "localhost"
port = 5432
# Database configuration
"#,
    "\"localhost:5432\""
);

#[cfg(feature = "from-toml")]
test!(
    toml_array_of_tables,
    &["--from-toml", "this.servers.map(s, s.ip)"],
    r#"[[servers]]
ip = "192.168.1.1"
name = "alpha"

[[servers]]
ip = "192.168.1.2"
name = "beta"
"#,
    "[\"192.168.1.1\",\"192.168.1.2\"]"
);

#[cfg(feature = "from-toml")]
test!(
    toml_dotted_keys,
    &[
        "--from-toml",
        "this.user.name + ' <' + this.user.email + '>'"
    ],
    r#"user.name = "Alice"
user.email = "alice@example.com"
"#,
    "\"Alice <alice@example.com>\""
);

#[cfg(feature = "from-toml")]
test!(
    toml_inline_table,
    &["--from-toml", "this.point.x + this.point.y"],
    r#"point = { x = 10, y = 20 }
"#,
    "30"
);
