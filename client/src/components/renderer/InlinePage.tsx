import * as React from 'react';
import { connect } from 'react-redux';
import PageModel from '../../state/Page';

import Page from '../Page';

import { State as ReduxState } from '../../store';
import { StyleProps } from '../doxygen/style';
import styled from 'styled-components';

export interface InlinePagePublicProps extends StyleProps {
  id: string;
}

interface InlinePagePrivateProps {
  page: PageModel;
  resolve: (id: string) => void;
}

interface InlinePageState {
}

type Props = InlinePagePublicProps & InlinePagePrivateProps;
type State = InlinePageState;

const Container = styled.div`
  width: 100%;
  border-radius: 10px;
  border: 1px solid rgba(0, 0, 0, 0.3);
  background-color: rgba(0, 0, 0, 0.1);
  min-height: 300px;
  max-height: 80%;
  overflow-y: auto;
  overflow-x: hidden;
`;

const Bar = styled.div`
  border-top: 1px solid rgba(0, 0, 0, 0.3);
`;

class InlinePage extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {

    };
  }

  componentWillReceiveProps(nextProps: Props) {
    if (nextProps.page.state === PageModel.State.Unresolved) {
      this.props.resolve(nextProps.page.id);
    }
  }

  private onPageChange_ = () => {

  };
  
  render() {
    const { props } = this;
    const { page, style, className } = props;
    if (!page) {
      return null;
    }
    
    if (page.state === PageModel.State.Unresolved) {
      this.props.resolve(page.id);
    }

    return (
      <Container>
        <Page style={{ ...style, height: 'auto' }} className={className} page={page} onPageChange={this.onPageChange_} />
      </Container>
    );
  }
}

export default connect((state: ReduxState, ownProps: InlinePagePublicProps) => {
  console.log(state.page.pages[ownProps.id], ownProps.id)
  return {
    page: state.page.pages[ownProps.id]
  };
}, (dispatch, ownProps: InlinePagePublicProps) => ({
  resolve: (id: string) => dispatch({
    type: 'page-resolve',
    id
  })
}))(InlinePage);