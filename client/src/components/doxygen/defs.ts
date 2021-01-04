export interface Scope {
  functions: any[];
  namespaces: NamespaceDef[];
  classes: any[];
}

export interface NamespaceDef {
  name: string;
  scope: Scope;
  brief_description: string;
  detailed_description: string;
}

export interface FunctionParamDef {
  ty: string,
  decl_name: string
}

export interface Function {

}
