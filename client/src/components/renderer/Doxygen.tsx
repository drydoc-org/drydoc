import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Entity } from '../doxygen/Entity';
// import { Function } from '../doxygen/Function';
// import { Namespace } from '../doxygen/Namespace';
// import { Class } from '../doxygen/Class';

export interface DoxygenProps {
  page: Page.Resolved;

  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

interface DoxygenState {

}

const Container = styled.div`
  width: 100%;
  margin: auto;
  padding: 20px;
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

    const { onPageChange } = props;

    return (
      <Container>
        <Entity onPageChange={onPageChange} page={page} />
      </Container>
    );
  }
}