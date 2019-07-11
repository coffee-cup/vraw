import * as React from "react";
import { CompileError } from "../compiler";
import styled from "../styled-components";
import { margins, paddings } from "../styles";

const StyledOutput = styled.div<{ isError: boolean }>`
  font-family: ${props => props.theme.fonts.code};
  padding: ${paddings.small};
  margin: ${margins.small} 0;
  background-color: ${props =>
    props.isError ? "#ff00005c" : props.theme.colours.border};
  border-radius: 4px;
  white-space: pre;

  transition: background-color 250ms ease-in-out;
`;

export interface Props {
  svg: string;
  error: CompileError | null;
}

/* const Output = (props: Props) => (props.error != null ? <StyledOutput>{props.value}</StyledOutput>); */
const Output = (props: Props) => {
  const formatError = (error: CompileError): string =>
    `[${error.line}:${error.column}] ${error.message}`;

  return (
    <StyledOutput isError={props.error != null}>
      {props.error == null ? props.svg : formatError(props.error)}
    </StyledOutput>
  );
};

export default Output;
