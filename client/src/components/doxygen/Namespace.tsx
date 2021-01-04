// import * as React from 'react';
// import styled from 'styled-components';

// import Page from '../../state/Page';
// import { NamespaceDef } from './defs';
// import { NamespaceItem } from './NamespaceItem';

// export interface NamespaceProps {
//   page: Page.Resolved;
// }

// interface NamespaceState {

// }

// const Container = styled.div`
//   width: 100%;
//   height: 100%;
// `;

// type Props = NamespaceProps;
// type State = NamespaceState;

// export class Namespace extends React.Component<Props, State> {
//   constructor(props: Props) {
//     super(props);

//     this.state = {};
//   }
  
//   render() {
//     const { props } = this;

//     const { page } = props;
//     const { content } = page;

//     const def: NamespaceDef = JSON.parse(content);

//     return (
//       <Container>
//         <NamespaceItem def={def} depth={0} />
//       </Container>
//     );
//   }
// }