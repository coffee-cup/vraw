import * as React from "react";
import ReactSplitPane from "react-split-pane";
import styled from "../styled-components";
import { HeaderHeight } from "../styles";

export interface Props {
  children: React.ReactNode;
}

const DEFAULT_SIZE = "50%";
const SPLIT_PANE_KEY = "SPLIT_PANE";

const StyledSplitPane = styled.div`
  .SplitPane {
    height: calc(100vh - ${HeaderHeight}) !important;
  }

  .Resizer {
    background: #000;
    opacity: 0.2;
    z-index: 1;
    -moz-box-sizing: border-box;
    -webkit-box-sizing: border-box;
    box-sizing: border-box;
    -moz-background-clip: padding;
    -webkit-background-clip: padding;
    background-clip: padding-box;
  }

  .Resizer:hover {
    -webkit-transition: all 2s ease;
    transition: all 2s ease;
  }

  .Resizer.horizontal {
    height: 11px;
    margin: -5px 0;
    border-top: 5px solid rgba(255, 255, 255, 0);
    border-bottom: 5px solid rgba(255, 255, 255, 0);
    cursor: row-resize;
    width: 100%;
  }

  .Resizer.horizontal:hover {
    border-top: 5px solid rgba(0, 0, 0, 0.5);
    border-bottom: 5px solid rgba(0, 0, 0, 0.5);
  }

  .Resizer.vertical {
    width: 11px;
    margin: 0 -5px;
    border-left: 5px solid rgba(255, 255, 255, 0);
    border-right: 5px solid rgba(255, 255, 255, 0);
    cursor: col-resize;
  }

  .Resizer.vertical:hover {
    border-left: 5px solid rgba(0, 0, 0, 0.5);
    border-right: 5px solid rgba(0, 0, 0, 0.5);
  }
  .Resizer.disabled {
    cursor: not-allowed;
  }
  .Resizer.disabled:hover {
    border-color: transparent;
  }
`;

const SplitPane = (props: Props) => {
  const savedDefaultSize = localStorage.getItem(SPLIT_PANE_KEY);
  let defaultSize: string | number = DEFAULT_SIZE;
  if (savedDefaultSize != null) {
    defaultSize = parseInt(savedDefaultSize, 10);
  }

  return (
    <StyledSplitPane>
      <ReactSplitPane
        split="vertical"
        minSize={100}
        defaultSize={defaultSize}
        onChange={size => localStorage.setItem(SPLIT_PANE_KEY, size.toString())}
      >
        {props.children}
      </ReactSplitPane>
    </StyledSplitPane>
  );
};

export default SplitPane;
