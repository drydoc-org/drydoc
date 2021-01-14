import * as React from 'react';
import styled from 'styled-components';
import Dict from '../../Dict';

import Page from '../../state/Page';
import { Error } from '../Error';
import { ClassItem } from './ClassItem';
import { EntityBrief } from './EntityBrief';
import { FunctionItem } from './FunctionItem';
import { ItemProps } from './Item';
import { PageData, Entity as EntityModel } from './model';
import { NamespaceItem } from './NamespaceItem';
import renderer from './renderer';
import { SubTitle } from './style';

import subSectionHeader from './sub-section-header';

export interface EntityProps {
  page: Page.Resolved;

  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

interface EntityState {

}

const Container = styled.div`
  width: 100%;
  font-family: 'Fira Code', monospace;
`;

type Props = EntityProps;
type State = EntityState;



const SubSectionContainer = styled.div`
  border-radius: 0.5em;
  overflow: hidden;
  border: 1px solid rgba(0, 0, 0, 0.3);
`;

export class Entity extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }

  private onClick_ = (id: string) => (event: React.MouseEvent<HTMLDivElement>) => {
    this.props.onPageChange(id, event);
  };
  
  render() {
    const { props } = this;

    const { page, onPageChange } = props;
    const { content } = page;

    let data: PageData;
    try {
      data = JSON.parse(content) as PageData
    } catch(e) {
      return <Error title='An Exception Occurred' message="Failed to parse JSON" />;
    }

    const { name, symbols } = data;

    const symbol = symbols[name];

    const childrenIds = EntityModel.children(symbol);
    const subsections: Dict<JSX.Element[]> = {};

    for (let i = 0; i < childrenIds.length; ++i) {
      const childId = childrenIds[i];
      const child = symbols[childId];
      
      if (!child) continue;

      const id = subSectionHeader(symbol, child);

      const element = <EntityBrief onPageChange={onPageChange} id={childId} symbols={symbols} />;
      subsections[id] = (subsections[id] ? [ ...subsections[id], element ] : [ element ]) 
    }

    const Renderer = renderer(symbol);
    console.log(subsections);
    if (!Renderer) return null;
    
    return (
      <Container>
        <Renderer onPageChange={onPageChange} model={symbol} depth={0} symbols={symbols} />
        {Dict.toList(subsections).map(([section, elements]) => (
          <>
            <SubTitle style={{ marginTop: '1.0em', marginBottom: '0.5em' }}>{section}</SubTitle>
            <SubSectionContainer>{elements}</SubSectionContainer>
          </>
        ))}
      </Container>
    );
  }
}