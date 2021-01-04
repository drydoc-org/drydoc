use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::page::{Page, Id};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockCommand {
  pub command: String,
  pub arguments: Vec<String>,
  pub children: Vec<CommentChild>
}

impl From<clang::documentation::BlockCommand> for BlockCommand {
  fn from(value: clang::documentation::BlockCommand) -> Self {
    Self {
      command: value.command,
      arguments: value.arguments,
      children: value.children.into_iter().map(|c| c.into()).collect()
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HtmlStartTag {
  pub name: String,
  pub attributes: Vec<(String, String)>,
  pub closing: bool
}

impl From<clang::documentation::HtmlStartTag> for HtmlStartTag {
  fn from(value: clang::documentation::HtmlStartTag) -> Self {
    Self {
      name: value.name,
      attributes: value.attributes,
      closing: value.closing
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum InlineCommandStyle {
  Bold,
  Monospace,
  Emphasized,
}

impl From<clang::documentation::InlineCommandStyle> for InlineCommandStyle {
  fn from(value: clang::documentation::InlineCommandStyle) -> Self {
    match value {
      clang::documentation::InlineCommandStyle::Bold => Self::Bold,
      clang::documentation::InlineCommandStyle::Monospace => Self::Monospace,
      clang::documentation::InlineCommandStyle::Emphasized => Self::Emphasized,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineCommand {
  pub command: String,
  pub arguments: Vec<String>,
  pub style: Option<InlineCommandStyle>,
}

impl From<clang::documentation::InlineCommand> for InlineCommand {
  fn from(value: clang::documentation::InlineCommand) -> Self {
    Self {
      command: value.command,
      arguments: value.arguments,
      style: value.style.map(|s| s.into())
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParameterDirection {
  In,
  Out,
  InOut,
}

impl From<clang::documentation::ParameterDirection> for ParameterDirection {
  fn from(value: clang::documentation::ParameterDirection) -> Self {
    match value {
      clang::documentation::ParameterDirection::In => Self::In,
      clang::documentation::ParameterDirection::Out => Self::Out,
      clang::documentation::ParameterDirection::InOut => Self::InOut,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamCommand {
  pub index: Option<usize>,
  pub parameter: String,
  pub direction: Option<ParameterDirection>,
  pub children: Vec<CommentChild>,
}

impl From<clang::documentation::ParamCommand> for ParamCommand {
  fn from(value: clang::documentation::ParamCommand) -> Self {
    Self {
      index: value.index,
      parameter: value.parameter,
      direction: value.direction.map(|c| c.into()),
      children: value.children.into_iter().map(|c| c.into()).collect()
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TParamCommand {
  pub position: Option<(usize, usize)>,
  pub parameter: String,
  pub children: Vec<CommentChild>,
}

impl From<clang::documentation::TParamCommand> for TParamCommand {
  fn from(value: clang::documentation::TParamCommand) -> Self {
    Self {
      position: value.position,
      parameter: value.parameter,
      children: value.children.into_iter().map(|c| c.into()).collect()
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CommentChild {
  BlockCommand(BlockCommand),
  HtmlStartTag(HtmlStartTag),
  HtmlEndTag { tag: String },
  InlineCommand(InlineCommand),
  Paragraph { children: Vec<CommentChild> },
  ParamCommand(ParamCommand),
  TParamCommand(TParamCommand),
  Text { text: String },
  VerbatimCommand { parts: Vec<String> },
  VerbatimLineCommand { line: String }
}

impl From<clang::documentation::CommentChild> for CommentChild {
  fn from(value: clang::documentation::CommentChild) -> Self {
    match value {
      clang::documentation::CommentChild::BlockCommand(cmd) => Self::BlockCommand(cmd.into()),
      clang::documentation::CommentChild::HtmlStartTag(tag) => Self::HtmlStartTag(tag.into()),
      clang::documentation::CommentChild::HtmlEndTag(tag) => Self::HtmlEndTag { tag: tag.into() },
      clang::documentation::CommentChild::InlineCommand(cmd) => Self::InlineCommand(cmd.into()),
      clang::documentation::CommentChild::Paragraph(para) => Self::Paragraph { children: para.into_iter().map(|c| c.into()).collect() },
      clang::documentation::CommentChild::ParamCommand(cmd) => Self::ParamCommand(cmd.into()),
      clang::documentation::CommentChild::TParamCommand(cmd) => Self::TParamCommand(cmd.into()),
      clang::documentation::CommentChild::Text(text) => Self::Text { text: text.into() },
      clang::documentation::CommentChild::VerbatimCommand(cmd) => Self::VerbatimCommand { parts: cmd.into() },
      clang::documentation::CommentChild::VerbatimLineCommand(cmd) => Self::VerbatimLineCommand { line: cmd.into() },
    }
  }
}

pub struct Mangler<'tu> {
  path: Vec<clang::Entity<'tu>>
}

impl<'tu> Mangler<'tu> {
  pub fn new() -> Self {
    Self {
      path: Vec::new()
    }
  }

  pub fn push(&mut self, entity: clang::Entity<'tu>) {
    self.path.push(entity)
  }

  pub fn pop(&mut self) -> Option<clang::Entity<'tu>> {
    self.path.pop()
  }

  fn is_fs_unsafe(c: char) -> bool {
    c == '>' || c == '<' || c == '*' || c == '\\' || c == '/'
  }

  fn to_fs_safe(string: String) -> String {
    let mut ret = String::with_capacity(string.len());
    for c in string.chars() {
      if Self::is_fs_unsafe(c) {
        ret.push('U');
        ret.push_str((c as u32).to_string().as_str());
      } else {
        ret.push(c);
      }
    }
    ret
  }

  fn name_from_parts(parts: &Vec<clang::Entity<'tu>>) -> String {
    let mut ret = String::new();
    for entity in parts.iter() {

        ret.push_str(match entity.get_kind() {
          clang::EntityKind::Namespace => "n-",
          clang::EntityKind::FunctionDecl => "f-",
          clang::EntityKind::ClassDecl => "c-",
          clang::EntityKind::ClassTemplate => "tc-",
          clang::EntityKind::Method => "m-",
          clang::EntityKind::StructDecl => "s-",
          _ => ""
        });

      ret.push_str(Self::to_fs_safe(entity.get_name().unwrap_or("".to_string())).as_str());
      ret.push('_');
    }
    ret.pop();

    ret
  }

  pub fn name(&self) -> String {
    Self::name_from_parts(&self.path)
  }

  pub fn lookup_name(entity: clang::Entity<'tu>) -> String {
    let mut parts = Vec::new();
    parts.push(entity);
    let mut current = entity;
    while let Some(parent) = current.get_semantic_parent() {
      if parent.get_kind() == clang::EntityKind::TranslationUnit {
        break;
      }

      parts.push(parent);
      current = parent;
    }
    parts.reverse();
    
    Self::name_from_parts(&parts)
  }
}

pub trait EntityLike {
  fn visit<'tu>(entity: clang::Entity<'tu>, mangler: &mut Mangler<'tu>, symbols: &mut HashMap<String, Entity>) -> HashSet<String>;
  fn to_page<P: AsRef<str>>(&self, prefix: P, symbols: &HashMap<String, Entity>) -> Page;
  fn children(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>>;
  fn linked(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>>;
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace {
  name: String,
  display_name: String,
  comment: Option<Vec<CommentChild>>,
  children: HashSet<String>
}

use std::iter::FromIterator;

impl EntityLike for Namespace {
  fn linked(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    let mut ret = HashSet::new();

    for child in self.children.iter() {
      if let Some(symbol) = symbols.get(child) {
        if let Some(linked) = symbol.linked(symbols) {
          ret.extend(linked);
        }
      }
    }

    if ret.len() > 0 { Some(ret) } else { None }
  }

  fn children(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    let mut ret = HashSet::from_iter(self.children.iter().map(|c| c.clone()));
    for child in self.children.iter() {
      match symbols.get(child) {
        Some(entity) => match entity.children(symbols) {
          Some(children) => ret.extend(children),
          _ => {}
        },
        _ => {}
      }
    }

    Some(ret)
  }

  fn visit<'tu>(entity: clang::Entity<'tu>, mangler: &mut Mangler<'tu>, symbols: &mut HashMap<String, Entity>) -> HashSet<String> {
    assert_eq!(entity.get_kind(), clang::EntityKind::Namespace);
    let display_name = entity.get_display_name().unwrap();

    let mut ret = HashSet::new();

    if display_name == "std" {
      return ret;
    }

    mangler.push(entity);
    let name = mangler.name();

    let mut children = HashSet::new();
    for entity in entity.get_children() {
      children.extend(Entity::visit(entity, mangler, symbols));
    }

    ret.insert(name.clone());

    if let Some(Entity::Namespace(ns)) = symbols.get_mut(&name) {
      ns.children.extend(children);
      return ret;
    }
    
    let namespace = Namespace {
      name: mangler.name(),
      display_name,
      comment: entity.get_parsed_comment().map(|c| c.get_children().into_iter().map(|c| c.into()).collect()),
      children
    };

    symbols.insert(name.clone(), Entity::Namespace(namespace));

    mangler.pop();

    ret
  }

  fn to_page<P: AsRef<str>>(&self, prefix: P, _: &HashMap<String, Entity>) -> Page {
    println!("PAGE namespace children {:?}", &self.children);
    Page::builder()
      .id(format!("{}{}", prefix.as_ref(), self.name))
      .name(self.display_name.clone())
      .renderer("clang")
      .content_type("clang/entity")
      .meta("section", "namespace")
      .children(self.children.iter())
      .build()
      .unwrap()
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TypeKind {
  Unexposed,
  Void,
  Bool,
  CharS,
  CharU,
  SChar,
  UChar,
  WChar,
  Char16,
  Char32,
  Short,
  UShort,
  Int,
  UInt,
  Long,
  ULong,
  LongLong,
  ULongLong,
  Int128,
  UInt128,
  Half,
  Float16,
  ShortAccum,
  Accum,
  LongAccum,
  UShortAccum,
  UAccum,
  ULongAccum,
  Float,
  Double,
  LongDouble,
  Nullptr,
  Complex,
  Dependent,
  Overload,
  ObjCId,
  ObjCClass,
  ObjCSel,
  Float128,
  ObjCInterface,
  ObjCObjectPointer,
  Pointer,
  BlockPointer,
  MemberPointer,
  LValueReference,
  RValueReference,
  Enum,
  Record,
  Typedef,
  FunctionPrototype,
  FunctionNoPrototype,
  ConstantArray,
  DependentSizedArray,
  IncompleteArray,
  VariableArray,
  Vector,
  Auto,
  Elaborated,
  Pipe,
  OCLImage1dRO,
  OCLImage1dArrayRO,
  OCLImage1dBufferRO,
  OCLImage2dRO,
  OCLImage2dArrayRO,
  OCLImage2dDepthRO,
  OCLImage2dArrayDepthRO,
  OCLImage2dMSAARO,
  OCLImage2dArrayMSAARO,
  OCLImage2dMSAADepthRO,
  OCLImage2dArrayMSAADepthRO,
  OCLImage3dRO,
  OCLImage1dWO,
  OCLImage1dArrayWO,
  OCLImage1dBufferWO,
  OCLImage2dWO,
  OCLImage2dArrayWO,
  OCLImage2dDepthWO,
  OCLImage2dArrayDepthWO,
  OCLImage2dMSAAWO,
  OCLImage2dArrayMSAAWO,
  OCLImage2dMSAADepthWO,
  OCLImage2dArrayMSAADepthWO,
  OCLImage3dWO,
  OCLImage1dRW,
  OCLImage1dArrayRW,
  OCLImage1dBufferRW,
  OCLImage2dRW,
  OCLImage2dArrayRW,
  OCLImage2dDepthRW,
  OCLImage2dArrayDepthRW,
  OCLImage2dMSAARW,
  OCLImage2dArrayMSAARW,
  OCLImage2dMSAADepthRW,
  OCLImage2dArrayMSAADepthRW,
  OCLImage3dRW,
  OCLSampler,
  OCLEvent,
  OCLQueue,
  OCLReserveID,
  ObjCObject,
  ObjCTypeParam,
  Attributed,
  OCLIntelSubgroupAVCMcePayload,
  OCLIntelSubgroupAVCImePayload,
  OCLIntelSubgroupAVCRefPayload,
  OCLIntelSubgroupAVCSicPayload,
  OCLIntelSubgroupAVCMceResult,
  OCLIntelSubgroupAVCImeResult,
  OCLIntelSubgroupAVCRefResult,
  OCLIntelSubgroupAVCSicResult,
  OCLIntelSubgroupAVCImeResultSingleRefStreamout,
  OCLIntelSubgroupAVCImeResultDualRefStreamout,
  OCLIntelSubgroupAVCImeSingleRefStreamin,
  OCLIntelSubgroupAVCImeDualRefStreamin,
  ExtVector,
}

impl From<clang::TypeKind> for TypeKind {
  fn from(value: clang::TypeKind) -> Self {
    match value {
      clang::TypeKind::Unexposed => Self::Unexposed,
      clang::TypeKind::Void => Self::Void,
      clang::TypeKind::Bool => Self::Bool,
      clang::TypeKind::CharS => Self::CharS,
      clang::TypeKind::CharU => Self::CharU,
      clang::TypeKind::SChar => Self::SChar,
      clang::TypeKind::UChar => Self::UChar,
      clang::TypeKind::WChar => Self::WChar,
      clang::TypeKind::Char16 => Self::Char16,
      clang::TypeKind::Char32 => Self::Char32,
      clang::TypeKind::Short => Self::Short,
      clang::TypeKind::UShort => Self::UShort,
      clang::TypeKind::Int => Self::Int,
      clang::TypeKind::UInt => Self::UInt,
      clang::TypeKind::Long => Self::Long,
      clang::TypeKind::ULong => Self::ULong,
      clang::TypeKind::LongLong => Self::LongLong,
      clang::TypeKind::ULongLong => Self::ULongLong,
      clang::TypeKind::Int128 => Self::Int128,
      clang::TypeKind::UInt128 => Self::UInt128,
      clang::TypeKind::Half => Self::Half,
      clang::TypeKind::Float16 => Self::Float16,
      clang::TypeKind::ShortAccum => Self::ShortAccum,
      clang::TypeKind::Accum => Self::Accum,
      clang::TypeKind::LongAccum => Self::LongAccum,
      clang::TypeKind::UShortAccum => Self::UShortAccum,
      clang::TypeKind::UAccum => Self::UAccum,
      clang::TypeKind::ULongAccum => Self::ULongAccum,
      clang::TypeKind::Float => Self::Float,
      clang::TypeKind::Double => Self::Double,
      clang::TypeKind::LongDouble => Self::LongDouble,
      clang::TypeKind::Nullptr => Self::Nullptr,
      clang::TypeKind::Complex => Self::Complex,
      clang::TypeKind::Dependent => Self::Dependent,
      clang::TypeKind::Overload => Self::Overload,
      clang::TypeKind::ObjCId => Self::ObjCId,
      clang::TypeKind::ObjCClass => Self::ObjCClass,
      clang::TypeKind::ObjCSel => Self::ObjCSel,
      clang::TypeKind::Float128 => Self::Float128,
      clang::TypeKind::ObjCInterface => Self::ObjCInterface,
      clang::TypeKind::ObjCObjectPointer => Self::ObjCObjectPointer,
      clang::TypeKind::Pointer => Self::Pointer,
      clang::TypeKind::BlockPointer => Self::BlockPointer,
      clang::TypeKind::MemberPointer => Self::MemberPointer,
      clang::TypeKind::LValueReference => Self::LValueReference,
      clang::TypeKind::RValueReference => Self::RValueReference,
      clang::TypeKind::Enum => Self::Enum,
      clang::TypeKind::Record => Self::Record,
      clang::TypeKind::Typedef => Self::Typedef,
      clang::TypeKind::FunctionPrototype => Self::FunctionPrototype,
      clang::TypeKind::FunctionNoPrototype => Self::FunctionNoPrototype,
      clang::TypeKind::ConstantArray => Self::ConstantArray,
      clang::TypeKind::DependentSizedArray => Self::DependentSizedArray,
      clang::TypeKind::IncompleteArray => Self::IncompleteArray,
      clang::TypeKind::VariableArray => Self::VariableArray,
      clang::TypeKind::Vector => Self::Vector,
      clang::TypeKind::Auto => Self::Auto,
      clang::TypeKind::Elaborated => Self::Elaborated,
      clang::TypeKind::Pipe => Self::Pipe,
      clang::TypeKind::OCLImage1dRO => Self::OCLImage1dRO,
      clang::TypeKind::OCLImage1dArrayRO => Self::OCLImage1dArrayRO,
      clang::TypeKind::OCLImage1dBufferRO => Self::OCLImage1dBufferRO,
      clang::TypeKind::OCLImage2dRO => Self::OCLImage2dRO,
      clang::TypeKind::OCLImage2dArrayRO => Self::OCLImage2dArrayRO,
      clang::TypeKind::OCLImage2dDepthRO => Self::OCLImage2dDepthRO,
      clang::TypeKind::OCLImage2dArrayDepthRO => Self::OCLImage2dArrayDepthRO,
      clang::TypeKind::OCLImage2dMSAARO => Self::OCLImage2dMSAARO,
      clang::TypeKind::OCLImage2dArrayMSAARO => Self::OCLImage2dArrayMSAARO,
      clang::TypeKind::OCLImage2dMSAADepthRO => Self::OCLImage2dMSAADepthRO,
      clang::TypeKind::OCLImage2dArrayMSAADepthRO => Self::OCLImage2dArrayMSAADepthRO,
      clang::TypeKind::OCLImage3dRO => Self::OCLImage3dRO,
      clang::TypeKind::OCLImage1dWO => Self::OCLImage1dWO,
      clang::TypeKind::OCLImage1dArrayWO => Self::OCLImage1dArrayWO,
      clang::TypeKind::OCLImage1dBufferWO => Self::OCLImage1dBufferWO,
      clang::TypeKind::OCLImage2dWO => Self::OCLImage2dWO,
      clang::TypeKind::OCLImage2dArrayWO => Self::OCLImage2dArrayWO,
      clang::TypeKind::OCLImage2dDepthWO => Self::OCLImage2dDepthWO,
      clang::TypeKind::OCLImage2dArrayDepthWO => Self::OCLImage2dArrayDepthWO,
      clang::TypeKind::OCLImage2dMSAAWO => Self::OCLImage2dMSAAWO,
      clang::TypeKind::OCLImage2dArrayMSAAWO => Self::OCLImage2dArrayMSAAWO,
      clang::TypeKind::OCLImage2dMSAADepthWO => Self::OCLImage2dMSAADepthWO,
      clang::TypeKind::OCLImage2dArrayMSAADepthWO => Self::OCLImage2dArrayMSAADepthWO,
      clang::TypeKind::OCLImage3dWO => Self::OCLImage3dWO,
      clang::TypeKind::OCLImage1dRW => Self::OCLImage1dRW,
      clang::TypeKind::OCLImage1dArrayRW => Self::OCLImage1dArrayRW,
      clang::TypeKind::OCLImage1dBufferRW => Self::OCLImage1dBufferRW,
      clang::TypeKind::OCLImage2dRW => Self::OCLImage2dRW,
      clang::TypeKind::OCLImage2dArrayRW => Self::OCLImage2dArrayRW,
      clang::TypeKind::OCLImage2dDepthRW => Self::OCLImage2dDepthRW,
      clang::TypeKind::OCLImage2dArrayDepthRW => Self::OCLImage2dArrayDepthRW,
      clang::TypeKind::OCLImage2dMSAARW => Self::OCLImage2dMSAARW,
      clang::TypeKind::OCLImage2dArrayMSAARW => Self::OCLImage2dArrayMSAARW,
      clang::TypeKind::OCLImage2dMSAADepthRW => Self::OCLImage2dMSAADepthRW,
      clang::TypeKind::OCLImage2dArrayMSAADepthRW => Self::OCLImage2dArrayMSAADepthRW,
      clang::TypeKind::OCLImage3dRW => Self::OCLImage3dRW,
      clang::TypeKind::OCLSampler => Self::OCLSampler,
      clang::TypeKind::OCLEvent => Self::OCLEvent,
      clang::TypeKind::OCLQueue => Self::OCLQueue,
      clang::TypeKind::OCLReserveID => Self::OCLReserveID,
      clang::TypeKind::ObjCObject => Self::ObjCObject,
      clang::TypeKind::ObjCTypeParam => Self::ObjCTypeParam,
      clang::TypeKind::Attributed => Self::Attributed,
      clang::TypeKind::OCLIntelSubgroupAVCMcePayload => Self::OCLIntelSubgroupAVCMcePayload,
      clang::TypeKind::OCLIntelSubgroupAVCImePayload => Self::OCLIntelSubgroupAVCImePayload,
      clang::TypeKind::OCLIntelSubgroupAVCRefPayload => Self::OCLIntelSubgroupAVCRefPayload,
      clang::TypeKind::OCLIntelSubgroupAVCSicPayload => Self::OCLIntelSubgroupAVCSicPayload,
      clang::TypeKind::OCLIntelSubgroupAVCMceResult => Self::OCLIntelSubgroupAVCMceResult,
      clang::TypeKind::OCLIntelSubgroupAVCImeResult => Self::OCLIntelSubgroupAVCImeResult,
      clang::TypeKind::OCLIntelSubgroupAVCRefResult => Self::OCLIntelSubgroupAVCRefResult,
      clang::TypeKind::OCLIntelSubgroupAVCSicResult => Self::OCLIntelSubgroupAVCSicResult,
      clang::TypeKind::OCLIntelSubgroupAVCImeResultSingleRefStreamout => Self::OCLIntelSubgroupAVCImeResultSingleRefStreamout,
      clang::TypeKind::OCLIntelSubgroupAVCImeResultDualRefStreamout => Self::OCLIntelSubgroupAVCImeResultDualRefStreamout,
      clang::TypeKind::OCLIntelSubgroupAVCImeSingleRefStreamin => Self::OCLIntelSubgroupAVCImeSingleRefStreamin,
      clang::TypeKind::OCLIntelSubgroupAVCImeDualRefStreamin => Self::OCLIntelSubgroupAVCImeDualRefStreamin,
      clang::TypeKind::ExtVector => Self::ExtVector,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
  kind: TypeKind,
  display_name: String,
  name: Option<String>,
  const_qualified: bool,
  pointee: Option<Box<Type>>
}

impl Type {
  fn linked(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    let mut ret = HashSet::new();
    if let Some(name) = &self.name {
      ret.insert(name.clone());
    }

    if let Some(pointee) = &self.pointee {
      if let Some(names) = pointee.linked(symbols) {
        ret.extend(names);
      }
    }

    if ret.len() > 0 { Some(ret) } else { None }
  }
}

impl<'tu> From<clang::Type<'tu>> for Type {
  fn from(value: clang::Type<'tu>) -> Self {
    let name = if value.get_kind() == clang::TypeKind::Record {
      Some(Mangler::lookup_name(value.get_declaration().unwrap()))
    } else {
      None
    };

    Self {
      display_name: value.get_display_name(),
      name,
      kind: value.get_kind().into(),
      const_qualified: value.is_const_qualified(),
      pointee: value.get_pointee_type().map(|t| Box::new(t.into()))
    }
  }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
  name: String,
  ty: Type
}

impl Param {
  pub fn new<'tu>(entity: clang::Entity<'tu>) -> Self {
    assert_eq!(entity.get_kind(), clang::EntityKind::ParmDecl);

    Self {
      name: entity.get_name().unwrap_or("".to_string()),
      ty: entity.get_type().unwrap().into()
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TemplateArg {
  Declaration,
  Expression,
  Null,
  Nullptr,
  Pack,
  Template,
  TemplateExpansion,
  Integral(i64, i64),
  Type(Type)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
  Default,
  Hidden,
  Protected
}

impl From<clang::Visibility> for Visibility {
  fn from(value: clang::Visibility) -> Self {
    match value {
      clang::Visibility::Default => Self::Default,
      clang::Visibility::Hidden => Self::Hidden,
      clang::Visibility::Protected => Self::Protected,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Accessibility {
  Private,
  Protected,
  Public
}

impl From<clang::Accessibility> for Accessibility {
  fn from(value: clang::Accessibility) -> Self {
    match value {
      clang::Accessibility::Private => Self::Private,
      clang::Accessibility::Protected => Self::Protected,
      clang::Accessibility::Public => Self::Public,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
  name: String,
  display_name: String,
  comment: Option<Vec<CommentChild>>,
  template_args: Option<Vec<TemplateArg>>,
  params: Vec<Param>,
  ret_ty: Type,
  visibility: Option<Visibility>,
  accessibility: Option<Accessibility>
}

impl EntityLike for Function {
  fn linked(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    let mut ret = HashSet::new();
    for param in self.params.iter() {
      if let Some(names) = param.ty.linked(symbols) {
        ret.extend(names);
      }
    }
    
    if ret.len() > 0 { Some(ret) } else { None }
  }
  
  fn children(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    None
  }

  fn to_page<P: AsRef<str>>(&self, prefix: P, symbols: &HashMap<String, Entity>) -> Page {
    Page::builder()
      .id(format!("{}{}", prefix.as_ref(), self.name))
      .name(self.display_name.clone())
      .renderer("clang")
      .content_type("clang/entity")
      .meta("section", "function")
      .build()
      .unwrap()
  }

  fn visit<'tu>(entity: clang::Entity<'tu>, mangler: &mut Mangler<'tu>, symbols: &mut HashMap<String, Entity>) -> HashSet<String> {
    mangler.push(entity);
    let name = mangler.name();

    
    let function = Function {
      name: name.clone(),
      ret_ty: entity.get_result_type().unwrap().into(),
      display_name: entity.get_name().unwrap(),
      comment: entity.get_parsed_comment().map(|c| c.get_children().into_iter().map(|c| c.into()).collect()),
      template_args: None,
      visibility: entity.get_visibility().map(|v| v.into()),
      accessibility: entity.get_accessibility().map(|v| v.into()),
      params: entity.get_arguments().unwrap().into_iter().map(|c| Param::new(c)).collect()
    };

    mangler.pop();


    symbols.insert(name.clone(), Entity::Function(function));

    let mut ret = HashSet::new();
    ret.insert(name);
    ret
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
  is_struct: bool,
  name: String,
  display_name: String,
  comment: Option<Vec<CommentChild>>,
  template_args: Option<Vec<TemplateArg>>, 
  children: HashSet<String>
}

impl EntityLike for Class {
  fn linked(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    let mut ret = HashSet::new();

    for child in self.children.iter() {
      if let Some(symbol) = symbols.get(child) {
        if let Some(linked) = symbol.linked(symbols) {
          ret.extend(linked);
        }
      }
    }

    if ret.len() > 0 { Some(ret) } else { None }
  }
  
  fn children(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    Some(self.children.clone())
  }

  fn to_page<P: AsRef<str>>(&self, prefix: P, symbols: &HashMap<String, Entity>) -> Page {
    Page::builder()
      .id(format!("{}{}", prefix.as_ref(), self.name))
      .name(self.display_name.clone())
      .renderer("clang")
      .content_type("clang/entity")
      .meta("section", if self.is_struct { "struct" } else { "class" })
      .children(self.children.iter())
      .build()
      .unwrap()
  }

  fn visit<'tu>(entity: clang::Entity<'tu>, mangler: &mut Mangler<'tu>, symbols: &mut HashMap<String, Entity>) -> HashSet<String> {
    mangler.push(entity);
    let name = mangler.name();
    

    
    let mut children = HashSet::new();
    for child in entity.get_children() {
      children.extend(Entity::visit(child, mangler, symbols))
    }

    let class = Class {
      is_struct: entity.get_kind() == clang::EntityKind::StructDecl,
      name: name.clone(),
      display_name: entity.get_display_name().unwrap(),
      comment: entity.get_parsed_comment().map(|c| c.get_children().into_iter().map(|c| c.into()).collect()),
      template_args: None,
      children
    };

    mangler.pop();

    // println!("{:?} {:?}", &name, );

    symbols.insert(name.clone(), Entity::Class(class));

    let mut ret = HashSet::new();
    ret.insert(name);
    ret
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variable {
  name: String,
  display_name: String,
  ty: Type,
  comment: Option<Vec<CommentChild>>,
  visibility: Option<Visibility>,
  accessibility: Option<Accessibility>
}

impl EntityLike for Variable {
  fn linked(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    self.ty.linked(symbols)
  }
  
  fn children(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    None
  }

  fn to_page<P: AsRef<str>>(&self, prefix: P, symbols: &HashMap<String, Entity>) -> Page {
    Page::builder()
      .id(format!("{}{}", prefix.as_ref(), self.name))
      .name(self.display_name.clone())
      .renderer("clang")
      .content_type("clang/entity")
      .meta("section", "variable")
      .build()
      .unwrap()
  }

  fn visit<'tu>(entity: clang::Entity<'tu>, mangler: &mut Mangler<'tu>, symbols: &mut HashMap<String, Entity>) -> HashSet<String> {
    mangler.push(entity);
    let name = mangler.name();

    let variable = Variable {
      name: name.clone(),
      display_name: entity.get_display_name().unwrap(),
      ty: entity.get_type().unwrap().into(),
      accessibility: entity.get_accessibility().map(|v| v.into()),
      visibility: entity.get_visibility().map(|v| v.into()),
      comment: entity.get_parsed_comment().map(|c| c.get_children().into_iter().map(|c| c.into()).collect()),
    };

    mangler.pop();

    // println!("{:?} {:?}", &name, );

    symbols.insert(name.clone(), Entity::Variable(variable));

    let mut ret = HashSet::new();
    ret.insert(name);
    ret
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Entity {
  Namespace(Namespace),
  Function(Function),
  Class(Class),
  Variable(Variable)
}

impl EntityLike for Entity {
  fn linked(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    match self {
      Self::Namespace(namespace) => namespace.linked(symbols),
      Self::Function(function) => function.linked(symbols),
      Self::Class(class) => class.linked(symbols),
      Self::Variable(variable) => variable.linked(symbols),
    }
  }
  
  fn children(&self, symbols: &HashMap<String, Entity>) -> Option<HashSet<String>> {
    match self {
      Self::Namespace(namespace) => namespace.children(symbols),
      Self::Function(function) => function.children(symbols),
      Self::Class(class) => class.children(symbols),
      Self::Variable(variable) => variable.children(symbols),
    }
  }

  fn to_page<P: AsRef<str>>(&self, prefix: P, symbols: &HashMap<String, Entity>) -> Page {
    match self {
      Self::Namespace(namespace) => namespace.to_page(prefix, symbols),
      Self::Function(function) => function.to_page(prefix, symbols),
      Self::Class(class) => class.to_page(prefix, symbols),
      Self::Variable(variable) => variable.to_page(prefix, symbols),
    }
  }

  fn visit<'tu>(entity: clang::Entity<'tu>, mangler: &mut Mangler<'tu>, symbols: &mut HashMap<String, Entity>) -> HashSet<String> {
    if entity.is_in_system_header() {
      return HashSet::new();
    }

    match entity.get_kind() {
      clang::EntityKind::Namespace => Namespace::visit(entity, mangler, symbols),
      clang::EntityKind::FunctionDecl | clang::EntityKind::Method => Function::visit(entity, mangler, symbols),
      clang::EntityKind::ClassDecl | clang::EntityKind::ClassTemplate | clang::EntityKind::StructDecl => Class::visit(entity, mangler, symbols),
      clang::EntityKind::FieldDecl | clang::EntityKind::VarDecl => Variable::visit(entity, mangler, symbols),
      clang::EntityKind::TranslationUnit => {
        let mut ret = HashSet::new();
        for entity in entity.get_children() {
          ret.extend(Self::visit(entity, mangler, symbols).into_iter());
        }
        ret
      },
      _ => {
        println!("Unhandled {:?}", entity);
        HashSet::new()
      }
    }
  }
}

pub fn subset<'a>(all: &'a HashMap<String, Entity>, sub: HashSet<String>) -> HashMap<&'a String, &'a Entity> {
  let mut ret = HashMap::new();
  for (name, entity) in all.iter() {
    if !sub.contains(name) {
      continue;
    }
    ret.insert(name, entity);
  }
  ret
}