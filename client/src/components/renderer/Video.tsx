import * as React from 'react';
import styled from 'styled-components';
import Page from '../../state/Page';

export interface VideoProps {
  page: Page.Resolved;
  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

interface VideoState {
}

type Props = VideoProps;
type State = VideoState;

const Container = styled.video`
  width: 100%;
`;

export class Video extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;
    const { page } = props;
    return (
      <Container controls>
        <source src={page.url} type={page.content_type} />
      </Container>
    );
  }
}