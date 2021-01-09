import * as React from 'react';
import styled from 'styled-components';

export interface LoadingProps {

}

type Props = LoadingProps;

const Container = styled.div`
  display: flex;
  width: 100%;
  height: 100%;
  flex-direction: row;
  align-items: center;
  justify-content: center;
`;

const InnerContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

export class Loading extends React.PureComponent<Props> {
  render() {
    return (
      <Container>
        <InnerContainer>
          <i style={{ fontSize: '3em' }} className="fa fa-cog fa-spin" />
          <p>Loading...</p>
        </InnerContainer>
      </Container>
    )
  }
}