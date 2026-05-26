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
    let (left, right) = split_assignment(line)?;
    let path = parse_path(left.trim())?;
    let value = parse_value(right.trim())?;
    set_value_at_path(root, &path, value)
}

#[derive(Debug)]
enum PathSegment {
    Property(String),
    Index(usize),
}

fn split_assignment(line: &str) -> Result<(&str, &str)> {
    let mut string_delimiter = None;
    let mut escaped = false;

    for (idx, ch) in line.char_indices() {
        match string_delimiter {
            Some(_) if escaped => escaped = false,
            Some(_) if ch == '\\' => escaped = true,
            Some(delimiter) if ch == delimiter => string_delimiter = None,
            Some(_) => {}
            None if ch == '"' || ch == '\'' => string_delimiter = Some(ch),
            None if ch == '=' => return Ok((&line[..idx], &line[idx + ch.len_utf8()..])),
            None => {}
        }
    }

    Err(anyhow!("Expected assignment (e.g., json.a = 1)"))
}

fn parse_path(left: &str) -> Result<Vec<PathSegment>> {
    let mut parser = PathParser::new(left);
    parser.parse()
}

struct PathParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> PathParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn parse(&mut self) -> Result<Vec<PathSegment>> {
        self.consume_root()?;

        let mut segments = Vec::new();
        while !self.is_done() {
            match self.peek_char() {
                Some('.') => {
                    self.bump_char();
                    segments.push(PathSegment::Property(self.parse_dot_property()?));
                }
                Some('[') => segments.push(self.parse_bracket_segment()?),
                Some(ch) => return Err(anyhow!("Unexpected character in path: '{}'", ch)),
                None => break,
            }
        }

