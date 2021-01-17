import * as React from 'react';

import { connect } from 'react-redux';
import styled from 'styled-components';
import { State as ReduxState } from '../store';

import Page from '../state/Page';
import { RouteComponentProps, withRouter } from 'react-router';
import { Icon, NAVIGATION_BACKGROUND_COLOR } from '../style';

import { PageData } from './doxygen/model';
import { LanguageKeyword } from './doxygen/style';
import { Search } from './Search';


export interface NavBarPublicProps {
  page: Page;
  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

interface NavBarPrivateProps {
  path: Page[];
  location: string;
}

/// `NavBar`'s React State
interface NavBarState {
  /// The number of hidden items on the nav bar (modified by the resize observer)
  hiddenItems: number;
}

type Props = NavBarPublicProps & NavBarPrivateProps;
type State = NavBarState;

const Container = styled.div`
  display: flex;
  align-items: center;
  flex-direction: row;  
  width: 100%;
  height: 28px;
  font-size: 12px;
  padding: 0;
  padding-left: 8px;
  padding-right: 8px;
  background-color: ${NAVIGATION_BACKGROUND_COLOR};
  overflow: hidden;
`;

const NavContainer = styled.div`
  display: flex;
  align-items: center;
  flex-direction; row;
  flex: 1 1;
  overflow: hidden;
`;

const NavInnerContainer = styled.div`
  display: flex;
  align-items: center;
  flex-direction; row;
  overflow: hidden;
`;

const ItemContainer = styled.div<any>`
  height: 28px;
  line-height: 28px;
  padding: 0;
  background-color: ${NAVIGATION_BACKGROUND_COLOR};
  user-select: none;
  padding-left: 5px;
  padding-right: 5px;
  :hover {
    background-color: rgba(255, 255, 255, 0.1);
  }
  cursor: pointer;
  transition: background-color 0.2s;
  font-family: ${(props: any) => props.monospace ? `'Fira Code', monospace` : 'inherit'};
`;


interface ItemProps {
  page: Page;
  terminal: boolean;

  onClick: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

const prefix = (page: Page) => {
  if (!page) return undefined;

  switch (page.content_type) {
    case "clang/namespace": return <LanguageKeyword>namespace</LanguageKeyword>;
    case "clang/class": return <LanguageKeyword>class</LanguageKeyword>;
    case "clang/struct": return <LanguageKeyword>struct</LanguageKeyword>;
    case "clang/typedef": return <LanguageKeyword>typedef</LanguageKeyword>;
    case "clang/enum": return <LanguageKeyword>enum</LanguageKeyword>;
  }

  if (page.state !== Page.State.Resolved) return undefined;

  

  return undefined;
};

const shouldMonospaceName = (page: Page) => {
  return page && page.content_type.startsWith('clang');
};

class Item extends React.PureComponent<ItemProps> {
  private onClick_ = (event: React.MouseEvent<HTMLDivElement>) => {
    this.props.onClick(this.props.page.id, event);
  }

  render() {
    const { props } = this;
    const { page } = props;

    if (!page) return null;

    return (
      <ItemContainer monospace={shouldMonospaceName(page)} onClick={this.onClick_}>
        {prefix(page)} {page.name}
      </ItemContainer>
    )
  }
}

class NavBar extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
  }

  private observer_: ResizeObserver;
  private registerObserver_ = () => {
    if (!this.navRef_ || !this.navInnerRef_ || this.observer_) return;

    this.observer_ = new ResizeObserver(entries => {
      let navEntry: ResizeObserverEntry | undefined = undefined;
      let navInnerEntry: ResizeObserverEntry | undefined = undefined;

      for (let i = 0; i < entries.length; ++i) {
        const entry = entries[i];

        if (entry.target === this.navRef_) {
          navEntry = entry;
          continue;
        }

        if (entry.target === this.navInnerRef_) {
          navInnerEntry = entry;
          continue;
        }
      }

      if (!navEntry || !navInnerEntry) return;

      const navSize = navEntry.borderBoxSize[0];
      const navInnerSize = navInnerEntry.borderBoxSize[0];

      // If the nav bar elements (navInnerSize) are about to overflow
      // the amount of space inside the flexbox (navSize)
      const remainingWidth = navInnerSize.inlineSize - navSize.inlineSize; 
      const { hiddenItems } = this.state;
      if (remainingWidth <= 20) {
        // Hide an item from the nav bar
        this.setState({
          hiddenItems: Math.min(this.props.path.length, hiddenItems + 1)
        });
      } else if (remainingWidth >= 300) {
        // Show an additional item on the nav bar
        this.setState({
          hiddenItems: Math.max(0, hiddenItems - 1)
        });
      } else {
        // Do nothing
      }
    });
  };

  private navRef_: HTMLDivElement;
  private bindNavRef_ = (navRef: HTMLDivElement) => {
    this.navRef_ = navRef;
    this.registerObserver_();
  }

  private navInnerRef_: HTMLDivElement;
  private bindNavInnerRef_ = (navInnerRef: HTMLDivElement) => {
    this.navInnerRef_ = navInnerRef;
    this.registerObserver_();
  }

  render() {
    const { props } = this;

    return (
      <Container>
        <NavContainer ref={this.bindNavRef_}>
          <NavInnerContainer ref={this.bindNavInnerRef_}>
            {props.path.map((page, i) => (
              <>
                <Item onClick={props.onPageChange} page={page} terminal={i + 1 == props.path.length} />
                {i + 1 !== props.path?.length ? <span><i style={{ marginLeft: '8px', marginRight: '8px' }} className='fa fa-xs fa-chevron-right' /></span> : undefined}
              </>
            ))}
          </NavInnerContainer>
        </NavContainer>
        <Search />
        <span><i style={{ margin: '4px' }} className='fa fa-bookmark' /></span>
        <i style={{ margin: '4px' }} className='fa fa-adjust' />
        <span style={{ marginLeft: '4px' }}>Generated by Drydoc</span>
      </Container>
    );
  }
}

export default connect((state: ReduxState, ownProps: Props) => {
  let path: Page[] = [];
  let current = ownProps.page.id;
  while (current) {
    path.push(state.page.pages[current]);
    current = state.page.byParent[current];
  }

  path = path.reverse();

  return {
    path
  };
}, (dispatch, ownProps) => {

})(NavBar) as React.ComponentType<NavBarPublicProps>;