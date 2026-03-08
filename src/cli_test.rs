use super::*;
use clap::Parser;

#[test]
fn test_argument_parse_success() {
    let arg: Argument = "name:string=value".parse().unwrap();

    assert_eq!(arg.name, "name");
    assert_eq!(arg.type_name, "string");
    assert_eq!(arg.value, "value");
}

#[test]
fn test_argument_parse_missing_type_separator() {
    let err = "name=value".parse::<Argument>().unwrap_err();

    assert!(err.contains("Invalid argument format"));
}

#[test]
fn test_argument_parse_missing_value_separator() {
    let err = "name:string".parse::<Argument>().unwrap_err();

    assert!(err.contains("Missing value for argument 'name'"));
}

#[test]
fn test_argument_parse_empty_value() {
    let arg: Argument = "name:string=".parse().unwrap();

    assert_eq!(arg.value, "");
}

#[test]
fn test_input_format_defaults_to_json() {
    let cli = Cli::parse_from(["celq", "this"]);

    assert_eq!(cli.input_format(), InputFormat::Json);
}

#[test]
fn test_input_format_xml() {
    let cli = Cli::parse_from(["celq", "--from-xml", "this"]);

    assert_eq!(cli.input_format(), InputFormat::Xml);
}

#[test]
fn test_parse_parallelism_positive() {
    assert_eq!(parse_parallelism("3").unwrap(), 3);
}

#[test]
fn test_parse_parallelism_zero_rejected() {
    let err = parse_parallelism("0").unwrap_err();

    assert_eq!(err, "parallelism level cannot be 0");
}

#[test]
fn test_parse_parallelism_negative_uses_auto() {
    assert_eq!(parse_parallelism("-4").unwrap(), -1);
}

#[test]
fn test_parse_parallelism_invalid() {
    let err = parse_parallelism("abc").unwrap_err();

    assert_eq!(err, "'abc' is not a valid integer");
}
