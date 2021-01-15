import * as React from 'react';
import styled from 'styled-components';
import { LanguageKeyword, MONOSPACE_FONT_FAMILY, Title } from './style';

import { Field as FieldModel } from './model';

export interface FieldProps {
  field: FieldModel;
}

type Props = FieldProps;

const Container = styled.div`
  border-radius: 0.5em;
  padding: 1em;
  font-size: 0.8em;
  overflow: hidden;
  background-color: rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(0, 0, 0, 0.3);
  margin-bottom: 1em;
`;

export class Field extends React.Component<Props> {
  render() {
    const { props } = this;

    const { field } = props;

    let fieldKind: JSX.Element | undefined = undefined;

    switch (field.kind.type) {
      case 'primitive': {
        fieldKind = <LanguageKeyword>{field.kind.kind}</LanguageKeyword>;
        break;
      }
      case 'reference': {
        fieldKind = <span style={{ fontFamily: 'inherit' }}>{`${field.kind.package}/${field.kind.name}`}</span>;
        break;
      }
    }

    let arrayKind: JSX.Element | undefined = undefined;
    switch (field.array_kind.type) {
      case 'variable': {
        arrayKind = <span style={{ fontFamily: 'inherit' }}>[]</span>;
        break;
      }
      case 'fixed': {
        arrayKind = <span style={{ fontFamily: 'inherit' }}>{`[${field.array_kind.size}]`}</span>;
        break;
      }
      default: {
        break;
      }
    }

    return (
      <Container>
        <Title style={{ fontFamily: MONOSPACE_FONT_FAMILY }}>{fieldKind} {arrayKind} <span style={{ fontFamily: 'inherit' }}>{field.name}</span></Title>
        {field.comment ? <div>{field.comment}</div> : undefined}
      </Container>
    );
  }
}