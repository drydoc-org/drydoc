// import * as React from 'react';
// import styled from 'styled-components';

// import Page from '../../state/Page';

// export interface ClassProps {
//   page: Page.Resolved;
// }

// interface ClassState {

// }

// const Container = styled.div`
//   width: 100%;
//   height: 100%;
// `;

// type Props = ClassProps;
// type State = ClassState;

// interface ClassDef {
//   name: string
// }

// export class Class extends React.Component<Props, State> {
//   constructor(props: Props) {
//     super(props);

//     this.state = {};
//   }
  
//   render() {
//     const { props } = this;

//     const { page } = props;
//     const { content } = page;

//     const def: ClassDef = JSON.parse(content);

//     return (
//       <Container>
//         {def.name}
//       </Container>
//     );
//   }
// }