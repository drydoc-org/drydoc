import * as React from 'react';
import styled from "styled-components";

export interface LinkProps {
  children?: any;
  pageId?: string;
  href?: string;
  onPageChange?: (id: string, event: React.MouseEvent<any>) => void;
}

type Props = LinkProps;

const Container = styled.a`
  color: inherit;
  cursor: pointer;
  font-family: inherit;
  :hover {
    text-decoration: underline;
  }
`;

export class Link extends React.PureComponent<Props> {
  constructor(props: Props) {
    super(props);
  }

  private onClick_ = (event: React.MouseEvent<any>) => {
    const { props } = this;
    const { pageId, href, onPageChange } = props;
    if (pageId && onPageChange) {
      onPageChange(pageId, event);
    } else if (href) {
      window.location.assign(href);
    }
  }

  render() {
    const { props } = this;
    const { children } = props;
  
    return (
      <Container onClick={this.onClick_}>{children}</Container>
    );
  }
}