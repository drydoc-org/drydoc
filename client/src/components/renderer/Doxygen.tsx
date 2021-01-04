import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Entity } from '../doxygen/Entity';
// import { Function } from '../doxygen/Function';
// import { Namespace } from '../doxygen/Namespace';
// import { Class } from '../doxygen/Class';

export interface DoxygenProps {
  page: Page.Resolved;
}

interface DoxygenState {

}

const Container = styled.div`
  width: 100%;
  max-width: 720px;
  margin: auto;
  height: 100%;
  padding: 10px;
`;

type Props = DoxygenProps;
type State = DoxygenState;

export class Doxygen extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;
    const { page } = props;

    if (page.content_type === "clang/home") {
      return null;
    }

    return (
      <Container>
        <Entity page={page} />
      </Container>
    );
  }
}