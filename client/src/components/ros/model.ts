export interface Comment {
  
}

export type PrimitiveKind =
  "bool" |
  "int8" |
  "uint8" |
  "int16" |
  "uint16" |
  "int32" |
  "uint32" |
  "int64" |
  "uint64" |
  "float32" |
  "float64" |
  "string" |
  "time" |
  "duration";

export interface Reference {
  type: "reference";
  package: string;
  name: string;
  page_id: string;
}

export interface Primitive {
  type: "primitive";
  kind: PrimitiveKind;
}

export type FieldKind = Reference | Primitive;

export interface ArrayKindNone {
  type: "none";
}

export interface ArrayKindFixed {
  type: "fixed";
  size: number;
}

export interface ArrayKindVariable {
  type: "variable";
}

export type ArrayKind = ArrayKindNone | ArrayKindFixed | ArrayKindVariable;

export interface Field {
  type: "field";
  kind: FieldKind;
  array_kind: ArrayKind;
  name: string;
  comment?: string;
}

export interface Constant {
  type: "constant";
  field: Field;
  value: string;
}

export type Statement = Field | Constant;

export interface Message {
  package: string;
  name: string;
  statements: Statement[];
  comment?: string;
}

export interface Service {
  package: string;
  name: string;
  request: Message;
  response: Message;
}

export interface Action {
  request: Message;
  progress: Message;
  response: Message;
}