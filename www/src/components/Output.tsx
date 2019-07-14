import * as React from "react";
import * as prettifyXml from "prettify-xml";
import { CompileError } from "../compiler";
import styled from "../styled-components";
import { margins, paddings } from "../styles";
import Button from "./Button";
import Modal from "./Modal";

const StyledOutput = styled.div<{ isError: boolean }>`
  appearance: none;
  position: absolute;
  bottom: ${paddings.medium};
  right: ${paddings.medium};
`;

const SvgOutput = styled.div`
  font-family: ${props => props.theme.fonts.code};
  white-space: pre;
`;

export interface Props {
  svg: string;
  error: CompileError | null;
}

const Output = (props: Props) => {
  const formatError = (error: CompileError): string =>
    `[${error.line}:${error.column}] ${error.message}`;

  const [isOpen, setIsOpen] = React.useState(false);

  return (
    <StyledOutput isError={props.error != null} className="output">
      <Button onClick={() => setIsOpen(true)} disabled={props.error != null}>
        svg
      </Button>
      <Modal
        key="output"
        isOpen={isOpen}
        onRequestClose={() => setIsOpen(false)}
      >
        <SvgOutput>
          {props.error == null
            ? prettifyXml(props.svg, { indent: 2 })
            : formatError(props.error)}
        </SvgOutput>
      </Modal>
    </StyledOutput>
  );
};

export default Output;
