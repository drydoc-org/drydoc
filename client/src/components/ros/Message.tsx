import * as React from 'react';

import styled from 'styled-components';

import { Message as MessageModel } from './model';
import { Statement } from './Statement';
import { Title } from './style';

export interface MessageProps {
  message: MessageModel;
}

type Props = MessageProps;

const Container = styled.div`
  width: 100%;
  font-family: 'Fira Code', monospace;
`;

export class Message extends React.Component<Props> {
  render() {
    const { props } = this;

    const { message } = props;


    const statements = message.statements.map((statement, i) => (
      <Statement key={i} statement={statement} />
    ));

    if (statements.length === 0) {
      return (
        <Container>
          <span style={{ marginBottom: '1em' }}>This message has no fields</span>
        </Container>
      );
    }

    return (
      <Container>
        {message.comment && message.comment.length > 0 ? <div>
          {message.comment}
        </div> : undefined}
        {statements}
      </Container>
    );
  }
}