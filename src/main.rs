use cel::Program;
use clap::Parser;
use std::io::{self, BufRead};
use std::process;

mod json2cel;
use json2cel::json_to_cel_variables;

#[derive(Debug, Clone)]
struct Argument {
    name: String,
    type_name: String,
    value: Option<String>,
}

impl std::str::FromStr for Argument {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Format: name:type or name:type=value
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid argument format '{}'. Expected 'name:type=value'",
                s
            ));
        }

        let name = parts[0].to_string();
        let type_and_value = parts[1];

        let (type_name, value) = if let Some(eq_pos) = type_and_value.find('=') {
            let (t, v) = type_and_value.split_at(eq_pos);
            (t.to_string(), Some(v[1..].to_string())) // Skip the '=' character
        } else {
            (type_and_value.to_string(), None)
        };

        Ok(Argument {
            name,
            type_name,
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

    /// CEL expression to evaluate
    #[arg(value_name = "expr")]
    expression: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    println!("Parsed CLI arguments:");
    println!("  Expression: {:?}", cli.expression);
    println!("  Arguments: {:?}", cli.args);
    println!("  Boolean mode: {}", cli.boolean);
    println!("  Null input: {}", cli.null_input);
    println!("  Slurp mode: {}", cli.slurp);

    println!("\nArguments:");
    for arg in &cli.args {
        println!("  {} ({}): {:?}", arg.name, arg.type_name, arg.value);
    }

    // Compile the CEL program
    println!("\nCompiling CEL expression: {}", cli.expression);
    let program = match Program::compile(&cli.expression) {
        Ok(prog) => {
            println!("✓ Program compiled successfully");
            prog
        }
        Err(parse_errors) => {
            eprintln!("✗ Failed to compile CEL expression:");
            for error in &parse_errors.errors {
                eprintln!("  Error: {:?}", error);
            }
            process::exit(2);
        }
    };

    // Read input from stdin unless null_input
    if !cli.null_input {
        println!("\nReading JSON input from stdin...");
        let stdin = io::stdin();
        let reader = stdin.lock();

        if cli.slurp {
            // Read all input as a single document
            let mut buffer = String::new();
            for line in reader.lines() {
                let line = line?;
                buffer.push_str(&line);
                buffer.push('\n');
            }
            println!(
                "Slurped input ({} bytes): {}",
                buffer.len(),
                if buffer.len() > 100 {
                    format!("{}...", &buffer[..100])
                } else {
                    buffer.clone()
                }
            );
        } else {
            // Read each line as a separate NLJSON document
            println!("Reading NLJSON documents (one per line):");
            for (i, line) in reader.lines().enumerate() {
                let line = line?;
                println!(
                    "  Document {}: {}",
                    i + 1,
                    if line.len() > 100 {
                        format!("{}...", &line[..100])
                    } else {
                        line
                    }
                );
            }
        }
    } else {
        println!("\nNull input mode: not reading from stdin");
    }

    Ok(())
}
