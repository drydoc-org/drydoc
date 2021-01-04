import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { ClassItem } from './ClassItem';
import { FunctionItem } from './FunctionItem';
import { ItemProps } from './Item';
import { PageData } from './model';
import { NamespaceItem } from './NamespaceItem';

export interface EntityProps {
  page: Page.Resolved;
}

interface EntityState {

}

const Container = styled.div`
  width: 100%;
  height: 100%;
  font-family: 'Fira Code', monospace;
`;

type Props = EntityProps;
type State = EntityState;

export class Entity extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { page } = props;
    const { content } = page;

    const { name, symbols } = JSON.parse(content) as PageData;

    const symbol = symbols[name];

    let Renderer: any | null = null;
    switch (symbol.type) {
      case "namespace": {
        Renderer = NamespaceItem;
        break;
      }
      case "class": {
        Renderer = ClassItem;
        break;
      }
      case "function": {
        Renderer = FunctionItem;
        break;
      }
      default: {
        return null;
      }
    }

    return (
      <Container>
        <Renderer model={symbol} depth={0} symbols={symbols} />
      </Container>
    );
  }
}