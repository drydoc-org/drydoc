import { Type } from "./model";

export default (ty: Type) => {
  if (ty.const_qualified) {
    return ty.display_name.slice("const ".length);
  }
  return ty.display_name;
};