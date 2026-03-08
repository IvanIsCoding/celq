use super::*;
use tempfile::NamedTempFile;

#[test]
fn test_compile_expression_inline() {
    let result = compile_expression(Some("1 + 2"), None);

    assert!(result.is_ok());
}

#[test]
fn test_compile_expression_from_file() {
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), "1 + 2").unwrap();

    let result = compile_expression(None, Some(&file.path().to_path_buf()));

    assert!(result.is_ok());
}

#[test]
fn test_compile_expression_missing_expression() {
    let err = compile_expression(None, None).unwrap_err().to_string();

    assert!(err.contains("missing CEL expression"));
}

#[test]
fn test_compile_expression_missing_file() {
    let path = PathBuf::from("/definitely/missing/celq-expression.txt");
    let err = compile_expression(None, Some(&path))
        .unwrap_err()
        .to_string();

    assert!(err.contains("failed to read expression file"));
}

#[test]
fn test_compile_expression_invalid_source() {
    let err = compile_expression(Some("("), None).unwrap_err().to_string();

    assert!(!err.is_empty());
}
