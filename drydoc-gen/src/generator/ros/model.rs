use serde::{Serialize, Deserialize};
use derive_more::{Display, Error};

use regex::Regex;
use url::form_urlencoded::Parse;

mod parser {
  include!(concat!(env!("OUT_DIR"), "/generator/ros/parser.rs"));
}

#[derive(Debug, Display, Error)]
pub enum ParseError {
  UnexpectedToken {
    message: String
  },
  Unimplemented
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
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
      _ => Err(ParseError::UnexpectedToken {
        message: format!("Unknown primitive {}", string.as_ref())
      })
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
  package: String,
  name: String,
  page_id: Option<String>
}

impl Reference {
  pub fn resolve(self, package_name: &String) -> Self {
    Self {
      package: if self.package == "$THIS_PACKAGE" { package_name.clone() } else { self.package },
      ..self
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FieldKind {
  Primitive { kind: Primitive },
  Reference(Reference)
}

impl From<Primitive> for FieldKind {
  fn from(value: Primitive) -> Self {
    Self::Primitive { kind: value }
  }
}

impl From<Reference> for FieldKind {
  fn from(value: Reference) -> Self {
    Self::Reference(value)
  }
}

impl FieldKind {
  fn parse<T: AsRef<str>>(text: T) -> Result<Self, ParseError> {
    parser::FieldKindParser::new().parse(text.as_ref()).map_err(|e| {
      ParseError::UnexpectedToken {
        message: e.to_string()
      }
    })
  }

  pub fn resolve(self, package_name: &String) -> Self {
    match self {
      Self::Reference(r) => Self::Reference(r.resolve(package_name)),
      _ => self
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ArrayKind {
  None,
  Fixed { size: usize },
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
    let text = text.as_ref();

    parser::FieldParser::new().parse(text.as_ref()).map_err(|e| {
      ParseError::UnexpectedToken {
        message: e.to_string()
      }
    })
  }

  pub fn resolve(self, package_name: &String) -> Self {
    Self {
      kind: self.kind.resolve(package_name),
      ..self
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constant {
  field: Field,
  value: String
}

impl Constant {
  pub fn parse<T: AsRef<str>>(text: T) -> Result<Self, ParseError> {
    let text = text.as_ref();
    

    let parts = text.split('=').collect::<Vec<&str>>();

    let field = parser::FieldParser::new().parse(parts[0]).map_err(|e| {
      ParseError::UnexpectedToken {
        message: e.to_string()
      }
    })?;
    
    Ok(Self {
      field,
      value: parts[1].trim().to_string()
    })
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
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
      Self::Constant(c) => &mut c.field.comment,
      Self::Field(f) => &mut f.comment,
    }
  }

  pub fn resolve(self, package_name: &String) -> Self {
    match self {
      Self::Constant(c) => Self::Constant(Constant {
        field: c.field.resolve(package_name),
        ..c
      }),
      Self::Field(f) => Self::Field(f.resolve(package_name))
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
  package: String,
  name: String,
  statements: Vec<Statement>,
  comment: Option<String>
}

impl Message {
  pub fn parse<T: AsRef<str>>(package: String, name: String, text: T) -> Result<Self, ParseError> {
    let text = text.as_ref();
    lazy_static! {
      static ref REGEX: Regex = Regex::new(r"(?m)^.*$").unwrap();
    }

    
    let mut first_break = true;
    let mut comment_lines = Vec::new();
    let mut statements = Vec::new();
    let mut message_comment = None;
    
    for line in REGEX.find_iter(text) {
      let line_str = line.as_str().trim();
      if line_str.is_empty() {
        if first_break {
          message_comment = Some(comment_lines.join("\n"));
        }
  
        comment_lines.clear();
        continue;
      }

      if line_str.starts_with('#') {
        let comment = line_str[1..].trim();

        // Some ROS messages have productions like:
        // # This is the message comment
        // #
        // # This is the field comment
        // int32 value
        if comment.is_empty() {
          if first_break {
            message_comment = Some(comment_lines.join("\n"));
          }
    
          comment_lines.clear();
          continue;
        }

        comment_lines.push(comment);
      } else {
        first_break = false;
        let mut statement = Statement::parse(line_str)?;
        if !comment_lines.is_empty() {
          *statement.comment_mut() = Some(comment_lines.join("\n"));
        }
        statements.push(statement);
      }
    }

    Ok(Self {
      package,
      name,
      statements,
      comment: message_comment
    })
  }

  pub fn resolve(self, package_name: &String) -> Self {
    Self {
      statements: self.statements.into_iter().map(|s| s.resolve(package_name)).collect(),
      ..self
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
  package: String,
  name: String,
  request: Message,
  response: Message,
}

impl Service {
  pub fn parse<T: AsRef<str>>(package: String, name: String, text: T) -> Result<Self, ParseError> {
    let text = text.as_ref();

    let parts = text.split("---").collect::<Vec<&str>>();

    if parts.len() != 2 {
      return Err(ParseError::UnexpectedToken {
        message: format!("Expected service to have two submessages, but got {}", parts.len())
      });
    }

    Ok(Self {
      package: package.clone(),
      request: Message::parse(package.clone(), format!("{}Req", name), parts[0])?,
      response: Message::parse(package, format!("{}Res", name), parts[1])?,
      name,
    })
  }

  pub fn resolve(self, package_name: &String) -> Self {
    Self {
      request: self.request.resolve(package_name),
      response: self.response.resolve(package_name),
      ..self
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
  request: Message,
  progress: Message,
  response: Message,
}

impl Action {
  pub fn parse<T: AsRef<str>>(package: String, name: String, text: T) -> Result<Self, ParseError> {
    let text = text.as_ref();

    let parts = text.split("---").collect::<Vec<&str>>();

    if parts.len() != 3 {
      return Err(ParseError::UnexpectedToken {
        message: format!("Expected service to have three submessages, but got {}", parts.len())
      });
    }

    Ok(Self {
      request: Message::parse(package.clone(), format!("{}Goal", name), parts[0])?,
      progress: Message::parse(package.clone(), format!("{}Feedback", name),parts[2])?,
      response: Message::parse(package, format!("{}Result", name), parts[1])?
    })
  }
}