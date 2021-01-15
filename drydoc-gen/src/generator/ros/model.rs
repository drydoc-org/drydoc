use serde::{Serialize, Deserialize};
use derive_more::{Display, Error};

use regex::Regex;
use url::form_urlencoded::Parse;

#[derive(Debug, Display, Error)]
pub enum ParseError {
  UnexpectedToken,
  Unimplemented
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Primitive {
  Bool,
  Int8,
  UInt8,
  Int16,
  UInt16,
  Int32,
  UInt32,
  Int64,
  UInt64,
  Float32,
  Float64,
  String,
  Time,
  Duration
}

impl Primitive {
  pub fn parse<S: AsRef<str>>(string: S) -> Result<Self, ParseError> {
    match string.as_ref() {
      "bool" => Ok(Self::Bool),
      "int8" => Ok(Self::Int8),
      "uint8" => Ok(Self::UInt8),
      "int16" => Ok(Self::Int16),
      "uint16" => Ok(Self::UInt16),
      "int32" => Ok(Self::Int32),
      "uint32" => Ok(Self::UInt32),
      "int64" => Ok(Self::Int64),
      "uint64" => Ok(Self::UInt64),
      "float32" => Ok(Self::Float32),
      "float64" => Ok(Self::Float64),
      "string" => Ok(Self::String),
      "time" => Ok(Self::Time),
      "duration" => Ok(Self::Duration),
      _ => Err(ParseError::UnexpectedToken)
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
  package: String,
  name: String,
  page_id: Option<String>
}
#[derive(Serialize, Deserialize, Debug)]
pub enum FieldKind {
  Primitive(Primitive),
  Reference(Reference)
}

impl From<Primitive> for FieldKind {
  fn from(value: Primitive) -> Self {
    Self::Primitive(value)
  }
}

impl From<Reference> for FieldKind {
  fn from(value: Reference) -> Self {
    Self::Reference(value)
  }
}

impl FieldKind {
  fn parse<T: AsRef<str>>(text: T) -> Result<Self, ParseError> {
    let parts: Vec<&str> = text.as_ref().split('/').collect();
    match parts.len() {
      1 => Ok(Self::Primitive(Primitive::parse(parts[0])?)),
      2 => Ok(Self::Reference(Reference {
        package: parts[0].to_string(),
        name: parts[1].to_string(),
        page_id: None
      })),
      _ => Err(ParseError::UnexpectedToken)
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArrayKind {
  None,
  Fixed(usize),
  Variable
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
  kind: FieldKind,
  array_kind: ArrayKind,
  name: String,
  comment: Option<String>
}

impl Field {
  pub fn parse<T: AsRef<str>>(text: T) -> Result<Self, ParseError> {
    Err(ParseError::Unimplemented)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constant {
  field: Field,
  value: String,
  comment: Option<String>
}

impl Constant {
  pub fn parse<T: AsRef<str>>(text: T) -> Result<Self, ParseError> {
    Err(ParseError::Unimplemented)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Statement {
  Field(Field),
  Constant(Constant)
}

impl Statement {
  fn parse<T: AsRef<str>>(text: T) -> Result<Self, ParseError> {
    let text = text.as_ref();
    lazy_static! {
      static ref REGEX: Regex = Regex::new(r"(?m)^.*$").unwrap();
    }

    if text.contains('=') {
      Ok(Self::Constant(Constant::parse(text)?))
    } else {
      Ok(Self::Field(Field::parse(text)?))
    }
  }

  pub fn comment_mut(&mut self) -> &mut Option<String> {
    match self {
      Self::Constant(c) => &mut c.comment,
      Self::Field(f) => &mut f.comment,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
  statements: Vec<Statement>
}

impl Message {
  pub fn parse<T: AsRef<str>>(text: T) -> Result<Self, ParseError> {
    let text = text.as_ref();
    lazy_static! {
      static ref REGEX: Regex = Regex::new(r"(?m)^.*$").unwrap();
    }

    let mut comment_lines = Vec::new();
    let mut statements = Vec::new();
    for line in REGEX.find_iter(text) {
      if text.trim().starts_with('#') {
        comment_lines.push(text);
      } else {
        let mut statement = Statement::parse(line.as_str())?;
        if !comment_lines.is_empty() {
          *statement.comment_mut() = Some(comment_lines.join("\n"));
        }
        statements.push(statement);
      }
    }

    Ok(Self {
      statements
    })
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
  request: Message,
  response: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
  request: Message,
  progress: Message,
  response: Message,
}