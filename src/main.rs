use cel::Program;
use clap::Parser;
use std::io;
use std::process;

mod args2cel;
mod cel2json;
mod input_handler;
mod json2cel;

use args2cel::args_to_cel_variables;
pub use cel2json::cel_value_to_json_value;
use input_handler::handle_input;
pub use json2cel::json_to_cel_variables;

#[derive(Debug, Clone)]
struct Argument {
    name: String,
    type_name: String,
    value: String,
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
#[command(about = "CEL expression evaluator", long_about = None)]
struct Cli {
    /// Define argument variables, types, and (optional) values
    /// Format: name:type or name:type=value
    #[arg(short = 'a', long = "arg", value_name = "name:type=value")]
    args: Vec<Argument>,

    /// Return a status code based on boolean output
    /// true = 0, false = 1, exception = 2
    #[arg(short = 'b', long = "boolean")]
    boolean: bool,

    /// Do not read JSON input from stdin
    #[arg(short = 'n', long = "null-input")]
    null_input: bool,

    /// Treat all input as a single JSON document
    /// Default is to treat each line as separate NLJSON
    #[arg(short = 's', long = "slurp")]
    slurp: bool,

    /// Parallelism level (number of threads, -1 for all available)
    #[arg(
        short = 'j',
        long = "jobs",
        value_name = "N",
        default_value = "1",
        value_parser = parse_parallelism
    )]
    parallelism: i32,

    /// CEL expression to evaluate
    #[arg(value_name = "expr")]
    expression: String,
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

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Compile the CEL program
    let program = match Program::compile(&cli.expression) {
        Ok(prog) => prog,
        Err(parse_errors) => {
            for error in &parse_errors.errors {
                eprintln!("  Error: {:?}", error);
            }
            process::exit(2);
        }
    };

    // Convert CLI arguments to CEL variables
    let arg_tuples: Vec<(String, String, String)> = cli
        .args
        .iter()
        .map(|a| (a.name.clone(), a.type_name.clone(), a.value.clone()))
        .collect();

    let arg_variables = match args_to_cel_variables(&arg_tuples) {
        Ok(vars) => vars,
        Err(e) => {
            eprintln!("Argument conversion failed: {}", e);
            process::exit(2);
        }
    };

    match handle_input(
        &program,
        &arg_variables,
        cli.null_input,
        cli.slurp,
        cli.parallelism,
    ) {
        Ok(results) => {
            // Print all outputs
            for (output, _) in &results {
                println!("{}", output);
            }

            // If boolean mode is enabled, exit with appropriate code based on last result
            if cli.boolean {
                let is_truthy = results.last().map(|(_, truthy)| *truthy).unwrap_or(false);
                let exit_code = if is_truthy { 0 } else { 1 };
                process::exit(exit_code);
            }
        }
        Err(e) => {
            eprintln!("✗ Execution failed: {}", e);
            process::exit(2);
        }
    }

    Ok(())
}
