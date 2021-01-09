import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Comment } from './Comment';
import { EnumValue as EnumValueModel } from './model';
import { ItemProps } from './Item';
import { LanguageKeyword, Title } from './style';
import { TypeItem } from './TypeItem';
import { ParamItem } from './ParamItem';
import { Accessibility } from './Accessibility';

export interface EnumValueItemProps extends ItemProps<EnumValueModel> {
}

const Container = styled.div`
  width: 100%;
`;

type Props = EnumValueItemProps;

export class EnumValueItem extends React.PureComponent<Props> {
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
        <Title style={{ fontFamily: "'Fira Code', monospace" }} depth={depth}><LanguageKeyword>enum</LanguageKeyword> {display_name}</Title>
        {comment ? <Comment comment={comment} /> : undefined}
      </Container>
    );
  }
}