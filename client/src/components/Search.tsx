import * as React from 'react';
import styled from 'styled-components';

export interface SearchProps {

}

interface SearchState {
  text: string;
}

type Props = SearchProps;
type State = SearchState;

const Input = styled.input`
  font-size: 11px;
  background-color: rgba(0, 0, 0, 0.1);
  border-radius: 0.5em;
  padding-top: 2px;
  padding-bottom: 2px;
  padding-left: 6px;
  padding-right: 6px;
  border: none;
  outline: none;
  color: white;
  width: 400px;
  margin-right: 5px;
  :hover {
    background-color: rgba(255, 255, 255, 0.05);
  }
  :focus {
    background-color: rgba(255, 255, 255, 0.1);
  }
`;

export class Search extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = {
      text: ''
    };
  }

  private onChange_ = (event: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({
      text: event.currentTarget.value
    });
  };
  
  render() {
    const { props, state } = this;

    const { text } = state;

    return (
      <Input type='text' onChange={this.onChange_} value={text} placeholder='Search...'></Input>
    )
  }
}