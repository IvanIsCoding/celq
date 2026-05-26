use quick_xml::Reader;
use quick_xml::XmlVersion;
use quick_xml::encoding::EncodingError;
use quick_xml::escape::{EscapeError, resolve_predefined_entity};
use quick_xml::events::attributes::AttrError;
use quick_xml::events::{BytesRef, BytesStart, BytesText, Event};
use serde_json::{Map, Value};
use std::error::Error;
use std::fmt;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct XmlParseError(String);

impl fmt::Display for XmlParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for XmlParseError {}

impl From<quick_xml::Error> for XmlParseError {
    fn from(err: quick_xml::Error) -> Self {
        Self(err.to_string())
    }
}

impl From<AttrError> for XmlParseError {
    fn from(err: AttrError) -> Self {
        Self(err.to_string())
    }
}

impl From<EncodingError> for XmlParseError {
    fn from(err: EncodingError) -> Self {
        Self(err.to_string())
    }
}

impl From<EscapeError> for XmlParseError {
    fn from(err: EscapeError) -> Self {
        Self(err.to_string())
    }
}

impl From<Utf8Error> for XmlParseError {
    fn from(err: Utf8Error) -> Self {
        Self(err.to_string())
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    value: Map<String, Value>,
    text: String,
}

impl Node {
    fn new(name: String, value: Map<String, Value>) -> Self {
        Self {
            name,
            value,
            text: String::new(),
        }
    }
}

pub fn parse_xml(xml: &str) -> Result<Value, XmlParseError> {
    let mut reader = Reader::from_str(xml);
    let mut stack = Vec::new();
    let mut output = Value::Null;
    let mut root_seen = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) => {
                if root_seen && stack.is_empty() {
                    return Err(XmlParseError("multiple root elements".to_string()));
                }
                stack.push(node_from_start(&reader, &event)?);
            }
            Ok(Event::Empty(event)) => {
                if root_seen && stack.is_empty() {
                    return Err(XmlParseError("multiple root elements".to_string()));
                }
                let node = node_from_start(&reader, &event)?;
                if let Some(root) = close_node(node, &mut stack)? {
                    root_seen = true;
                    output = root;
                }
            }
            Ok(Event::Text(event)) => append_text(&mut stack, event)?,
            Ok(Event::GeneralRef(event)) => append_ref(&mut stack, event)?,
            Ok(Event::CData(event)) => {
                if let Some(node) = stack.last_mut() {
                    node.text.push_str(&event.decode()?);
                } else {
                    return Err(XmlParseError("CDATA outside root element".to_string()));
                }
            }
            Ok(Event::End(event)) => {
                let node = stack.pop().ok_or_else(|| {
                    XmlParseError("unexpected closing tag without an open element".to_string())
                })?;
                let close_name = std::str::from_utf8(event.name().as_ref())?.to_string();
                if node.name != close_name {
                    return Err(XmlParseError(format!(
                        "mismatched closing tag: expected {}, got {}",
                        node.name, close_name
                    )));
                }
                if let Some(root) = close_node(node, &mut stack)? {
                    root_seen = true;
                    output = root;
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(XmlParseError(format!(
                    "error at position {}: {}",
                    reader.buffer_position(),
                    err
                )));
            }
        }
    }

    if !stack.is_empty() {
        return Err(XmlParseError("unexpected end of input".to_string()));
    }

    Ok(output)
}

fn node_from_start(reader: &Reader<&[u8]>, event: &BytesStart<'_>) -> Result<Node, XmlParseError> {
    let name = std::str::from_utf8(event.name().as_ref())?.to_string();
    let mut value = Map::new();
    let mut attrs = Map::new();

    for attr in event.attributes() {
        let attr = attr?;
        let key = std::str::from_utf8(attr.key.as_ref())?.to_string();
        let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, reader.decoder())?;
        attrs.insert(key, Value::String(value.into_owned()));
    }

    if !attrs.is_empty() {
        value.insert("$".to_string(), Value::Object(attrs));
    }

    Ok(Node::new(name, value))
}

fn append_text(stack: &mut [Node], event: BytesText<'_>) -> Result<(), XmlParseError> {
    if let Some(node) = stack.last_mut() {
        node.text.push_str(&event.decode()?);
    } else if !event.decode()?.chars().all(char::is_whitespace) {
        return Err(XmlParseError("text outside root element".to_string()));
    }
    Ok(())
}

fn append_ref(stack: &mut [Node], event: BytesRef<'_>) -> Result<(), XmlParseError> {
    if let Some(node) = stack.last_mut() {
        if let Some(ch) = event.resolve_char_ref()? {
            node.text.push(ch);
        } else {
            let entity = event.decode()?;
            let resolved = resolve_predefined_entity(&entity).ok_or_else(|| {
                XmlParseError(format!("unknown XML entity reference: &{entity};"))
            })?;
            node.text.push_str(resolved);
        }
    } else {
        return Err(XmlParseError(
            "entity reference outside root element".to_string(),
        ));
    }
    Ok(())
}

fn close_node(mut node: Node, stack: &mut [Node]) -> Result<Option<Value>, XmlParseError> {
    let text = if node.text.chars().all(char::is_whitespace) {
        String::new()
    } else {
        node.text
    };

    let value = if node.value.is_empty() {
        Value::String(text)
    } else {
        if !text.is_empty() {
            node.value.insert("_".to_string(), Value::String(text));
        }
        Value::Object(node.value)
    };

    if let Some(parent) = stack.last_mut() {
        assign_or_push(&mut parent.value, node.name, value);
        Ok(None)
    } else {
        let mut root = Map::new();
        root.insert(node.name, value);
        Ok(Some(Value::Object(root)))
    }
}

fn assign_or_push(object: &mut Map<String, Value>, key: String, value: Value) {
    match object.get_mut(&key) {
        None => {
            object.insert(key, value);
        }
        Some(existing @ Value::String(_))
        | Some(existing @ Value::Object(_))
        | Some(existing @ Value::Bool(_))
        | Some(existing @ Value::Number(_))
        | Some(existing @ Value::Null) => {
            let previous = std::mem::replace(existing, Value::Null);
            *existing = Value::Array(vec![previous, value]);
        }
        Some(Value::Array(values)) => values.push(value),
    }
}

#[cfg(test)]
#[path = "xml2json_test.rs"]
mod tests;
