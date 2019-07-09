import * as React from "react";
import styled from "../styled-components";
import { paddings, margins } from "../styles";

const StyledOutput = styled.div`
  font-family: ${props => props.theme.fonts.code};
  padding: ${paddings.small};
  margin: ${margins.small} 0;
  border: solid 1px ${props => props.theme.colours.accent};
  border-radius: 4px;
`;

export interface Props {
  value: string;
}

const Output = (props: Props) => <StyledOutput>{props.value}</StyledOutput>;

export default Output;
