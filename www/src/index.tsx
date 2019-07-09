import * as React from "react";
import * as ReactDOM from "react-dom";
import { ThemeProvider } from "styled-components";
import App from "./components/App";
import { theme } from "./styles";

import "./index.scss";

const render = () => {
  ReactDOM.render(
    <ThemeProvider theme={theme}>
      <App />
    </ThemeProvider>,
    document.getElementById("root"),
  );
};

render();
