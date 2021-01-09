import Dict from "../../Dict";
import { Entity } from "./model";
import { StyleProps } from "./style";

export interface ItemProps<T> extends StyleProps {
  model: T;
  depth: number;
  symbols: Dict<Entity>;

  onPageChange: (id: string, event: React.MouseEvent<any>) => void;
}