import * as React from "react";
import styled from "../styled-components";
import { margins, paddings } from "../styles";

const StyledOutput = styled.div`
  font-family: ${props => props.theme.fonts.code};
  padding: ${paddings.small};
  margin: ${margins.small} 0;
  background-color: ${props => props.theme.colours.border};
  border-radius: 4px;
  white-space: pre;
`;

export interface Props {
  value: string;
}

const Output = (props: Props) => <StyledOutput>{props.value}</StyledOutput>;

export default Output;
