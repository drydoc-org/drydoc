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
    
    const { comment, is_struct } = model;

    const keyword = is_struct ? 'struct' : 'class';

    console.log('class item', model);

    return (
      <Container>
        <Title style={{ fontFamily: "'Fira Code', monospace" }} depth={depth}><LanguageKeyword>{keyword}</LanguageKeyword> {model.display_name}</Title>
        {comment ? <Comment comment={comment} /> : undefined}
      </Container>
    );
  }
}