use serde::{Serialize, Deserialize};

use derive_more::Display;

use std::collections::HashMap;

#[derive(Display, Debug)]
pub enum ParseError {
  #[display(fmt = "{}: {}", line, message)]
  SyntaxError {
    line: usize,
    message: String
  },
}

#[derive(Serialize, Deserialize, Hash)]
pub enum Type {
  Builtin(String),
  Reference(String, String)
}

impl Type {
  pub fn parse(string: &str) -> Option<Self> {
    let parts: Vec<&str> = string.split('/').collect();

    if parts.len() == 1 {
      Some(Self::Builtin(parts[0].to_string()))
    } else if parts.len() == 2 {
      Some(Self::Reference(parts[0].to_string(), parts[1].to_string()))
    } else {
      None
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Constant {
  name: String,
  ty: Type,
  value: String,
  comment: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Field {
  name: String,
  ty: Type,
  comment: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct RosMsg {
  constants: Vec<Constant>,
  fields: Vec<Field>,
}

impl RosMsg {
  pub fn parse(contents: &str) -> Result<Self, ParseError> {
    let lines: Vec<&str> = contents.split(|c| c == '\n').collect();

    let mut ret = Self {
      fields: Vec::new(),
      constants: Vec::new()
    };

    let mut i = 1;
    for line in lines {
      let parts: Vec<&str> = line.split_ascii_whitespace().collect();

      if parts.len() == 4 {
        if parts[2] != "=" {
          return Err(ParseError::SyntaxError { line: i, message: format!("Couldn't parse \"{}\"", line) });
        }
        if let Some(ty) = Type::parse(parts[0]) {
          ret.constants.push(Constant {
            name: parts[1].to_string(),
            ty,
            value: parts[3].to_string(),
            comment: None
          });
        } else {
          return Err(ParseError::SyntaxError {
            line: i,
            message: format!("Couldn't parse type \"{}\"", parts[0])
          });
        }
        
      } else if parts.len() == 3 {
        if let Some(ty) = Type::parse(parts[0]) {
          ret.fields.push(Field {
            name: parts[1].to_string(),
            ty,
            comment: None
          });
        } else {
          return Err(ParseError::SyntaxError {
            line: i,
            message: format!("Couldn't parse type \"{}\"", parts[0])
          });
        }
      } else {
        return Err(ParseError::SyntaxError {
          line: i,
          message: format!("Couldn't parse \"{}\"", line)
        });
      }

      i += 1;
    }

    Ok(ret)
  }
}

#[derive(Serialize, Deserialize)]
pub struct RosSrv {
  request: RosMsg,
  response: RosMsg
}

impl RosSrv {
  pub fn parse(string: &str) -> Result<Self, ParseError> {
    let parts: Vec<&str> = string.split("---").collect();

    if parts.len() != 2 {
      Err(ParseError::SyntaxError { line: 0, message: "Expected one ---".to_string() })
    } else {
      let request = RosMsg::parse(parts[0])?;
      let response = RosMsg::parse(parts[1])?;

      Ok(Self {
        request,
        response
      })
    }
  }
}