import * as React from 'react';
import styled from 'styled-components';

export interface HtmlProps {
  html: string;
}

interface HtmlState {

}

const Container = styled.div`
  width: 100%;
  height: 100%;
`;

type Props = HtmlProps;
type State = HtmlState;

export class Html extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;
    return (
      <Container dangerouslySetInnerHTML={{ __html: props.html }} />
    );
  }
}