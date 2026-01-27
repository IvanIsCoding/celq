use anyhow::Context as AnyhowContext;
use anyhow::Result;
use cel::Context;
use cel::Program;
use cel::objects::Value as CelValue;
use rayon::prelude::*;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::io::{self};

use crate::InputFormat;
use crate::InputParameters;
use crate::json_to_cel_variables;
use crate::slice_extension;

/// Process input from stdin and execute the CEL program
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `input_params` - Input configuration parameters
///
/// # Returns
/// * Ok(Vec<(output_string, is_truthy)>) - Vector of outputs and their truthiness
/// * Err(anyhow::Error) - Any error that occurred
pub fn handle_input(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    input_params: &InputParameters,
) -> Result<Vec<(String, bool)>> {
    if !input_params.null_input {
        // Read from stdin
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());
        handle_buffer(program, arg_variables, input_params, reader)
    } else {
        // No input from stdin - use empty cursor
        let empty_cursor = Cursor::new(Vec::<u8>::new());
        let reader = BufReader::new(empty_cursor);
        handle_buffer(program, arg_variables, input_params, reader)
    }
}

/// Process input from a BufReader and execute the CEL program
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `input_params` - Input configuration parameters
/// * `reader` - BufReader to read input from
///
/// # Returns
/// * Ok(Vec<(output_string, is_truthy)>) - Vector of outputs and their truthiness
/// * Err(anyhow::Error) - Any error that occurred
fn handle_buffer<R: Read>(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    input_params: &InputParameters,
    reader: BufReader<R>,
) -> Result<Vec<(String, bool)>> {
    // Check if we're processing JSON or NDJSON that is not slurped
    if input_params.input_format == InputFormat::Json {
        // Determine thread pool size
        anyhow::ensure!(
            input_params.parallelism != 0,
            "Parallelism level cannot be 0"
        );

        let num_threads = if input_params.parallelism <= -1 {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        } else {
            input_params.parallelism as usize
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
            let result = handle_json(program, arg_variables, input_params, None)?;
            return Ok(vec![result]);
        }

        // Try to process the last line
        let last_idx = lines.len() - 1;
        let last_result = handle_json(program, arg_variables, input_params, Some(&lines[last_idx]));

        match last_result {
            Ok(last_output) => {
                // Last line succeeded, process remaining lines in parallel
                if lines.len() == 1 {
                    return Ok(vec![last_output]);
                }

                let remaining_results: Result<Vec<_>> = if num_threads == 1 {
                    // Use regular iterator for single-threaded execution
                    lines[..last_idx]
                        .iter()
                        .map(|line| handle_json(program, arg_variables, input_params, Some(line)))
                        .collect()
                } else {
                    // Use Rayon for parallel execution
                    rayon::ThreadPoolBuilder::new()
                        .num_threads(num_threads)
                        .build()
                        .context("Failed to build thread pool")?
                        .install(|| {
                            lines[..last_idx]
                                .par_iter()
                                .map(|line| {
                                    handle_json(program, arg_variables, input_params, Some(line))
                                })
                                .collect()
                        })
                };

                let mut results = remaining_results?;
                results.push(last_output);
                Ok(results)
            }
            Err(_) => {
                // Last line failed, try reading entire input as single JSON document
                let full_buffer = lines.join("\n");
                let result = handle_json(program, arg_variables, input_params, Some(&full_buffer))?;
                Ok(vec![result])
            }
        }
    } else {
        anyhow::ensure!(
            input_params.parallelism != 0,
            "Parallelism level cannot be 0"
        );

        // Read all input as a single document
        let mut buffer = String::new();
        for line in reader.lines() {
            let line = line.context("Failed to read line from input")?;
            buffer.push_str(&line);
            buffer.push('\n');
        }

        // Process the entire buffer as one document
        let result = handle_json(program, arg_variables, input_params, Some(&buffer))?;
        Ok(vec![result])
    }
}

