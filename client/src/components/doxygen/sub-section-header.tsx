import { Entity } from "./model";

export default (parent: Entity, child: Entity): string => {
  switch (parent.type) {
    case "class": {
      switch (child.type) {
        case "function": return "Methods";
        case "variable": return "Fields";
        case "class": return child.is_struct ? "Substructures" : "Subclasses";
      }
    }
  }

  switch (child.type) {
    case "function": return "Functions";
    case "variable": return "Variables";
    case "namespace": return "Namespaces";
    case "typedef": return "Type Definitions";
    case "class": return child.is_struct ? "Structures" : "Classes";
  }

  return "Other";
};