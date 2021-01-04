import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Comment } from './Comment';
import { Entity, Function as FunctionModel, Type, TypeKind } from './model';
import { ItemProps } from './Item';
import { LanguageKeyword, Title } from './style';
import Dict from '../../Dict';

export interface TypeItemProps {
  model: Type;
  symbols: Dict<Entity>
}

interface TypeItemState {

}

const Container = styled.span`
  
`;

type Props = TypeItemProps;
type State = TypeItemState;

const KIND_MAPPING: Dict<string> = {
  "lvaluereference": "&",
  "rvaluereference": "&&",
  "pointer": "*"
};

export class TypeItem extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { model, symbols } = props;
    
    const { const_qualified, pointee } = model;

    console.log(symbols);

    let kind = model.kind !== "record" ? (KIND_MAPPING[model.kind] || model.kind) : model.name;
    
    return (
      <Container>
        {const_qualified ? <LanguageKeyword>const</LanguageKeyword> : undefined} {pointee ? <TypeItem model={pointee} symbols={symbols} /> : undefined} <LanguageKeyword>{kind}</LanguageKeyword>
      </Container>
    );
  }
}