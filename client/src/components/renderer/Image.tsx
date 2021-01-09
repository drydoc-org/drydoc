import * as React from 'react';
import styled from 'styled-components';
import Page from '../../state/Page';

export interface ImageProps {
  page: Page.Resolved;
  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

interface ImageState {
}

type Props = ImageProps;
type State = ImageState;

const Container = styled.img`
  width: 100%;
`;

export class Image extends React.Component<Props, State> {
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