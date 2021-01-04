import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Comment } from './Comment';
import { Function as FunctionModel } from './model';
import { ItemProps } from './Item';
import { LanguageKeyword, Title } from './style';
import { TypeItem } from './TypeItem';
import { ParamItem } from './ParamItem';
import { Accessibility } from './Accessibility';

export interface FunctionItemProps extends ItemProps<FunctionModel> {
}

interface FunctionItemState {

}

const Container = styled.div`
  width: 100%;
`;

type Props = FunctionItemProps;
type State = FunctionItemState;

const LANGUAGE_KEYWORDS = new Set<string>([
  'void',
  'int',
  'char',
  'long',
  'bool',
  'short',
  'unsigned',
  'signed',
  'const'
]);

export class FunctionItem extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { model, depth, symbols } = props;
    
    const { comment, display_name } = model;

    const params: JSX.Element[] = model.params.map((param, i) => (
      <>
        <ParamItem key={i} model={param} symbols={symbols} />
        {i !== model.params.length - 1 ? ', ' : ''}
      </>
    ));

    return (
      <Container>
        <Title style={{ fontFamily: "'Fira Code', monospace" }} depth={depth}><Accessibility model={model.accessibility}/> <TypeItem symbols={symbols} model={model.ret_ty} /> {display_name}({params})</Title>
        {comment ? <Comment comment={comment} /> : undefined}
      </Container>
    );
  }
}