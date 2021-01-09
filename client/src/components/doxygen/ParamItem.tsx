import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Comment } from './Comment';
import { Entity, Function as FunctionModel, Param, Type, TypeKind } from './model';
import { ItemProps } from './Item';
import { LanguageKeyword, StyleProps, Title } from './style';
import Dict from '../../Dict';
import { TypeItem } from './TypeItem';

export interface ParamItemProps extends StyleProps {
  model: Param;
  symbols: Dict<Entity>;
  comma?: boolean;

  onPageChange: (id: string, event: React.MouseEvent<any>) => void;
}

const Container = styled.span`
  font-family: 'Fira Code', monospace;
`;

type Props = ParamItemProps;

export class ParamItem extends React.PureComponent<Props> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { model, symbols, onPageChange, style, className, comma } = props;
    
    const { name, ty } = model;

    return (
      <Container style={style} className={className}>
        <TypeItem onPageChange={onPageChange} model={ty} symbols={symbols} /> {`${name}${comma ? ', ' : ''}`}
      </Container>
    );
  }
}