import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Comment } from './Comment';
import { Entity, Function as FunctionModel, Param, Type, TypeKind } from './model';
import { ItemProps } from './Item';
import { LanguageKeyword, Title } from './style';
import Dict from '../../Dict';
import { TypeItem } from './TypeItem';

export interface ParamItemProps {
  model: Param;
  symbols: Dict<Entity>;
}

const Container = styled.span`
  
`;

type Props = ParamItemProps;

export class ParamItem extends React.PureComponent<Props> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { model, symbols } = props;
    
    const { name, ty } = model;

    return (
      <Container>
        <TypeItem model={ty} symbols={symbols} /> {name}
      </Container>
    );
  }
}