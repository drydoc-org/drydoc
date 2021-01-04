import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { CommentChild, Entity as ModelEntity } from './model';

import { Comment as CommentModel, ParamCommand } from './model';

export interface CommentProps {
  comment: CommentModel
}

interface CommentState {

}

const Container = styled.div`
  width: 100%;
  height: 100%;
  font-family: 'Fira Code', monospace;
`;

const BlockCommand = styled.div`
  width: 100%;
`;

type Props = CommentProps;
type State = CommentState;

const toJsx = (comment: CommentChild[]): JSX.Element[] => {
  let components: JSX.Element[] = [];

  let params: ParamCommand[] = [];
  for (let i = 0; i < comment.length; ++i) {
    let child = comment[i];

    switch (child.type) {
      case "paramcommand": {
        params.push(child);
        break;
      }
      case "text": {
        const text = child.text.trim();
        if (text.length === 0) continue;
        components.push(
          <span>{child.text}</span>
        );
        break;
      }
      case "blockcommand": {
        components.push(
          <BlockCommand>
            {toJsx(child.children)}
          </BlockCommand>
        )
        break;
      }
    }
  }

  return components;
};

export class Comment extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { comment } = props;

    let components: JSX.Element[] = toJsx(comment);


    return (
      <Container>
        {/*entity.name}()*/}
        {components}
      </Container>
    );
  }
}