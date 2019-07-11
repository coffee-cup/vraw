import * as React from "react";
import styled from "../styled-components";
import { margins, media } from "../styles";

const StyledCanvas = styled.div<{ isError: boolean }>`
  width: 100%;
  text-align: center;
  border: solid 1px transparent;
  border-radius: 4px;

  height: 100%;
  ${media.phone`height: auto;`}

  transition: border 250ms ease-in-out;
`;

export interface Props {
  value: string;
  isError: boolean;
}

const Canvas = (props: Props) => (
  <StyledCanvas isError={props.isError}>
    <div dangerouslySetInnerHTML={{ __html: props.value }} />
  </StyledCanvas>
);

export default Canvas;
