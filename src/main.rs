use cel::Program;
use clap::Parser;
use std::io;
use std::process;

mod args2cel;
mod cel2json;
mod cli;
mod input_handler;
mod json2cel;

use args2cel::args_to_cel_variables;
pub use cel2json::cel_value_to_json_value;
pub use cli::Argument;
use cli::Cli;
use input_handler::handle_input;
pub use json2cel::json_to_cel_variables;

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
