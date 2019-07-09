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

const Canvas = () => (
  <StyledCanvas>
    <h1>Canvas</h1>
  </StyledCanvas>
);

export default Canvas;
