import * as React from "react";
import styled from "../styled-components";
import { useStore } from "../store";
import Editor from "./Editor";
import { paddings } from "../styles";

const StyledApp = styled.div`
  max-width: 800px;
  margin: 0 auto;
  padding: 0 ${paddings.medium};

  background-color: ${props => props.theme.colours.bg}
  color: ${props => props.theme.colours.text};
  line-height: 1.6;
  font-family: ${props => props.theme.fonts.text};

  /* Better Font Rendering =========== */
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;

  ::selection {
    background: ${props => props.theme.colours.accent};
  }
  ::-moz-selection {
    background: ${props => props.theme.colours.accent};
  }
`;

const start = (mymod: typeof import("../../../crate/pkg")) => {
  console.log("all modules loaded");
  console.log(mymod);

  const input = `
shape main() {
  svg(value: "hello world")
}
  `;

  const svg = mymod.compile(input);
  console.log(svg);
};

const load = async () => {
  start(await import("../../../crate/pkg"));
};

load();

export default () => {
  const { state, actions } = useStore();

  return (
    <StyledApp>
      <h1>(v)ector d(raw)</h1>
      <Editor />
    </StyledApp>
  );
};
