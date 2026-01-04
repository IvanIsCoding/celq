use anyhow::{Context, Result};
use cel::Program;
use cel::parser::ParseErrors;
use clap::Parser;
use std::path::PathBuf;
use std::{fs, io, process};

mod args2cel;
mod cel2json;
mod cli;
mod input_handler;
mod json2cel;

use args2cel::args_to_cel_variables;
pub use cel2json::cel_value_to_json_value;
pub use cli::Argument;
use cli::Cli;
pub use cli::InputParameters;
use input_handler::handle_input;
pub use json2cel::json_to_cel_variables;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Compile the CEL program
    let program = match compile_expression(cli.expression.as_deref(), cli.from_file.as_ref()) {
        Ok(prog) => prog,
        Err(err) => {
            if let Some(parse_errors) = err.downcast_ref::<ParseErrors>() {
                for error in &parse_errors.errors {
                    eprintln!("  Error: {:?}", error);
                }
            } else {
                eprintln!("Error: {err:#}");
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

    let input_params = InputParameters {
        root_var: cli.root_var,
        null_input: cli.null_input,
        slurp: cli.slurp,
        from_json5: cli.from_json5,
        parallelism: cli.parallelism,
        sort_keys: cli.sort_keys,
        pretty_print: cli.pretty_print,
    };

    match handle_input(&program, &arg_variables, &input_params) {
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

fn compile_expression(expression: Option<&str>, from_file: Option<&PathBuf>) -> Result<Program> {
    let source = if let Some(path) = from_file {
        fs::read_to_string(path)
            .with_context(|| format!("failed to read expression file `{}`", path.display()))?
    } else {
        expression.context("missing CEL expression")?.to_owned()
    };

    Program::compile(&source).map_err(|e| anyhow::anyhow!(e))
}
