import * as React from 'react';
import styled from 'styled-components';

import PageModel from '../../state/Page';
import { Message } from '../ros/Message';
import { MONOSPACE_FONT_FAMILY, Title } from '../ros/style';

export interface RosMessageProps {
  page: PageModel.Resolved;

  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

const Container = styled.div`
  padding: 20px;
  width: 100%;
  overflow-y: auto;
  overflow-x: hidden;
`;

type Props = RosMessageProps;

export class RosMessage extends React.PureComponent<Props> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const message = JSON.parse(props.page.content);

    return (
      <Container>
        <Title style={{ fontFamily: MONOSPACE_FONT_FAMILY }}>{`${message.package}/${message.name}`}</Title>
        <Message message={message} />
      </Container>
    );
  }
}