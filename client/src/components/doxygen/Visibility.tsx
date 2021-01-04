import * as React from 'react';

import { Visibility as VisibilityModel } from './model'

export interface VisibilityProps {
  model?: VisibilityModel;
}

type Props = VisibilityProps;

export class Visibility extends React.PureComponent<Props> {
  render() {
    const { props } = this;
    const { model } = props;

    if (!model) return null;

    switch (model) {
      
    }

    return null;
  }
}