import Dict from "../../Dict";
import { Entity } from "./model";

export interface ItemProps<T> {
  model: T;
  depth: number;
  symbols: Dict<Entity>;
}