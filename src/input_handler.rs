use cel::objects::Value as CelValue;
use cel::{Context, Program};
use std::collections::BTreeMap;
use std::io::{self, BufRead};

use crate::json_to_cel_variables;

#[derive(Debug)]
pub enum HandleError {
    IoError(io::Error),
    JsonParseError(serde_json::Error),
    ContextError(String),
    ExecutionError(cel::ExecutionError),
}

impl std::fmt::Display for HandleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandleError::IoError(e) => write!(f, "I/O error: {}", e),
            HandleError::JsonParseError(e) => write!(f, "JSON parse error: {}", e),
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

/// Process a buffer (JSON string or None) and execute the CEL program
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `buffer` - Optional JSON string to process
///
/// # Returns
/// * Ok((output_string, is_truthy)) - The output and whether it's truthy
/// * Err(HandleError) - Any error that occurred
fn handle_buffer(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    buffer: Option<String>,
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
    if let Some(json_str) = buffer {
        let json_variables = json_to_cel_variables(&json_str)?;

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

    // Convert result to string
    let output_string = format!("{:?}", result);

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

    Ok((output_string, is_truthy))
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
/// * Ok((output_string, is_truthy)) - The output and whether it's truthy
/// * Err(HandleError) - Any error that occurred
pub fn handle_input(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    null_input: bool,
    slurp: bool,
) -> Result<(String, bool), HandleError> {
    let buffer = if null_input {
        // No input from stdin
        None
    } else if slurp {
        // Read all input as a single document
        let stdin = io::stdin();
        let reader = stdin.lock();
        let mut buffer = String::new();

        for line in reader.lines() {
            let line = line?;
            buffer.push_str(&line);
            buffer.push('\n');
        }

        Some(buffer)
    } else {
        // Read line by line (NLJSON mode)
        // For now, we'll process just the first line
        // TODO: Handle multiple lines separately
        let stdin = io::stdin();
        let reader = stdin.lock();

        if let Some(line) = reader.lines().next() {
            Some(line?)
        } else {
            None
        }
    };

    handle_buffer(program, arg_variables, buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cel::Program;

    #[test]
    fn test_handle_buffer_null_input() {
        let program = Program::compile("2 + 3").unwrap();
        let args = BTreeMap::new();

        let (output, is_truthy) = handle_buffer(&program, &args, None).unwrap();

        assert!(output.contains("5"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_buffer_with_json() {
        let program = Program::compile(".x + .y").unwrap();
        let args = BTreeMap::new();
        let json = r#"{"x": 10, "y": 20}"#.to_string();

        let (output, is_truthy) = handle_buffer(&program, &args, Some(json)).unwrap();

        assert!(output.contains("30"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_buffer_with_args() {
        let program = Program::compile("x + y").unwrap();
        let mut args = BTreeMap::new();
        args.insert("x".to_string(), CelValue::Int(5));
        args.insert("y".to_string(), CelValue::Int(7));

        let (output, is_truthy) = handle_buffer(&program, &args, None).unwrap();

        assert!(output.contains("12"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_buffer_args_and_json() {
        let program = Program::compile("x + .value").unwrap();
        let mut args = BTreeMap::new();
        args.insert("x".to_string(), CelValue::Int(100));
        let json = r#"{"value": 50}"#.to_string();

        let (output, is_truthy) = handle_buffer(&program, &args, Some(json)).unwrap();

        assert!(output.contains("150"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_buffer_boolean_false() {
        let program = Program::compile("2 > 5").unwrap();
        let args = BTreeMap::new();

        let (output, is_truthy) = handle_buffer(&program, &args, None).unwrap();

        assert!(output.contains("false"));
        assert!(!is_truthy);
    }

    #[test]
    fn test_handle_buffer_boolean_true() {
        let program = Program::compile("5 > 2").unwrap();
        let args = BTreeMap::new();

        let (output, is_truthy) = handle_buffer(&program, &args, None).unwrap();

        assert!(output.contains("true"));
        assert!(is_truthy);
    }

    #[test]
    fn test_handle_buffer_truthiness_zero() {
        let program = Program::compile("0").unwrap();
        let args = BTreeMap::new();

        let (_output, is_truthy) = handle_buffer(&program, &args, None).unwrap();

        assert!(!is_truthy);
    }

    #[test]
    fn test_handle_buffer_truthiness_empty_string() {
        let program = Program::compile(r#""""#).unwrap();
        let args = BTreeMap::new();

        let (_output, is_truthy) = handle_buffer(&program, &args, None).unwrap();

        assert!(!is_truthy);
    }

    #[test]
    fn test_handle_buffer_invalid_json() {
        let program = Program::compile(".x").unwrap();
        let args = BTreeMap::new();
        let json = r#"not valid json"#.to_string();

        let result = handle_buffer(&program, &args, Some(json));

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HandleError::JsonParseError(_)
        ));
    }

    #[test]
    fn test_handle_buffer_missing_variable() {
        let program = Program::compile("missing_var").unwrap();
        let args = BTreeMap::new();

        let result = handle_buffer(&program, &args, None);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HandleError::ExecutionError(_)
        ));
    }
}
