import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Class } from './model';
import { ItemProps } from './Item';
import { LanguageKeyword, SubTitle, Title } from './style';
import { Comment } from './Comment';
import { FunctionItem } from './FunctionItem';
import { VariableItem } from './VariableItem';

export interface ClassItemProps extends ItemProps<Class> {
}

interface ClassItemState {

}

const Container = styled.div`
  width: 100%;
`;

type Props = ClassItemProps;
type State = ClassItemState;

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



export class ClassItem extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { model, symbols, depth } = props;
    
    const { comment, children, is_struct } = model;

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
          break;
        }
      }
    }

    return (
      <Container>
        <Title style={{ fontFamily: "'Fira Code', monospace" }} depth={depth}><LanguageKeyword>{is_struct ? 'struct' : 'class'} </LanguageKeyword> {model.display_name}</Title>
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
            <SubTitle depth={depth}>Methods</SubTitle>
            {functions}
          </>
        ) : undefined}

        {variables.length > 0 ? (
          <>
            <SubTitle depth={depth}>Fields</SubTitle>
            {variables}
          </>
        ) : undefined}
      </Container>
    );
  }
}