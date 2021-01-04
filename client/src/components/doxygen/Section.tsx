import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';


export interface SectionProps {
  expanded: boolean;
  collapsable: boolean;

  onExpandChange: (expand: boolean) => void;
  children?: any;
}

interface SectionState {
}

const Container = styled.div`
  width: 100%;
  height: 100%;
`;

type Props = SectionProps;
type State = SectionState;

export class Section extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    return (
      <Container>
      </Container>
    );
  }
}