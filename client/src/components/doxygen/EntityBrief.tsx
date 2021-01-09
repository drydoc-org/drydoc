import * as React from 'react';
import styled from 'styled-components';
import Dict from '../../Dict';

import Page from '../../state/Page';
import { ClassItem } from './ClassItem';
import { FunctionItem } from './FunctionItem';
import { ItemProps } from './Item';
import { PageData, Entity as EntityModel } from './model';
import { NamespaceItem } from './NamespaceItem';
import renderer from './renderer';

export interface EntityBriefProps {
  id: string;
  symbols: Dict<EntityModel>;

  onPageChange: (id: string, event: React.MouseEvent<any>) => void;
}

interface EntityBriefState {

}

const Container = styled.div`
  width: 100%;
  font-family: 'Fira Code', monospace;
  font-size: 0.8em;
  padding: 1em;
  background-color: rgba(0, 0, 0, 0.2);
  border-bottom: 1px solid rgba(0, 0, 0, 0.3);
  transition: background-color 0.2s;
  cursor: pointer;
  :nth-child(even) {
    background-color: rgba(0, 0, 0, 0.15);
  }
  :last-child {
    border-bottom: none;
  }
  :hover {
    background-color: rgba(0, 0, 0, 0.1);
  }
`;

type Props = EntityBriefProps;
type State = EntityBriefState;

export class EntityBrief extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }

  private onClick_ = (event: React.MouseEvent<any>) => {
    this.props.onPageChange(this.props.id, event);
  }
  
  render() {
    const { props } = this;

    const { id, symbols, onPageChange } = props;

    const symbol = symbols[id];

    const Renderer = renderer(symbol);
    if (!Renderer) return null;

    return (
      <Container onClick={this.onClick_}>
        <Renderer onPageChange={onPageChange} model={symbol} depth={0} symbols={symbols} />
      </Container>
    );
  }
}