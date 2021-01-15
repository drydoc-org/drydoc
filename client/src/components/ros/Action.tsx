import * as React from 'react';

import { Action as ActionModel } from './model';

export interface ActionProps {
  action: ActionModel;
}

type Props = ActionProps;

export class Action extends React.Component<Props> {
  render() {
    const { props } = this;

    return null;
  }
}