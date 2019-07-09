import * as React from "react";
import * as ReactDOM from "react-dom";
import App from "./components/App";
import { ThemeProvider } from "styled-components";
import { StateProvider } from "./store";
import { theme } from "./styles";

import "./index.scss";

const render = () => {
  ReactDOM.render(
    <StateProvider>
      <ThemeProvider theme={theme}>
        <App />
      </ThemeProvider>
    </StateProvider>,
    document.getElementById("root"),
  );
};

render();
