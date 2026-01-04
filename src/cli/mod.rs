// Adapted from the cel-python CLI documentation
// Original: https://github.com/cloud-custodian/cel-python/blob/3a134c10394058c73a6bbe0e4ca7e862ea9707b3/docs/source/cli.rst
// Copyright 2020 The Cloud Custodian Authors.
// SPDX-License-Identifier: Apache-2.0
use clap::ArgGroup;
use clap::Parser;

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub type_name: String,
    pub value: String,
}

impl std::str::FromStr for Argument {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Format: name:type=value
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid argument format '{}'. Expected 'name:type=value'",
                s
            ));
        }

        let name = parts[0].to_string();
        let type_and_value = parts[1];

        let eq_pos = type_and_value.find('=').ok_or_else(|| {
            format!(
                "Missing value for argument '{}'. Expected 'name:type=value'",
                name
            )
        })?;

        let (type_name, value_with_eq) = type_and_value.split_at(eq_pos);
        let value = value_with_eq[1..].to_string(); // Skip the '=' character

        Ok(Argument {
            name,
            type_name: type_name.to_string(),
            value,
        })
    }
}

#[derive(Parser, Debug)]
#[command(name = "celq")]
#[command(
    name = "celq",
    about = "CEL expression evaluator",
    long_about = None,
    group(
        ArgGroup::new("program")
            .required(true)
            .args(&["expression", "from_file"])
    ),
    group(
        ArgGroup::new("input_format")
            .args(&["slurp", "from_json5"])
    )
)]
pub struct Cli {
    /// Define argument variables, types, and values.
    /// Format: name:type=value.
    /// Supported types: int, uint, float, bool, string
    #[arg(short = 'a', long = "arg", value_name = "name:type=value")]
    pub args: Vec<Argument>,

    /// Return a status code based on boolean output
    /// true = 0, false = 1, exception = 2
    #[arg(short = 'b', long = "boolean")]
    pub boolean: bool,

    /// Do not read JSON input from stdin
    #[arg(short = 'n', long = "null-input")]
    pub null_input: bool,

    /// Treat all input as a single JSON document
    /// Default is to treat each line as separate NLJSON
    #[arg(short = 's', long = "slurp")]
    pub slurp: bool,

    /// Parse input as JSON5 instead of JSON
    #[arg(long = "from-json5")]
    pub from_json5: bool,

    /// Parallelism level for NDJSON inputs (number of threads, -1 for all available)
    #[arg(
        short = 'j',
        long = "jobs",
        value_name = "N",
        default_value = "1",
        value_parser = parse_parallelism
    )]
    pub parallelism: i32,

    /// Variable name for the root JSON input
    #[arg(short = 'R', long = "root-var", default_value = "this")]
    pub root_var: String,

    /// Output the fields of each object with the keys in sorted order
    #[arg(short = 'S', long = "sort-keys")]
    pub sort_keys: bool,

    /// Read CEL expression from a file
    #[arg(short = 'f', long = "from-file", value_name = "FILE")]
    pub from_file: Option<std::path::PathBuf>,

    #[arg(short = 'p', long = "pretty-print")]
    pub pretty_print: bool,

    /// CEL expression to evaluate
    #[arg(value_name = "expr")]
    pub expression: Option<String>,
}

fn parse_parallelism(s: &str) -> Result<i32, String> {
    let value: i32 = s
        .parse()
        .map_err(|_| format!("'{}' is not a valid integer", s))?;

    if value == 0 {
        Err("parallelism level cannot be 0".to_string())
    } else if value < 0 {
        Ok(-1)
    } else {
        Ok(value)
    }
}

#[derive(Clone, Debug)]
pub struct InputParameters {
    pub root_var: String,
    pub null_input: bool,
    pub slurp: bool,
    pub from_json5: bool,
    pub parallelism: i32,
    pub sort_keys: bool,
    pub pretty_print: bool,
}
