import * as React from "react";
import styled from "styled-components";

const start = (mymod: typeof import("../../crate/pkg")) => {
  console.log("all modules loaded");
  console.log(mymod);

  const foo = mymod.bar("this is a test");
  console.log(foo.go_riders("woot"));
};

const load = async () => {
  start(await import("../../crate/pkg"));
};

load();

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
