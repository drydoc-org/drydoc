import styled, { StyledComponent } from 'styled-components';

export const LanguageKeyword = styled.span`
  color: #4EC9B0;
  font-family: 'Fira Code', monospace;
`;

export const Title: StyledComponent<"div", any, any> = styled.div`
  font-size: ${(props: any) => Math.max(3 - props.depth, 0) * 4 + 14}px;
  margin-top: ${(props: any) => Math.max(3 - props.depth, 0) * 4 + 6}px;
  margin-bottom: ${(props: any) => Math.max(3 - props.depth, 0) * 4 + 6}px;
`;

export const SubTitle: StyledComponent<"div", any, any> = styled.div`
  font-size: ${(props: any) => Math.max(3 - props.depth, 0) * 4 + 12}px;
  margin-top: ${(props: any) => Math.max(3 - props.depth, 0) * 4 + 4}px;
  margin-bottom: ${(props: any) => Math.max(3 - props.depth, 0) * 4 + 4}px;
`;