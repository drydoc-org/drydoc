import * as React from 'react';
import styled from 'styled-components';
import Page from '../../state/Page';

export interface AudioProps {
  page: Page.Resolved;
  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

interface AudioState {
}

type Props = AudioProps;
type State = AudioState;

const Container = styled.audio`
  width: 100%;
  height: 100%;
`;

export class Audio extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;
    return (
      <Container src={props.page.url} />
    );
  }
}