use anyhow::{Context as AnyhowContext, Result};
use cel::objects::Value as CelValue;
use cel::{Context, Program};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader, Cursor, Read};

use crate::cel_value_to_json_value;
use crate::json_to_cel_variables;

/// Execute the CEL program with given JSON input and argument variables
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `json_str` - Optional JSON string to process
///
/// # Returns
/// * Ok((output_string, is_truthy)) - The output and whether it's truthy
/// * Err(anyhow::Error) - Any error that occurred
fn handle_json(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    json_str: Option<&str>,
) -> Result<(String, bool)> {
    // Create context with default values
    let mut context = Context::default();

    // Add argument variables to context
    for (name, value) in arg_variables {
        context
            .add_variable(name.clone(), value.clone())
            .with_context(|| format!("Failed to add variable '{}'", name))?;
    }

    // If we have input, parse it as JSON and add to context
    if let Some(json) = json_str {
        let json_variables = json_to_cel_variables(json).context("Failed to parse JSON input")?;

        // Add JSON variables to context
        for (name, value) in json_variables {
            context
                .add_variable(name.clone(), value)
                .with_context(|| format!("Failed to add JSON variable '{}'", name))?;
        }
    }

    // Execute the program
    let result = program
        .execute(&context)
        .context("Failed to execute CEL program")?;

    // Determine if the result is truthy
    let is_truthy = match result {
        CelValue::Bool(b) => b,
        CelValue::Int(i) => i != 0,
        CelValue::UInt(u) => u != 0,
        CelValue::Float(f) => f != 0.0 && !f.is_nan(),
        CelValue::String(ref s) => !s.is_empty(),
        CelValue::List(ref l) => !l.is_empty(),
        CelValue::Map(ref m) => !m.map.is_empty(),
        CelValue::Null => false,
        _ => true, // Other types are considered truthy
    };

    // Convert result to JSON string
    let json_value = cel_value_to_json_value(&result);
    let output_string =
        serde_json::to_string(&json_value).context("Failed to serialize result to JSON")?;

    Ok((output_string, is_truthy))
}

/// Process input from a BufReader and execute the CEL program
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `reader` - BufReader to read input from
/// * `slurp` - If true, treat all input as a single JSON document
/// * `parallelism` - Number of threads (-1 for all available)
///
/// # Returns
/// * Ok(Vec<(output_string, is_truthy)>) - Vector of outputs and their truthiness
/// * Err(anyhow::Error) - Any error that occurred
fn handle_buffer<R: Read>(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    reader: BufReader<R>,
    slurp: bool,
    parallelism: i32,
) -> Result<Vec<(String, bool)>> {
    if !slurp {
        // Determine thread pool size
        anyhow::ensure!(parallelism != 0, "Parallelism level cannot be 0");

        let num_threads = if parallelism == -1 {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        } else {
            parallelism as usize
        };

        // Collect all non-empty lines first
        let lines: Vec<String> = reader
            .lines()
            .collect::<std::io::Result<Vec<_>>>()
            .context("Failed to read lines from input")?
            .into_iter()
            .filter(|line| !line.trim().is_empty())
            .collect();

        // If no lines were processed, execute with no input
        if lines.is_empty() {
            let result = handle_json(program, arg_variables, None)?;
            return Ok(vec![result]);
        }

        // Process lines in parallel, preserving order
        let results: Result<Vec<_>> = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .context("Failed to build thread pool")?
            .install(|| {
                lines
                    .par_iter()
                    .map(|line| handle_json(program, arg_variables, Some(line)))
                    .collect()
            });

        results
    } else {
        // Read all input as a single document
        let mut buffer = String::new();
        for line in reader.lines() {
            let line = line.context("Failed to read line from input")?;
            buffer.push_str(&line);
            buffer.push('\n');
        }

        // Process the entire buffer as one JSON document
        let result = handle_json(program, arg_variables, Some(&buffer))?;
        Ok(vec![result])
    }
}

