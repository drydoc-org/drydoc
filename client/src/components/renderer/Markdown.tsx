import * as React from 'react';
import styled from 'styled-components';

import * as ReactMarkdown from 'react-markdown';
import {Prism as SyntaxHighlighter} from 'react-syntax-highlighter'
import {dark} from 'react-syntax-highlighter/dist/esm/styles/prism'

import { Page } from '../Page';

import PageModel from '../../state/Page';
import { Loading } from './Loading';
import { Link } from '../Link';
import InlinePage from './InlinePage';

export interface MarkdownProps {
  page: PageModel.Resolved;

  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

interface MarkdownState {

}

const Container = styled.div`
  padding: 20px;
  width: 100%;
  overflow-y: auto;
  overflow-x: hidden;
`;

type Props = MarkdownProps;
type State = MarkdownState;

const PAGE_PROTOCOL = 'page:';
const INLINE_PAGE_PROTOCOL = 'inline-page:';

const RENDERERS: any = {
  code: ({language, value}) => {
    return <SyntaxHighlighter style={dark} language={language} children={value} />
  },
  link: (props) => {
    const { href, children, node } = props;
    const url = new URL(href);
    switch (url.protocol) {
      case PAGE_PROTOCOL: {
        return <Link pageId={href.slice(PAGE_PROTOCOL.length + 2)} children={children} />;
      }
      case INLINE_PAGE_PROTOCOL: {
        return <InlinePage id={href.slice(INLINE_PAGE_PROTOCOL.length + 2)} />;
      }
      case "javascript:": {
        if (node.url.startsWith(`${PAGE_PROTOCOL}//`)) {
          return <InlinePage id={node.url.slice(PAGE_PROTOCOL.length + 2)} />;
        }
        if (node.url.startsWith(`${INLINE_PAGE_PROTOCOL}//`)) {
          return <InlinePage id={node.url.slice(INLINE_PAGE_PROTOCOL.length + 2)} />;
        }
      }
      default: {
        return <Link href={href} children={children} />
      }
    }
  }
};

export class Markdown extends React.PureComponent<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    return (
      <Container>
        <ReactMarkdown renderers={RENDERERS} children={props.page.content} />
      </Container>
    );
  }
}