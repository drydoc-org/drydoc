import * as React from 'react';
import styled from 'styled-components';
import Dict from '../../Dict';

import Page from '../../state/Page';
import { CommentChild, Entity as ModelEntity } from './model';

import { Comment as CommentModel, ParamCommand } from './model';
import { MONOSPACE_FONT_FAMILY } from './style';

export interface CommentProps {
  comment: CommentModel
}

interface CommentState {

}

const Container = styled.div`
  width: 100%;
  font-family: 'Fira Code', monospace;
`;

const BlockCommand = styled.div`
  width: 100%;
`;

const ParameterCommand = styled.div`
  width: 100%;
  margin-bottom: 0.25em;
  :last-child {
    margin-bottom: 0;
  }
`;


type Props = CommentProps;
type State = CommentState;

const SubHeader = styled.div`
  margin-top: 0.5em;
  font-size: 1.1em;
  font-weight: bold;
  margin-bottom: 0.25em;
`;

const SubSubHeader = styled.div`
  font-weight: bold;
`;

const Indent = styled.div`
  width: 100%;
  margin-left: 1em;
`;

const COMMAND_MAPPINGS: Dict<string> = {
  'return': "Returns",
  'brief': 'Description',
  'detailed': 'Detailed Description',
  'throws': 'Throws',
}

const toJsx = (comment: CommentChild[]): JSX.Element[] => {
  let components: JSX.Element[] = [];

  let params: ParamCommand[] = [];
  for (let i = 0; i < comment.length; ++i) {
    let child = comment[i];

    switch (child.type) {
      case "paramcommand": {
        params.push(child);
        break;
      }
      case "text": {
        const text = child.text.trim();
        if (text.length === 0) continue;
        components.push(
          <span>{child.text}</span>
        );
        break;
      }
      case "blockcommand": {
        components.push(
          <BlockCommand>
            <SubHeader>{COMMAND_MAPPINGS[child.command] || child.command}</SubHeader>
            <Indent>
              {toJsx(child.children)}
            </Indent>
          </BlockCommand>
        )
        break;
      }
    }
  }

  

  if (params.length > 0) components.unshift(
    <>
      <SubHeader>Parameters</SubHeader>
      <Indent>{...params.map((param, id) => (
      <ParameterCommand>
        <SubSubHeader style={{ fontFamily: MONOSPACE_FONT_FAMILY }}>{param.parameter || ''}</SubSubHeader>
        <Indent>
          {toJsx(param.children)}
        </Indent>
      </ParameterCommand>
      ))}
      </Indent>
    </>
  );
  return components;
};

export class Comment extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const { comment } = props;

    if (!comment) {
      return (
        <Container>
          No documentation provided
        </Container>
      )
    }

    let components: JSX.Element[] = toJsx(comment);

    console.log(comment);

    return (
      <Container>
        {/*entity.name}()*/}
        {components}
      </Container>
    );
  }
}