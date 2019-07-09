import * as React from "react";
import styled from "../styled-components";
import { useStore } from "../store";
import { paddings, margins, media } from "../styles";
import Editor from "./Editor";
import Canvas from "./Canvas";
import Output from "./Output";
import Title from "./Title";

const StyledApp = styled.div`
  max-width: 1200px;
  min-height: 100vh;
  margin: 0 auto;
  padding: 0 ${paddings.medium};

  display: flex;
  flex-direction: row;
  align-items: center;

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

const SvgContainer = styled.div`
  display: flex;

  ${media.phone`display: block;`}
`;

const EditorContainer = styled.div`
  flex-basis: 500px;
`;

const CanvasContainer = styled.div`
  flex-grow: 1;
  margin-left: ${margins.small};
  height: 600px;

  ${media.phone`margin-left: 0;`}
  ${media.phone`margin-top: ${margins.small};`}
  ${media.phone`height: auto;`}
`;

const CenteredContainer = styled.div`
  width: 100%;
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

const App = () => {
  return (
    <StyledApp className="app">
      <CenteredContainer>
        <Title>vraw</Title>

        <SvgContainer>
          <EditorContainer>
            <Editor />
          </EditorContainer>

          <CanvasContainer>
            <Canvas />
          </CanvasContainer>
        </SvgContainer>
        <Output />
      </CenteredContainer>
    </StyledApp>
  );
};

export default App;
