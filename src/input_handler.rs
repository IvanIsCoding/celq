use cel::objects::Value as CelValue;
use cel::{Context, Program};
use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader, Cursor, Read};

use crate::cel_value_to_json_value;
use crate::json_to_cel_variables;

#[derive(Debug)]
pub enum HandleError {
    IoError(io::Error),
    JsonParseError(serde_json::Error),
    JsonSerializationError(serde_json::Error),

    ContextError(String),
    ExecutionError(cel::ExecutionError),
}

impl std::fmt::Display for HandleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandleError::IoError(e) => write!(f, "I/O error: {}", e),
            HandleError::JsonParseError(e) => write!(f, "JSON parse error: {}", e),
            HandleError::JsonSerializationError(e) => write!(f, "JSON serialization error: {}", e),

            HandleError::ContextError(e) => write!(f, "Context error: {}", e),
            HandleError::ExecutionError(e) => write!(f, "Execution error: {:?}", e),
        }
    }
}

impl std::error::Error for HandleError {}

impl From<io::Error> for HandleError {
    fn from(err: io::Error) -> Self {
        HandleError::IoError(err)
    }
}

impl From<serde_json::Error> for HandleError {
    fn from(err: serde_json::Error) -> Self {
        HandleError::JsonParseError(err)
    }
}

impl From<cel::ExecutionError> for HandleError {
    fn from(err: cel::ExecutionError) -> Self {
        HandleError::ExecutionError(err)
    }
}

/// Execute the CEL program with given JSON input and argument variables
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `json_str` - Optional JSON string to process
///
/// # Returns
/// * Ok((output_string, is_truthy)) - The output and whether it's truthy
/// * Err(HandleError) - Any error that occurred
fn handle_json(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    json_str: Option<&str>,
) -> Result<(String, bool), HandleError> {
    // Create context with default values
    let mut context = Context::default();

    // Add argument variables to context
    for (name, value) in arg_variables {
        context
            .add_variable(name.clone(), value.clone())
            .map_err(|e| {
                HandleError::ContextError(format!("Failed to add variable '{}': {:?}", name, e))
            })?;
    }

    // If we have input, parse it as JSON and add to context
    if let Some(json) = json_str {
        let json_variables = json_to_cel_variables(json)?;

        // Add JSON variables to context
        for (name, value) in json_variables {
            context.add_variable(name.clone(), value).map_err(|e| {
                HandleError::ContextError(format!(
                    "Failed to add JSON variable '{}': {:?}",
                    name, e
                ))
            })?;
        }
    }

    // Execute the program
    let result = program.execute(&context)?;

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
        serde_json::to_string(&json_value).map_err(HandleError::JsonSerializationError)?;

    Ok((output_string, is_truthy))
}

/// Process input from a BufReader and execute the CEL program
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `reader` - BufReader to read input from
/// * `slurp` - If true, treat all input as a single JSON document
///
/// # Returns
/// * Ok(Vec<(output_string, is_truthy)>) - Vector of outputs and their truthiness
/// * Err(HandleError) - Any error that occurred
fn handle_buffer<R: Read>(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    reader: BufReader<R>,
    slurp: bool,
) -> Result<Vec<(String, bool)>, HandleError> {
    if slurp {
        // Read all input as a single document
        let mut buffer = String::new();
        for line in reader.lines() {
            let line = line?;
            buffer.push_str(&line);
            buffer.push('\n');
        }

        // Process the entire buffer as one JSON document
        let result = handle_json(program, arg_variables, Some(&buffer))?;
        Ok(vec![result])
    } else {
        // Process line by line (NLJSON mode)
        let mut results = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue; // Skip empty lines
            }

            let result = handle_json(program, arg_variables, Some(&line))?;
            results.push(result);
        }

        // If no lines were processed, execute with no input
        if results.is_empty() {
            let result = handle_json(program, arg_variables, None)?;
            results.push(result);
        }

        Ok(results)
    }
}

/// Process input from stdin and execute the CEL program
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `null_input` - If true, don't read from stdin
/// * `slurp` - If true, treat all input as a single JSON document
///
/// # Returns
/// * Ok(Vec<(output_string, is_truthy)>) - Vector of outputs and their truthiness
/// * Err(HandleError) - Any error that occurred
pub fn handle_input(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    null_input: bool,
    slurp: bool,
) -> Result<Vec<(String, bool)>, HandleError> {
    if null_input {
        // No input from stdin - use empty cursor
        let empty_cursor = Cursor::new(Vec::<u8>::new());
        let reader = BufReader::new(empty_cursor);
        handle_buffer(program, arg_variables, reader, slurp)
    } else {
        // Read from stdin
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());
        handle_buffer(program, arg_variables, reader, slurp)
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
        assert!(matches!(
            result.unwrap_err(),
            HandleError::JsonParseError(_)
        ));
    }

    #[test]
    fn test_handle_json_missing_variable() {
        let program = Program::compile("missing_var").unwrap();
        let args = BTreeMap::new();

        let result = handle_json(&program, &args, None);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HandleError::ExecutionError(_)
        ));
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
