import * as React from 'react';
import styled from 'styled-components';
import Explorer from './components/Explorer';
import NavBar from './components/NavBar';
import Page from './components/Page';
import StatePage from './state/Page';

import { RouteComponentProps, withRouter } from 'react-router';
import { State as ReduxState } from './store';
import { connect } from 'react-redux';

import { Resolve } from './store/page';
import { push, replace, routerActions } from 'connected-react-router';

const Row = styled.div`
  flex: 1 1;
  display: flex;
  flex-direction: row;
  overflow: hidden;
`;

const Container = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  color: white;
`;

export interface AppPublicProps {
  
}

export interface AppPrivateProps extends AppPublicProps {
  page: StatePage;
  root: string;
  resolve: (id: string) => void;
  goTo: (id: string) => void;
}

interface AppState {

}

type Props = AppPrivateProps;
type State = AppState;

export class App extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.resolve_(props.page);
    this.state = {};
  }

  private resolve_ = (page: StatePage) => {
    if (page.state !== StatePage.State.Unresolved) return;
    this.props.resolve(page.id);
  };

  // This method is deprecated. :( Research alternative
  componentWillReceiveProps(nextProps: Props) {
    this.resolve_(nextProps.page);
  }

  private onPageChange_ = (id: string, event: React.MouseEvent<HTMLDivElement>) => {
    this.props.goTo(id);
    event.preventDefault();
    event.stopPropagation();
  }

  render() {
    const { props, state } = this;

    const { page, root } = props;

    if (!page) {
      setTimeout(() => {
        props.goTo(root);
      }, 5);
      return null;
    }

    document.title = page ? page.name : 'Drydoc';

    return (
      <Container>
        <NavBar onPageChange={this.onPageChange_} page={page} />
        <Row>
          <Explorer onPageChange={this.onPageChange_} page={page} />
          <Page onPageChange={this.onPageChange_} page={page} />
        </Row>
      </Container>
    )
  }
}

export default connect((state: ReduxState, ownProps: Props) => {
  const pageId = state.router.location.hash.slice(2);
  return {
    root: state.page.root,
    page: state.page.pages[pageId]
  };
}, (dispatch, ownProps) => {
  return {
    resolve: (id: string) => dispatch({
      type: "page-resolve",
      id
    } as Resolve),
    goTo: (id: string) => {
      dispatch(push(`#/${id}`));
    }
  }
})(App) as React.ComponentType<AppPublicProps>;
