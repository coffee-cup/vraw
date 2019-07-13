import * as React from "react";
import styled from "../styled-components";
import { paddings } from "../styles";

export interface Props {
  onClick: () => any;
  children: React.ReactNode;
  disabled?: boolean;
}

const StyledButton = styled.button<{ isDisabled: boolean }>`
  appearance: none;
  padding: ${paddings.small} ${paddings.medium};
  background-color: ${props =>
    !props.isDisabled ? props.theme.colours.primary : "lightgrey"}
  font-weight: bold;
  border-radius: 4px;
  border: none;
  cursor: pointer;

  transition: background-color 150ms ease-in-out;

  &:hover {
    background-color: ${props =>
      !props.isDisabled ? props.theme.colours.secondary : "lightgrey"}
  }

`;

const Button = (props: Props) => (
  <StyledButton onClick={props.onClick} isDisabled={!!props.disabled}>
    {props.children}
  </StyledButton>
);

export default Button;
