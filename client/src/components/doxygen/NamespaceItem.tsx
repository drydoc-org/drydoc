import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';


import { Namespace } from './model';
// import { FunctionItem } from './';
import { ItemProps } from './Item';
import { LanguageKeyword, SubTitle, Title } from './style';
import { Comment } from './Comment';
import { FunctionItem } from './FunctionItem';
import { ClassItem } from './ClassItem';
import { VariableItem } from './VariableItem';

export interface NamespaceItemProps extends ItemProps<Namespace> {
}

interface NamespaceItemState {

}

const Container = styled.div`
  display: flex;
  flex-direction: column;
  width: 100%;
`;

type Props = NamespaceItemProps;
type State = NamespaceItemState;

export class NamespaceItem extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { model, depth, symbols } = props;
    const { comment, children } = model;

    let functions: JSX.Element[] = [];
    let classes: JSX.Element[] = [];
    let structs: JSX.Element[] = [];
    let variables: JSX.Element[] = [];

    console.log('model', model);
    
    for (let i = 0; i < children.length; ++i) {
      const child = children[i];
      const symbol = symbols[child];
      console.log('symbol', symbol);

      if (!symbol) continue;


      switch (symbol.type) {
        case "function": {
          functions.push(
            <FunctionItem model={symbol} symbols={symbols} depth={depth + 1} />
          );
          break;
        }
        case "class": {
          if (symbol.is_struct) {
            structs.push(
              <ClassItem model={symbol} symbols={symbols} depth={depth + 1} />
            );
          } else {
            classes.push(
              <ClassItem model={symbol} symbols={symbols} depth={depth + 1} />
            );
          }
          break;
        }
        case "variable": {
          variables.push(
            <VariableItem model={symbol} symbols={symbols} depth={depth + 1} />
          );
        }
      }
    }
    
    return (
      <Container>
        <Title depth={depth} style={{ fontFamily: `"Fira Code", monospace` }}><LanguageKeyword>namespace</LanguageKeyword> {model.display_name}</Title>
        {comment ? <Comment comment={comment} /> : undefined}

        {classes.length > 0 ? (
          <>
            <SubTitle depth={depth}>Classes</SubTitle>
            {classes}
          </>
        ) : undefined}

        {structs.length > 0 ? (
          <>
            <SubTitle depth={depth}>Structs</SubTitle>
            {structs}
          </>
        ) : undefined}

        {functions.length > 0 ? (
          <>
            <SubTitle depth={depth}>Functions</SubTitle>
            {functions}
          </>
        ) : undefined}

        {variables.length > 0 ? (
          <>
            <SubTitle depth={depth}>Variables</SubTitle>
            {variables}
          </>
        ) : undefined}

      </Container>
    );
  }
}