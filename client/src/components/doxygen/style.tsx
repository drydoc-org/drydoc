import styled, { StyledComponent } from 'styled-components';

export const MONOSPACE_FONT_FAMILY = `'Fira Code', monospace`;

export const LanguageKeyword = styled.span`
  color: #FBDE2D;
  font-family: ${MONOSPACE_FONT_FAMILY};
`;

export const Title: StyledComponent<"div", any, any> = styled.div`
  font-size: 1.2em;
  margin-bottom: 0.5em;
  :last-child {
    margin-bottom: 0;
  }
`;

export const SubTitle: StyledComponent<"div", any, any> = styled.div`
  font-size: 1.1em;
`;

export interface StyleProps {
  style?: React.CSSProperties;
  className?: string;
}