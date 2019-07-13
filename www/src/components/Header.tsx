import * as React from "react";
import styled from "../styled-components";
import { HeaderHeight, paddings } from "../styles";
import Title from "./Title";

const StyledHeader = styled.header`
  display: flex;
  align-items: center;
  padding-left: ${paddings.large};
  height: ${HeaderHeight};
  background: linear-gradient(
    to bottom right,
    ${props => props.theme.colours.primary},
    ${props => props.theme.colours.secondary}
  );
  font-size: 1.2em;

  h1 {
    margin: 0;
    padding: 0;
  }
`;

const Header = () => (
  <StyledHeader>
    <Title>vraw</Title>
  </StyledHeader>
);

export default Header;
