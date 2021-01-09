import Dict from "../../Dict";

export interface BlockCommand {
  type: "blockcommand",
  command: string,
  arguments: string[],
  children: CommentChild[]
}

export interface HtmlStartTag {
  type: "htmlstarttag",
  name: string,
  attributes: [string, string][],
  closing: boolean
}

export interface HtmlEndTag {
  type: "htmlendtag",
  tag: string
}

export type InlineCommandStyle = "bold" | "monospace" | "emphasized";

export interface InlineCommand {
  type: "inlinecommand",
  command: string,
  arguments: string[],
  style?: InlineCommandStyle
}

export interface Paragraph {
  type: "paragraph",
  children: CommentChild[]
}

export type ParameterDirection = "in" | "out" | "inout";

export interface ParamCommand {
  type: "paramcommand",
  index?: number,
  parameter?: string,
  direction?: ParameterDirection,
  children: CommentChild[]
}

export interface TParamCommand {
  type: "tparamcommand",
  position?: [number, number],
  parameter: string,
  children: CommentChild[]
}

export interface Text {
  type: "text",
  text: string
}

export interface VerbatimCommand {
  type: "verbatimcommand",
  parts: string[]
}

export interface VerbatimLineCommand {
  type: "verbatimlinecommand",
  line: string
}

export type CommentChild = BlockCommand | HtmlStartTag | HtmlEndTag | InlineCommand | Paragraph | ParamCommand | TParamCommand | Text | VerbatimCommand | VerbatimLineCommand;

export type Comment = CommentChild[];

export interface Namespace {
  type: "namespace",
  name: string,
  display_name: string,
  comment?: CommentChild[],
  children: string[]
}

export type TypeKind = (
  "unexposed" |
  "void" |
  "bool" |
  "chars" |
  "charu" |
  "schar" |
  "uchar" |
  "wchar" |
  "char16" |
  "char32" |
  "short" |
  "ushort" |
  "int" |
  "uint" |
  "long" |
  "ulong" |
  "longlong" |
  "ulonglong" |
  "int128" |
  "uint128" |
  "half" |
  "float16" |
  "shortaccum" |
  "accum" |
  "longaccum" |
  "ushortaccum" |
  "uaccum" |
  "ulongaccum" |
  "float" |
  "double" |
  "longdouble" |
  "nullptr" |
  "complex" |
  "dependent" |
  "overload" |
  "objcid" |
  "objcclass" |
  "objcsel" |
  "float128" |
  "objcinterface" |
  "objcobjectpointer" |
  "pointer" |
  "blockpointer" |
  "memberpointer" |
  "lvaluereference" |
  "rvaluereference" |
  "enum" |
  "record" |
  "typedef" |
  "functionprototype" |
  "functionnoprototype" |
  "constantarray" |
  "dependentsizedarray" |
  "incompletearray" |
  "variablearray" |
  "vector" |
  "auto" |
  "elaborated" |
  "pipe" |
  "oclimage1dro" |
  "oclimage1darrayro" |
  "oclimage1dbufferro" |
  "oclimage2dro" |
  "oclimage2darrayro" |
  "oclimage2ddepthro" |
  "oclimage2darraydepthro" |
  "oclimage2dmsaaro" |
  "oclimage2darraymsaaro" |
  "oclimage2dmsaadepthro" |
  "oclimage2darraymsaadepthro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage3dro" |
  "oclimage1dwo" |
  "oclimage1darraywo" |
  "oclimage1dbufferwo" |
  "oclimage2dwo" |
  "oclimage2darraywo" |
  "oclimage2ddepthwo" |
  "oclimage2darraydepthwo" |
  "oclimage2dmsaawo" |
  "oclimage2darraymsaawo" |
  "oclimage2dmsaadepthwo" |
  "oclimage2darraymsaadepthwo" |
  "oclimage3dwo" |
  "oclimage1drw" |
  "oclimage1darrayrw" |
  "oclimage1dbufferrw" |
  "oclimage2drw" |
  "oclimage2darrayrw" |
  "oclimage2ddepthrw" |
  "oclimage2darraydepthrw" |
  "oclimage2dmsaarw" |
  "oclimage2darraymsaarw" |
  "oclimage2dmsaadepthrw" |
  "oclimage2darraymsaadepthrw" |
  "oclimage3drw" |
  "oclsampler" |
  "oclevent" |
  "oclqueue" |
  "oclreserveid" |
  "objcobject" |
  "objctypeparam" |
  "attributed" |
  "oclintelsubgroupavcmcepayload" |
  "oclintelsubgroupavcimepayload" |
  "oclintelsubgroupavcrefpayload" |
  "oclintelsubgroupavcsicpayload" |
  "oclintelsubgroupavcmceresult" |
  "oclintelsubgroupavcimeresult" |
  "oclintelsubgroupavcrefresult" |
  "oclintelsubgroupavcsicresult" |
  "oclintelsubgroupavcimeresultsinglerefstreamout" |
  "oclintelsubgroupavcimeresultdualrefstreamout" |
  "oclintelsubgroupavcimesinglerefstreamin" |
  "oclintelsubgroupavcimedualrefstreamin" |
  "extvector"
);

export interface Type {
  kind: TypeKind,
  display_name: string,
  name?: string,
  const_qualified: boolean,
  pointee?: Type,
  elaborated?: Type,
}

export interface Param {
  name: string,
  ty: Type
}

export type Visibility = "default" | "hidden" | "protected";
export type Accessibility = "private" | "protected" | "public";

export interface Function {
  type: "function",
  name: string,
  display_name: string,
  comment?: CommentChild[],
  template_args?: any[],
  params: Param[],
  ret_ty: Type,
  visibility?: Visibility,
  accessibility?: Accessibility,
  is_ctor: boolean,
  is_dtor: boolean
}

export interface Class {
  type: "class",
  is_struct: boolean,
  name: string,
  display_name: string,
  comment?: CommentChild[],
  template_args?: any[]
  children: string[]
}

export interface Variable {
  type: "variable",
  name: string,
  display_name: string,
  comment?: CommentChild[],
  ty: Type
}

export interface Typedef {
  type: "typedef",
  name: string,
  display_name: string,
  comment?: CommentChild[],
  ty: Type
}


export interface Enum {
  type: "enum",
  name: string,
  display_name: string,
  comment?: CommentChild[],
  children: string[]
}

export interface EnumValue {
  type: "enumvalue",
  name: string,
  display_name: string,
  comment?: CommentChild[],
  value?: String
}

export type Entity = Namespace | Function | Class | Variable | Typedef | Enum | EnumValue;

export namespace Entity {
  export const comment = (entity: Entity): CommentChild[] | undefined => entity.comment;
  export const name = (entity: Entity): string => entity.name;
  export const display_name = (entity: Entity): string => entity.display_name;
  
  export const children = (entity: Entity): string[] => {
    switch (entity.type) {
      case 'class':
      case 'namespace':
      case 'enum':
        return entity.children;
      default:
        return [];
    }
  };
}

export interface PageData {
  name: string,
  symbols: Dict<Entity>
}