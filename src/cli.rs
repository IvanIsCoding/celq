// Adapted from the cel-python CLI documentation
// Original: https://github.com/cloud-custodian/cel-python/blob/3a134c10394058c73a6bbe0e4ca7e862ea9707b3/docs/source/cli.rst
// Copyright 2020 The Cloud Custodian Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License. You may obtain
// a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

use clap::ArgGroup;
use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Json,
    SlurpJson,
    Json5,
    Toml,
    Yaml,
    Gron,
}

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
    about = "A CEL command-line query tool for JSON data",
    version,
    long_about = None,
    group(
        ArgGroup::new("program")
            .args(&["expression", "from_file"])
    ),
    group(
        ArgGroup::new("input_format")
            .args(&["slurp", "from_json5", "from_toml", "from_yaml", "from_gron"])
    ),
    group(
        ArgGroup::new("output_style")
            .args(&["pretty_print", "greppable"])
    ),
    group(
        ArgGroup::new("output_options")
            .args(&["raw_output", "greppable"])
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

    /// Do not write JSON output to stdout
    #[arg(long = "void")]
    pub void: bool,

    /// Treat all input as a single JSON document
    /// Default is to treat each line as separate NDJSON
    #[arg(short = 's', long = "slurp")]
    pub slurp: bool,

    /// Parse input as JSON5 instead of JSON
    #[arg(long = "from-json5")]
    pub from_json5: bool,

    /// Parse input as TOML instead of JSON
    #[arg(long = "from-toml")]
    pub from_toml: bool,

    /// Parse input as YAML instead of JSON
    #[arg(long = "from-yaml")]
    pub from_yaml: bool,

    /// Parse input as gron (greppable output) instead of JSON
    #[arg(long = "from-gron")]
    pub from_gron: bool,

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

    /// If the output is a JSON string, output it raw without quotes
    #[arg(short = 'r', long = "raw-output")]
    pub raw_output: bool,

    /// Output the fields of each object with the keys in sorted order
    #[arg(short = 'S', long = "sort-keys")]
    pub sort_keys: bool,

    /// Read CEL expression from a file
    #[arg(short = 'f', long = "from-file", value_name = "FILE")]
    pub from_file: Option<std::path::PathBuf>,

    /// Output JSON with identation and line breaks for human readability
    #[arg(short = 'p', long = "pretty-print")]
    pub pretty_print: bool,

    /// Output in a greppable format (gron style)
    #[arg(short = 'g', long = "greppable")]
    pub greppable: bool,

    /// Disable extensions and use only standard CEL functions
    #[arg(long = "no-extensions")]
    pub no_extensions: bool,

    /// CEL expression to evaluate
    #[arg(value_name = "expr", default_value = "this")]
    pub expression: Option<String>,
}

impl Cli {
    pub fn input_format(&self) -> InputFormat {
        if self.from_gron {
            InputFormat::Gron
        } else if self.from_yaml {
            InputFormat::Yaml
        } else if self.from_toml {
            InputFormat::Toml
        } else if self.from_json5 {
            InputFormat::Json5
        } else if self.slurp {
            InputFormat::SlurpJson
        } else {
            InputFormat::Json
        }
    }
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
    pub input_format: InputFormat,
    pub parallelism: i32,
    pub sort_keys: bool,
    pub pretty_print: bool,
    pub raw_output: bool,
    pub greppable: bool,
    pub no_extensions: bool,
}
