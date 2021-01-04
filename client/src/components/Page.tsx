import * as React from 'react';
import { connect } from 'react-redux';
import styled from 'styled-components';

import StatePage from '../state/Page';
import Resolver from '../state/Resolver';
import { State as ReduxState } from '../store';
import { Resolve,  } from '../store/page';
import { PAGE_BACKGROUND_COLOR } from '../style';
import { Doxygen } from './renderer/Doxygen';
import { Markdown } from './renderer/Markdown';

const Container = styled.div`
  flex: 1 1;
  background-color: ${PAGE_BACKGROUND_COLOR};
  overflow-y: auto;
`;

export interface PageProps {
  page: StatePage;
}

interface PageState {
}

type Props = PageProps;
type State = PageState;

export class Page extends React.PureComponent<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
    };
  }



  render() {
    const { props } = this;
    console.log('RENDER', props.page);

    if (!props.page) return null;

    const { page } = props;
    

    if (page.state != StatePage.State.Resolved) return null;


    let Renderer: React.ComponentType<{ page: StatePage.Resolved }> | null = null;
    switch (page.renderer) {
      case 'markdown': {
        Renderer = Markdown;
        break;
      }
      case 'clang': {
        Renderer = Doxygen;
        break;
      }
    }

    if (!Renderer) return null;


    return (
      <Container>
        <Renderer page={page} />
      </Container>
    );
  }
}

export default Page;