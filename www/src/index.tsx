import * as React from "react";
import * as ReactDOM from "react-dom";
import App from "./components/App";
import { StateProvider } from "./store";

import "./index.scss";

const render = () => {
  ReactDOM.render(
    <StateProvider>
      <App />
    </StateProvider>,
    document.getElementById("root"),
  );
};

render();