/// Execute the CEL program with given JSON input and argument variables
///
/// # Arguments
/// * `program` - The compiled CEL program
/// * `arg_variables` - BTreeMap of variables from CLI arguments
/// * `input_params` - Input configuration parameters
/// * `json_str` - Optional JSON string to process
///
/// # Returns
/// * Ok((output_string, is_truthy)) - The output and whether it's truthy
/// * Err(anyhow::Error) - Any error that occurred
fn handle_json(
    program: &Program,
    arg_variables: &BTreeMap<String, CelValue>,
    input_params: &InputParameters,
    json_str: Option<&str>,
) -> Result<(String, bool)> {
    // Create context with default values
    let mut context = Context::default();

    if !input_params.no_extensions {
        context.add_function("slice", slice_extension::slice);
    }

    // Add argument variables to context
    for (name, value) in arg_variables {
        context
            .add_variable(name.clone(), value.clone())
            .with_context(|| format!("Failed to add variable '{}'", name))?;
    }

    // If we have input, parse it and add to context
    if let Some(json) = json_str {
        let json_variables = json_to_cel_variables(
            json,
            &input_params.root_var,
            input_params.input_format,
            input_params.parallelism,
        )
        .context("Failed to parse input")?;

        // Add JSON variables to context
        for (name, value) in json_variables {
            context
                .add_variable(name.clone(), value)
                .with_context(|| format!("Failed to add input variable '{}'", name))?;
        }
    }

    // Execute the program
    let result = program
        .execute(&context)
        .context("Failed to execute CEL program")?;

    // Determine if the result is truthy
    let is_truthy = is_cel_value_truthy(&result);

    // Convert result to JSON string
    let output_string = handle_json_output(&result, input_params)?;

    Ok((output_string, is_truthy))
}

fn handle_json_output(result: &CelValue, input_params: &InputParameters) -> Result<String> {
    let mut json_value = result.json().map_err(|e| anyhow::anyhow!(e.to_string()))?;

    if input_params.sort_keys {
        sort_keys_recursive(&mut json_value);
    }

    // Handle greppable output
    if input_params.greppable {
        #[cfg(feature = "greppable")]
        {
            let gron_output = crate::gron::json_to_gron(&json_value);
            return Ok(gron_output);
        }

        #[cfg(not(feature = "greppable"))]
        {
            anyhow::bail!("Binary was compiled without greppable support",);
        }
    }

    // Handle regular output
    let output_string = if let serde_json::Value::String(s) = &json_value {
        if input_params.raw_output {
            s.clone()
        } else {
            serde_json::to_string(&json_value).context("Failed to serialize result to JSON")?
        }
    } else if input_params.pretty_print {
        serde_json::to_string_pretty(&json_value).context("Failed to serialize result to JSON")?
    } else {
        serde_json::to_string(&json_value).context("Failed to serialize result to JSON")?
    };

    Ok(output_string)
}

/// Determine if a CEL value is truthy
///
/// # Arguments
/// * `value` - The CEL value to check
///
/// # Returns
/// * `true` if the value is considered truthy, `false` otherwise
fn is_cel_value_truthy(value: &CelValue) -> bool {
    match value {
        CelValue::Bool(b) => *b,
        CelValue::Int(i) => *i != 0,
        CelValue::UInt(u) => *u != 0,
        CelValue::Float(f) => *f != 0.0 && !f.is_nan(),
        CelValue::String(s) => !s.is_empty(),
        CelValue::List(l) => !l.is_empty(),
        CelValue::Map(m) => !m.map.is_empty(),
        CelValue::Null => false,
        _ => true, // Other types are considered truthy
    }
}

fn sort_keys_recursive(value: &mut JsonValue) {
    use std::mem;
    match value {
        JsonValue::Object(map) => {
            for v in map.values_mut() {
                sort_keys_recursive(v);
            }

            // Rebuild the map in sorted key order
            let old_map = mem::take(map);

            let mut entries: Vec<_> = old_map.into_iter().collect();
            entries.sort_by(|(a, _), (b, _)| a.cmp(b));

            *map = entries.into_iter().collect();
        }
        JsonValue::Array(arr) => {
            for v in arr {
                sort_keys_recursive(v);
            }
        }
        _ => {} // Null / Bool / Number / String
    }
}

#[cfg(test)]
#[path = "input_handler_test.rs"]
mod test;
