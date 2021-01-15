import * as React from 'react';

import { Constant as ConstantModel } from './model';

export interface ConstantProps {
  constant: ConstantModel;
}

type Props = ConstantProps;

export class Constant extends React.Component<Props> {
  render() {
    const { props } = this;

    const { constant} = props;

    return null;
  }
}