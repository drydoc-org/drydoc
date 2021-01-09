import { ClassItem } from "./ClassItem";
import { EnumItem } from "./EnumItem";
import { EnumValueItem } from "./EnumValueItem";
import { FunctionItem } from "./FunctionItem";
import { ItemProps } from "./Item";
import { Entity } from "./model";
import { NamespaceItem } from "./NamespaceItem";
import { TypedefItem } from "./TypedefItem";
import { VariableItem } from "./VariableItem";


export default (symbol: Entity): React.ComponentType<ItemProps<any>> | null => {
  switch (symbol.type) {
    case "namespace": return NamespaceItem;
    case "variable": return VariableItem;
    case "class": return ClassItem;
    case "function": return FunctionItem;
    case "typedef": return TypedefItem;
    case "enum": return EnumItem;
    case "enumvalue": return EnumValueItem;
    default: return null;
  }
};