import * as React from "react";
import styled from "styled-components";

import module from "../../crate/Cargo.toml";
console.log(module.bar("asdf"));

const StyledApp = styled.div`
  max-width: 800px;
  margin: 0 auto;
  padding: 0 1rem;
  height: 100vh;
  color: hotpink;
`;

export default () => (
  <StyledApp>
    <h1>Hello</h1>
  </StyledApp>
);
