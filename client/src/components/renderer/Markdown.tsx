import * as React from 'react';
import styled from 'styled-components';

import * as marked from 'marked';

import Page from '../../state/Page';

export interface MarkdownProps {
  page: Page.Resolved;
}

interface MarkdownState {

}

const Container = styled.div`
  padding: 10px;
  width: 100%;
  max-width: 600px;
  margin: auto;
  height: 100%;
`;

type Props = MarkdownProps;
type State = MarkdownState;

export class Markdown extends React.PureComponent<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;
    const html = marked(props.page.content, {
      sanitize: true
    });
    return (
      <Container dangerouslySetInnerHTML={{ __html: html }} />
    );
  }
}