/// Process input from stdin and execute the CEL program
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `null_input` - If true, don't read from stdin
/// * `slurp` - If true, treat all input as a single JSON document
/// * `parallelism` - Number of threads to use for parallel processing (-1 for all available)
///
/// # Returns
/// * Ok(Vec<(output_string, is_truthy)>) - Vector of outputs and their truthiness
/// * Err(anyhow::Error) - Any error that occurred
pub fn handle_input(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    null_input: bool,
    slurp: bool,
    parallelism: i32,
) -> Result<Vec<(String, bool)>> {
    if null_input {
        // No input from stdin - use empty cursor
        let empty_cursor = Cursor::new(Vec::<u8>::new());
        let reader = BufReader::new(empty_cursor);
        handle_buffer(program, arg_variables, reader, slurp, parallelism)
    } else {
        // Read from stdin
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());
        handle_buffer(program, arg_variables, reader, slurp, parallelism)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cel::Program;

    #[test]
    fn test_handle_json_null_input() {
        let program = Program::compile("2 + 3").unwrap();
        let args = BTreeMap::new();

        let (output, is_truthy) = handle_json(&program, &args, None).unwrap();

        assert!(output.contains("5"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_json_with_json() {
        let program = Program::compile(".x + .y").unwrap();
        let args = BTreeMap::new();
        let json = r#"{"x": 10, "y": 20}"#;

        let (output, is_truthy) = handle_json(&program, &args, Some(json)).unwrap();

        assert!(output.contains("30"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_json_with_args() {
        let program = Program::compile("x + y").unwrap();
        let mut args = BTreeMap::new();
        args.insert("x".to_string(), CelValue::Int(5));
        args.insert("y".to_string(), CelValue::Int(7));

        let (output, is_truthy) = handle_json(&program, &args, None).unwrap();

        assert!(output.contains("12"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_json_args_and_json() {
        let program = Program::compile("x + .value").unwrap();
        let mut args = BTreeMap::new();
        args.insert("x".to_string(), CelValue::Int(100));
        let json = r#"{"value": 50}"#;

        let (output, is_truthy) = handle_json(&program, &args, Some(json)).unwrap();

        assert!(output.contains("150"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_json_boolean_false() {
        let program = Program::compile("2 > 5").unwrap();
        let args = BTreeMap::new();

        let (output, is_truthy) = handle_json(&program, &args, None).unwrap();

        assert!(output.contains("false"));
        assert!(!is_truthy);
    }

    #[test]
    fn test_handle_json_boolean_true() {
        let program = Program::compile("5 > 2").unwrap();
        let args = BTreeMap::new();

        let (output, is_truthy) = handle_json(&program, &args, None).unwrap();

        assert!(output.contains("true"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_json_truthiness_zero() {
        let program = Program::compile("0").unwrap();
        let args = BTreeMap::new();

        let (_output, is_truthy) = handle_json(&program, &args, None).unwrap();

        assert!(!is_truthy);
    }

    #[test]
    fn test_handle_json_truthiness_empty_string() {
        let program = Program::compile(r#""""#).unwrap();
        let args = BTreeMap::new();

        let (_output, is_truthy) = handle_json(&program, &args, None).unwrap();

        assert!(!is_truthy);
    }

    #[test]
    fn test_handle_json_invalid_json() {
        let program = Program::compile(".x").unwrap();
        let args = BTreeMap::new();
        let json = r#"not valid json"#;

        let result = handle_json(&program, &args, Some(json));

        assert!(result.is_err());
    }

    #[test]
    fn test_handle_json_missing_variable() {
        let program = Program::compile("missing_var").unwrap();
        let args = BTreeMap::new();

        let result = handle_json(&program, &args, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_handle_buffer_single_line() {
        let program = Program::compile(".x").unwrap();
        let args = BTreeMap::new();
        let input = r#"{"x": 42}"#;
        let cursor = Cursor::new(input.as_bytes());
        let reader = BufReader::new(cursor);

        let results = handle_buffer(&program, &args, reader, false).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].0.contains("42"));
        assert!(results[0].1);
    }

    #[test]
    fn test_handle_buffer_multiple_lines() {
        let program = Program::compile(".x").unwrap();
        let args = BTreeMap::new();
        let input = r#"{"x": 1}
{"x": 2}
{"x": 3}"#;
        let cursor = Cursor::new(input.as_bytes());
        let reader = BufReader::new(cursor);

        let results = handle_buffer(&program, &args, reader, false).unwrap();

        assert_eq!(results.len(), 3);
        assert!(results[0].0.contains("1"));
        assert!(results[1].0.contains("2"));
        assert!(results[2].0.contains("3"));
    }

    #[test]
    fn test_handle_buffer_slurp() {
        let program = Program::compile(".x + .y").unwrap();
        let args = BTreeMap::new();
        let input = r#"{"x": 10,
"y": 20}"#;
        let cursor = Cursor::new(input.as_bytes());
        let reader = BufReader::new(cursor);

        let results = handle_buffer(&program, &args, reader, true).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].0.contains("30"));
        assert!(results[0].1);
    }

    #[test]
    fn test_handle_buffer_empty_input() {
        let program = Program::compile("2 + 3").unwrap();
        let args = BTreeMap::new();
        let cursor = Cursor::new(Vec::<u8>::new());
        let reader = BufReader::new(cursor);

        let results = handle_buffer(&program, &args, reader, false).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].0.contains("5"));
        assert!(results[0].1);
    }

    #[test]
    fn test_handle_input_null_input() {
        let program = Program::compile("2 + 3").unwrap();
        let args = BTreeMap::new();

        let results = handle_input(&program, &args, true, false).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].0.contains("5"));
        assert!(results[0].1);
    }

    #[test]
    fn test_handle_buffer_skip_empty_lines() {
        let program = Program::compile(".x").unwrap();
        let args = BTreeMap::new();
        let input = r#"{"x": 1}

{"x": 2}
   
{"x": 3}"#;
        let cursor = Cursor::new(input.as_bytes());
        let reader = BufReader::new(cursor);

        let results = handle_buffer(&program, &args, reader, false).unwrap();

        assert_eq!(results.len(), 3);
        assert!(results[0].0.contains("1"));
        assert!(results[1].0.contains("2"));
        assert!(results[2].0.contains("3"));
    }
}
