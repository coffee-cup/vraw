import * as React from "react";
import { compile, CompileError, initialCode } from "../compiler";
import styled from "../styled-components";
import { margins, media, paddings } from "../styles";
import Canvas from "./Canvas";
import Editor from "./Editor";
import Header from "./Header";
import Output from "./Output";
import SplitPane from "./SplitPane";
import { debounce } from "lodash";

const CODE_KEY = "CODE";

const StyledApp = styled.div`
  min-height: 100vh;
  margin: 0 auto;

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

const CanvasContainer = styled.div`
  margin-left: ${margins.small};
  height: 600px;

  ${media.phone`margin-left: 0;`}
  ${media.phone`margin-top: ${margins.small};`}
  ${media.phone`height: auto;`}
`;

const CenteredContainer = styled.div`
  width: 100%;
`;

const savedCode = localStorage.getItem(CODE_KEY);

const saveCodeToLocalStorage = debounce(
  (value: string) => localStorage.setItem(CODE_KEY, value),
  100,
);

const App = () => {
  const [code, setCode] = React.useState(savedCode || initialCode);
  const [svg, setSvg] = React.useState("");
  const [error, setError] = React.useState<CompileError | null>(null);

  const updateCode = (value: string) => {
    setCode(value);
    saveCodeToLocalStorage(value);
  };

  React.useEffect(() => {
    const fn = async () => {
      const result = await compile(code);

      if (result.svg) {
        setSvg(result.svg);
        setError(null);
      } else if (result.error) {
        setError(result.error);
      }
    };

    fn();
  }, [code]);

  return (
    <StyledApp className="app">
      <CenteredContainer>
        <Header />

        <SplitPane>
          <Editor code={code} setCode={updateCode} error={error} />
          <Canvas value={svg} isError={error != null} />
        </SplitPane>

        <Output svg={svg} error={error} />
      </CenteredContainer>
    </StyledApp>
  );
};

export default App;
