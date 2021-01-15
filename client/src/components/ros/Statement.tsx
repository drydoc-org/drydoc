import * as React from 'react';
import { Field } from './Field';

import { Statement as StatementModel } from './model';

export interface StatementProps {
  statement: StatementModel;
}

type Props = StatementProps;

export class Statement extends React.Component<Props> {
  render() {
    const { props } = this;

    const { statement } = props;

    switch (statement.type) {
      case 'field': {
        return <Field field={statement} />;
      }
      case 'constant': {
        return null;
      }
    }
  }
}