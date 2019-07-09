import * as React from "react";
import styled from "../styled-components";
import { margins, media } from "../styles";

const StyledCanvas = styled.div`
  width: 100%;
  text-align: center;
  border: solid 1px ${props => props.theme.colours.accent};

  height: 100%;
  ${media.phone`height: auto;`}
`;

export interface Props {
  value: string;
}

const Canvas = (props: Props) => (
  <StyledCanvas>
    <div dangerouslySetInnerHTML={{ __html: props.value }} />
  </StyledCanvas>
);

export default Canvas;
