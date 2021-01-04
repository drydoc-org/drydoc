// import * as React from 'react';
// import styled from 'styled-components';

// import Page from '../../state/Page';
// import { FunctionDef } from './defs';

// export interface FunctionProps {
//   page: Page.Resolved;
// }

// interface FunctionState {

// }

// const Container = styled.div`
//   width: 100%;
//   height: 100%;
//   font-family: 'Fira Code', monospace;
// `;

// type Props = FunctionProps;
// type State = FunctionState;



// export class Function extends React.Component<Props, State> {
//   constructor(props: Props) {
//     super(props);

//     this.state = {};
//   }
  
//   render() {
//     const { props } = this;

//     const { page } = props;
//     const { content } = page;

//     const def: FunctionDef = JSON.parse(content);

//     return (
//       <Container>
//         {def.name}()
//         <p>{def.detailed_description}</p>
//       </Container>
//     );
//   }
// }