        Ok(segments)
    }

    fn consume_root(&mut self) -> Result<()> {
        let Some(root) = self.read_identifier() else {
            return Err(anyhow!("Path root must be 'json'"));
        };

        if root != "json" {
            return Err(anyhow!("Path root must be 'json', found '{}'", root));
        }

        Ok(())
    }

    fn parse_dot_property(&mut self) -> Result<String> {
        self.read_identifier()
            .ok_or_else(|| anyhow!("Expected property name after '.'"))
    }

    fn parse_bracket_segment(&mut self) -> Result<PathSegment> {
        self.expect_char('[')?;
        let segment = match self.peek_char() {
            Some('"') => PathSegment::Property(self.parse_json_string()?),
            Some('\'') => PathSegment::Property(self.parse_quoted_string('\'')?),
            Some(ch) if ch.is_ascii_digit() => PathSegment::Index(self.parse_index()?),
            Some(ch) => {
                return Err(anyhow!(
                    "Unsupported bracket path segment starting with '{}'",
                    ch
                ));
            }
            None => return Err(anyhow!("Unclosed bracket path segment")),
        };
        self.expect_char(']')?;
        Ok(segment)
    }

    fn parse_json_string(&mut self) -> Result<String> {
        let start = self.pos;
        self.bump_char();

        let mut escaped = false;
        while let Some(ch) = self.peek_char() {
            self.bump_char();
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                return serde_json::from_str(&self.input[start..self.pos])
                    .context("Failed to parse quoted path segment");
            }
        }

        Err(anyhow!("Unclosed string path segment"))
    }

    fn parse_quoted_string(&mut self, delimiter: char) -> Result<String> {
        self.bump_char();

        let mut result = String::new();
        while let Some(ch) = self.peek_char() {
            self.bump_char();
            if ch == delimiter {
                return Ok(result);
            }
            if ch == '\\' {
                result.push(self.parse_escape_sequence()?);
            } else {
                result.push(ch);
            }
        }

        Err(anyhow!("Unclosed string path segment"))
    }

    fn parse_escape_sequence(&mut self) -> Result<char> {
        let Some(ch) = self.peek_char() else {
            return Err(anyhow!("Unclosed escape sequence"));
        };
        self.bump_char();

        match ch {
            '"' => Ok('"'),
            '\'' => Ok('\''),
            '\\' => Ok('\\'),
            '/' => Ok('/'),
            'b' => Ok('\u{0008}'),
            'f' => Ok('\u{000c}'),
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            'u' => self.parse_unicode_escape(),
            other => Ok(other),
        }
    }

    fn parse_unicode_escape(&mut self) -> Result<char> {
        let start = self.pos;
        for _ in 0..4 {
            match self.peek_char() {
                Some(ch) if ch.is_ascii_hexdigit() => self.bump_char(),
                Some(ch) => return Err(anyhow!("Invalid unicode escape character '{}'", ch)),
                None => return Err(anyhow!("Unclosed unicode escape sequence")),
            }
        }

        let code = u32::from_str_radix(&self.input[start..self.pos], 16)
            .context("Failed to parse unicode escape")?;
        char::from_u32(code).ok_or_else(|| anyhow!("Invalid unicode escape code point"))
    }

    fn parse_index(&mut self) -> Result<usize> {
        let start = self.pos;
        while matches!(self.peek_char(), Some(ch) if ch.is_ascii_digit()) {
            self.bump_char();
        }
        self.input[start..self.pos]
            .parse()
            .context("Failed to parse array index")
    }

    fn read_identifier(&mut self) -> Option<String> {
        let start = self.pos;
        let mut chars = self.input[self.pos..].char_indices();
        let (_, first) = chars.next()?;

        if !(first == '_' || first == '$' || first.is_ascii_alphabetic()) {
            return None;
        }

        self.pos += first.len_utf8();
        while let Some(ch) = self.peek_char() {
            if ch == '_' || ch == '$' || ch.is_ascii_alphanumeric() {
                self.bump_char();
            } else {
                break;
            }
        }

        Some(self.input[start..self.pos].to_string())
    }

    fn expect_char(&mut self, expected: char) -> Result<()> {
        match self.peek_char() {
            Some(ch) if ch == expected => {
                self.bump_char();
                Ok(())
            }
            Some(ch) => Err(anyhow!("Expected '{}', found '{}'", expected, ch)),
            None => Err(anyhow!("Expected '{}', found end of path", expected)),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn bump_char(&mut self) {
        if let Some(ch) = self.peek_char() {
            self.pos += ch.len_utf8();
        }
    }

    fn is_done(&self) -> bool {
        self.pos == self.input.len()
    }
}

fn parse_value(right: &str) -> Result<JsonValue> {
    let Some(value_src) = strip_trailing_semicolon(right) else {
        return Err(anyhow!("Expected assignment to end with ';'"));
    };

    let value_src = value_src.trim();
    if value_src.starts_with('\'') {
        return parse_single_quoted_value(value_src);
    }

    serde_json::from_str(value_src).context("Failed to parse gron value as JSON")
}

fn strip_trailing_semicolon(right: &str) -> Option<&str> {
    let mut string_delimiter = None;
    let mut escaped = false;
    let mut semicolon = None;

    for (idx, ch) in right.char_indices() {
        match string_delimiter {
            Some(_) if escaped => escaped = false,
            Some(_) if ch == '\\' => escaped = true,
            Some(delimiter) if ch == delimiter => string_delimiter = None,
            Some(_) => {}
            None if ch == '"' || ch == '\'' => string_delimiter = Some(ch),
            None if ch == ';' => semicolon = Some(idx),
            None if !ch.is_whitespace() && semicolon.is_some() => return None,
            None => {}
        }
    }

    semicolon.map(|idx| &right[..idx])
}

fn parse_single_quoted_value(value_src: &str) -> Result<JsonValue> {
    let mut parser = PathParser::new(value_src);
    let value = parser.parse_quoted_string('\'')?;
    if !parser.input[parser.pos..].trim().is_empty() {
        return Err(anyhow!("Unexpected characters after string value"));
    }
    Ok(JsonValue::String(value))
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
