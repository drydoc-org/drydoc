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

    const { model, depth, symbols, onPageChange } = props;
    
    const { comment, display_name, is_ctor, is_dtor } = model;

    const paramLength = model.params.map(p => p.name.length + p.ty.display_name.length).reduce((a, b) => a + b, 0);
    const multiline = paramLength > 100;

    const paramStyle: React.CSSProperties = multiline ? {
      display: 'block',
      marginLeft: '10px',
    } : {};

    const params: JSX.Element[] = model.params.map((param, i) => (
      <>
        <ParamItem style={paramStyle} onPageChange={onPageChange} key={i} model={param} symbols={symbols} comma={i !== model.params.length - 1} />
      </>
    ));

    const retTyComponent = (!is_ctor && !is_dtor) ? <TypeItem onPageChange={onPageChange} symbols={symbols} model={model.ret_ty} /> : undefined;

    return (
      <Container>
        <Title style={{ fontFamily: "'Fira Code', monospace" }} depth={depth}><Accessibility model={model.accessibility}/> {retTyComponent} {display_name}({params})</Title>
        {comment ? <Comment comment={comment} /> : undefined}
      </Container>
    );
  }
}