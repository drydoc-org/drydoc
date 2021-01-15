import * as React from 'react';
import { connect } from 'react-redux';
import styled from 'styled-components';

import StatePage from '../state/Page';
import Resolver from '../state/Resolver';
import { State as ReduxState } from '../store';
import { Resolve,  } from '../store/page';
import { PAGE_BACKGROUND_COLOR } from '../style';
import { Audio } from './renderer/Audio';
import { Error } from './Error';
import { Doxygen } from './renderer/Doxygen';
import { Image } from './renderer/Image';
import { Markdown } from './renderer/Markdown';
import { Loading } from './renderer/Loading';
import { StyleProps } from './doxygen/style';
import { Video } from './renderer/Video';
import { RosMessage } from './renderer/RosMessage';
import { RosService } from './renderer/RosService';
import { RosAction } from './renderer/RosAction';

const Container = styled.div`
  flex: 1 1;
  overflow-y: auto;
  overflow-x: hidden;
`;

export interface PageProps extends StyleProps {
  page: StatePage;
  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
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
    const { page, onPageChange, style, className } = props;

    console.log('RENDER', props.page);

    if (!page) return (
      <Container style={style} className={className}>
        <Error title="Unkown Page" message={"This element does not reference a known page"} />
      </Container>
    )

    

    if (page.state != StatePage.State.Resolved) {
      return (
        <Container style={style} className={className}>
          <Loading />
        </Container>
      )
    }


    let Renderer: React.ComponentType<{ page: StatePage.Resolved, onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void }> | null = null;
    switch (page.renderer) {
      case 'clang': {
        Renderer = Doxygen;
        break;
      }
      case 'audio': {
        Renderer = Audio;
        break;
      }
      case 'image': {
        Renderer = Image;
        break;
      }
      case 'video': {
        Renderer = Video;
        break;
      }
    }

    switch (page.content_type) {
      case 'text/markdown': {
        Renderer = Markdown;
        break;
      }
      case 'ros/message': {
        Renderer = RosMessage;
        break;
      }
      case 'ros/service': {
        Renderer = RosService;
        break;
      }
      case 'ros/action': {
        Renderer = RosAction;
        break;
      }
    }

    if (!Renderer) {
      return (
        <Container style={style} className={className}>
          <Error title="Unable to render content" message={`Unable to find renderer "${page.renderer}" (content-type: "${page.content_type}")`} />
        </Container>
      )
    }


    return (
      <Container style={style} className={className}>
        <Renderer page={page} onPageChange={onPageChange} />
      </Container>
    );
  }
}

export default Page;