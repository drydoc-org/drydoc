import * as React from 'react';
import { connect } from 'react-redux';

import styled from 'styled-components';

import { NAVIGATION_BACKGROUND_COLOR } from '../style';
import { State as ReduxState } from '../store';
import Page from '../state/Page';
import Dict from '../Dict';
import { isPropertyAccessExpression } from 'typescript';
import { Section } from './doxygen/Section';

const ExplorerItemContainer = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  :hover {
    background-color: rgba(255, 255, 255, 0.1);
  }
  cursor: pointer;
  transition: background-color 0.25s;
`;

const ExplorerSectionContainer = styled.div`
  display: flex;
  flex-direction: column;
`;

const ExplorerTitleContainer = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  margin: 8px;
`;

const ItemText = styled.div`
  margin: 8px;
  font-size: 12px;
  transition: opacity 0.25s;
  user-select: none;
`;

const SectionText = styled.div`
  margin: 8px;
  font-size: 14px;
  font-weight: bold;
  transition: opacity 0.25s;
  user-select: none;
`;

const TitleText = styled.div`
  font-size: 16px;
  flex: 1 1;
  text-align: center;
  transition: opacity 0.25s;
  user-select: none;
`;

interface ExplorerTitleProps {
  title: string;

  onBack: ((event: React.MouseEvent<HTMLElement>) => void) | undefined;
}

export class ExplorerTitle extends React.PureComponent<ExplorerTitleProps> {
  render() {
    const { onBack } = this.props;
    return (
      <ExplorerTitleContainer>
        <i onClick={onBack} style={{ cursor: !!onBack ? 'pointer' : 'default', visibility: !!onBack ? 'visible' : 'hidden' }} className={'fa fa-chevron-left'} />
        <TitleText>{this.props.title}</TitleText>
        <i className={'fa fa-chevron-left'} style={{ visibility: 'hidden' }} />
      </ExplorerTitleContainer>
    );
  }
}

interface ExplorerItemProps {
  icon?: string;
  text: string;
  expanded: boolean;
  onClick: (event: React.MouseEvent<HTMLDivElement>) => void;
}

export class ExplorerItem extends React.Component<ExplorerItemProps> {
  render() {
    const { props } = this;
    return (
      <ExplorerItemContainer onClick={props.onClick}>
        <ItemText style={{ opacity: props.expanded ? 1 : 0 }}>{props.text}</ItemText>
      </ExplorerItemContainer>
    )
  }
}

interface ExplorerSectionProps {
  id: string;
  pages: Page[];
  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;

}

const SECTION_NAMES = {
  namespace: "Namespaces",
  class: "Classes",
  struct: "Structs",
  function: "Functions"
};

export class ExplorerSection extends React.Component<ExplorerSectionProps> {
  private onClick_ = (id: string) => (event: React.MouseEvent<HTMLDivElement>) => {
    this.props.onPageChange(id, event);
  };

  render() {
    const { props } = this;
    return (
      <ExplorerSectionContainer>
        <SectionText>{SECTION_NAMES[props.id] || props.id}</SectionText>
        {props.pages.map(page => (
          <ExplorerItem onClick={this.onClick_(page.id)} text={page.name} expanded />
        ))}
      </ExplorerSectionContainer>
    )
  }
}

const Container = styled.div`
  height: 100%;
  background-color: white;
  transition: width 0.25s;
  overflow-x: hidden;
  display: flex;
  flex-direction: column;
  overflow-y: hidden;
  background-color: ${NAVIGATION_BACKGROUND_COLOR};
`;


const ScrollArea = styled.div`
  flex: 1 1;
  overflow-y: auto;
`;

export interface ExplorerProps {
  page: Page;
  childPages?: Dict<Page>;
  parentId?: string;

  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

interface ExplorerState {
  expanded: boolean
}

type Props = ExplorerProps;
type State = ExplorerState;

export class Explorer extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      expanded: false
    };
  }

  private onMouseEnter_ = (event: React.MouseEvent<HTMLDivElement>) => {
    this.setState({
      expanded: true
    });
  };

  private onMouseLeave_ = (event: React.MouseEvent<HTMLDivElement>) => {
    this.setState({
      expanded: false
    });
  };

  private onBack_ = (event: React.MouseEvent<any>) => {
    
    console.log('BACK', this.props.parentId);
    this.props.onPageChange(this.props.parentId || '', event)
  }

  render() {
    const { props, state } = this;

    if (!props.childPages) return null;

    console.log('props', props);

    const sections: Dict<Page[]> = {};
    Dict.forEach(props.childPages, page => {
      const section = (page.metadata || {})['section'] || '';
      if (section in sections) {
        sections[section].push(page);
      } else {
        sections[section] = [ page ];
      }
    });

    

    return (
      <Container
        style={{ width: '256px' }}
        onMouseEnter={this.onMouseEnter_}
        onMouseLeave={this.onMouseLeave_}
      >
        <ExplorerTitle onBack={this.props.parentId !== undefined ? this.onBack_ : undefined} title={props.page.name} />
        <ScrollArea>
          {Dict.values(Dict.map(sections, (pages, id) => (
            <ExplorerSection onPageChange={props.onPageChange} pages={pages} id={id} />
          )))}
        </ScrollArea>
      </Container>
    );
  }
}

export default connect((state: ReduxState, ownProps: Props) => {
  console.log('parent', state.page.byParent[ownProps.page.id]);
  return {
    parentId: state.page.byParent[ownProps.page.id],
    childPages: Dict.subset(state.page.pages, ownProps.page.children)
  };
})(Explorer);