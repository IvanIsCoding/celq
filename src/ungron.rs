// Feature inspired by the gron CLI
// Source: https://github.com/tomnomnom/gron
// gron is licensed under the MIT License
// Copyright (c) 2016 Tom Hudson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use anyhow::{Context, Result, anyhow};
use resast::prelude::*;
use ressa::Parser;
use serde_json::{Value as JsonValue, json};

pub fn gron_to_json(input: &str) -> Result<JsonValue> {
    let mut root = JsonValue::Null;

    for (line_num, line) in input
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty())
    {
        let line = line.trim();
        parse_and_apply_line(line, &mut root)
            .with_context(|| format!("Error on line {}: {}", line_num + 1, line))?;
    }
    Ok(root)
}

fn parse_and_apply_line(line: &str, root: &mut JsonValue) -> Result<()> {
    let mut parser = Parser::new(line).map_err(|e| anyhow!("Parser init failed: {:?}", e))?;

    let part = parser
        .next()
        .ok_or_else(|| anyhow!("Empty line"))?
        .map_err(|e| anyhow!("Parse error: {:?}", e))?;

    if let ProgramPart::Stmt(Stmt::Expr(Expr::Assign(assign))) = part {
        let path = extract_path(&assign.left)?;
        let value = extract_value(&assign.right)?;
        set_value_at_path(root, &path, value)?;
        Ok(())
    } else {
        Err(anyhow!("Expected assignment (e.g., json.a = 1)"))
    }
}

#[derive(Debug)]
enum PathSegment {
    Property(String),
    Index(usize),
}

fn extract_path(left: &AssignLeft) -> Result<Vec<PathSegment>> {
    let mut segments = Vec::new();
    let expr = match left {
        AssignLeft::Expr(e) => e,
        _ => return Err(anyhow!("Unsupported assignment target")),
    };

    let mut current_expr: &Expr = expr;

    // Walk up the MemberExpressions (e.g., json.user["id"])
    while let Expr::Member(mem) = current_expr {
        let seg = match &*mem.property {
            Expr::Ident(i) => PathSegment::Property(i.name.to_string()),
            Expr::Lit(Lit::String(StringLit::Double(s)))
            | Expr::Lit(Lit::String(StringLit::Single(s))) => PathSegment::Property(s.to_string()),
            Expr::Lit(Lit::Number(n)) => PathSegment::Index(n.parse()?),
            _ => return Err(anyhow!("Unsupported path segment type")),
        };
        segments.push(seg);
        current_expr = &*mem.object;
    }

    // Root must be 'json'
    if let Expr::Ident(ident) = current_expr
        && ident.name != "json"
    {
        return Err(anyhow!("Path root must be 'json', found '{}'", ident.name));
    }

    segments.reverse();
    Ok(segments)
}

fn extract_value(expr: &Expr) -> Result<JsonValue> {
    match expr {
        Expr::Lit(lit) => match lit {
            Lit::Null => Ok(JsonValue::Null),
            Lit::Boolean(b) => Ok(JsonValue::Bool(*b)),
            Lit::Number(n) => {
                // Try integer first, then float, to ensure test equality passes
                if let Ok(i) = n.parse::<i64>() {
                    Ok(json!(i))
                } else {
                    Ok(json!(n.parse::<f64>()?))
                }
            }
            Lit::String(StringLit::Double(s)) | Lit::String(StringLit::Single(s)) => {
                Ok(json!(s.to_string()))
            }
            _ => Err(anyhow!("Unsupported literal")),
        },
        Expr::Unary(u) if u.operator == UnaryOp::Minus => {
            let val = extract_value(&u.argument)?;
            if let Some(i) = val.as_i64() {
                Ok(json!(-i))
            } else if let Some(f) = val.as_f64() {
                Ok(json!(-f))
            } else {
                Err(anyhow!("Cannot negate non-numeric value"))
            }
        }
        Expr::Array(_) => Ok(json!([])),
        Expr::Obj(_) => Ok(json!({})),
        _ => Err(anyhow!("Value type not supported")),
    }
}

fn set_value_at_path(root: &mut JsonValue, path: &[PathSegment], value: JsonValue) -> Result<()> {
    let mut cur = root;

    for seg in path {
        match seg {
            PathSegment::Property(p) => {
                if !cur.is_object() {
                    *cur = json!({});
                }
                cur = cur
                    .as_object_mut()
                    .unwrap()
                    .entry(p.clone())
                    .or_insert(JsonValue::Null);
            }
            PathSegment::Index(i) => {
                if !cur.is_array() {
                    *cur = json!([]);
                }
                let arr = cur.as_array_mut().unwrap();
                if *i >= arr.len() {
                    arr.resize(*i + 1, JsonValue::Null);
                }
                cur = &mut arr[*i];
            }
        }
    }

    // Don't overwrite an existing object/array with an empty one.
    if (value.is_object() && value.as_object().unwrap().is_empty() && cur.is_object())
        || (value.is_array() && value.as_array().unwrap().is_empty() && cur.is_array())
    {
        return Ok(());
    }

    *cur = value;
    Ok(())
}

#[cfg(test)]
#[path = "ungron_test.rs"]
mod test;
