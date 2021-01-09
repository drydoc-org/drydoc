import * as React from 'react';
import styled from 'styled-components';

import Page from '../../state/Page';
import { Comment } from './Comment';
import { Entity, Function as FunctionModel, Type, TypeKind } from './model';
import { ItemProps } from './Item';
import { LanguageKeyword, Title } from './style';
import Dict from '../../Dict';
import { JsxExpression } from 'typescript';
import { Link } from '../Link';
import display_name from './display_name';

export interface TypeItemProps {
  model: Type;
  symbols: Dict<Entity>;

  onPageChange: (id: string, event: React.MouseEvent<any>) => void;
}

interface TypeItemState {

}

const Container = styled.span`
  font-family: 'Fira Code', monospace;
`;

type Props = TypeItemProps;
type State = TypeItemState;

const KIND_MAPPING: Dict<string> = {
  "lvaluereference": "&",
  "rvaluereference": "&&",
  "pointer": "*"
};

const BEAUTIFY_REGEX = /const|void/g;

const beautify = (str: string) => {
  let matches = BEAUTIFY_REGEX.exec(str);
  if (!matches) return <span>str</span>;


};

export class TypeItem extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { model, symbols, onPageChange } = props;
    
    const { const_qualified, kind } = model;

    console.log('TYPE', model);

    const const_component = const_qualified ? <LanguageKeyword>const</LanguageKeyword> : undefined;

    let component: JSX.Element | undefined;
    switch (kind) {
      case 'lvaluereference': {
        const pointee = model.pointee as Type;
        component = (
          <>
             {pointee ? <TypeItem onPageChange={onPageChange} model={pointee} symbols={symbols} /> : undefined} <LanguageKeyword>{'&'}</LanguageKeyword>
          </>
        );
        break;
      }
      case 'record': {
        const pointee_component = model.pointee ? <TypeItem onPageChange={onPageChange} model={model.pointee} symbols={symbols} /> : undefined;
        component = (
          <>
            {const_component} {pointee_component} <Link pageId={model.name} onPageChange={onPageChange}>{display_name(model)}</Link>
          </>
        );
        break;
      }
      case 'elaborated': {
        const elaborated = model.elaborated as Type;
        component = (
          <>
            {const_component} {display_name(model)}
          </>
        );
        break;
      }
      case 'typedef': {
        component = (
          <>
            {const_component} {display_name(model)}
          </>
        );
        break;
      }
      case "bool": {
        component = (
          <>
            {const_component} <LanguageKeyword>bool</LanguageKeyword>
          </>
        );
        break;
      }
      case "chars": {
        component = (
          <>
            {const_component} <LanguageKeyword>char</LanguageKeyword>
          </>
        );
        break;
      }
      case "pointer": {
        const pointee_component = model.pointee ? <TypeItem onPageChange={onPageChange} model={model.pointee} symbols={symbols} /> : undefined;

        component = (
          <>
            {pointee_component} <LanguageKeyword>*</LanguageKeyword>
          </>
        );
        break;
      }
      case "int": {
        component = (
          <>
            <LanguageKeyword>int</LanguageKeyword>
          </>
        );
        break;
      }
      case "void": {
        component = (
          <>
            <LanguageKeyword>void</LanguageKeyword>
          </>
        );
        break;
      }
      default: {
        component = (
          <>
            {const_component} {display_name(model)}
          </>
        );
        break;
      }
    }

    return (
      <Container>
        {component}
      </Container>
    );
  }
}