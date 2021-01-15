import * as React from 'react';
import styled from 'styled-components';

import { Message } from './Message';

import { Service as ServiceModel } from './model';
import { Title } from './style';

export interface ServiceProps {
  service: ServiceModel;
}

type Props = ServiceProps;

const Container = styled.div`
  width: 100%;
  font-family: 'Fira Code', monospace;
`;

export class Service extends React.Component<Props> {
  render() {
    const { props } = this;

    const { service } = props;

    const { request, response } = service;

    return (
      <Container>
        <Title>Request</Title>
        <Message message={request} />
        <Title>Response</Title>
        <Message message={response} />
      </Container>
    );
  }
}