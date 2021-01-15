import * as React from 'react';
import styled from 'styled-components';

import PageModel from '../../state/Page';
import { Action } from '../ros/Action';

export interface RosActionProps {
  page: PageModel.Resolved;

  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

const Container = styled.div`
  padding: 20px;
  width: 100%;
  overflow-y: auto;
  overflow-x: hidden;
`;

type Props = RosActionProps;

export class RosAction extends React.PureComponent<Props> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const action = JSON.parse(props.page.content);

    return (
      <Container>
        <Action action={action} />
      </Container>
    );
  }
}