import * as React from 'react';

import { Accessibility as AccessibilityModel } from './model'
import { LanguageKeyword } from './style';

export interface AccessibilityProps {
  model?: AccessibilityModel;
}

type Props = AccessibilityProps;

export class Accessibility extends React.PureComponent<Props> {
  render() {
    const { props } = this;
    const { model } = props;

    if (!model) return null;

    switch (model) {
      case 'public': return <LanguageKeyword>public</LanguageKeyword>;
      case 'protected': return <LanguageKeyword>protected</LanguageKeyword>;
      case 'private': return <LanguageKeyword>private</LanguageKeyword>;
    }

    return null;
  }
}