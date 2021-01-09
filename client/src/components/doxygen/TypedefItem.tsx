import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Comment } from './Comment';
import { Typedef as TypedefModel } from './model';
import { ItemProps } from './Item';
import { LanguageKeyword, Title } from './style';
import { TypeItem } from './TypeItem';
import { ParamItem } from './ParamItem';

export interface TypedefItemProps extends ItemProps<TypedefModel> {
}

const Container = styled.div`
  width: 100%;
`;

type Props = TypedefItemProps;

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

export class TypedefItem extends React.PureComponent<Props> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { model, depth, symbols, onPageChange } = props;
    
    const { comment, display_name } = model;

    return (
      <Container>
        <Title style={{ fontFamily: "'Fira Code', monospace" }} depth={depth}><LanguageKeyword>typedef</LanguageKeyword> <TypeItem onPageChange={onPageChange} symbols={symbols} model={model.ty} /> {display_name}</Title>
        {comment ? <Comment comment={comment} /> : undefined}
      </Container>
    );
  }
}