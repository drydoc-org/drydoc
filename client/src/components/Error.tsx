import * as React from 'react';
import styled from 'styled-components';

export interface ErrorProps {
  title: string;
  message: string;
}

type Props = ErrorProps;

const Container = styled.div`
  padding: 20px;
`

export class Error extends React.PureComponent<Props> {
  render() {
    const { props } = this;
    const { title, message } = props;
    return (
      <Container>
        <div>Error: {title}</div>
        <div>{message}</div>
      </Container>
    )
  }
}