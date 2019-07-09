import * as React from "react";
import styled from "../../styled-components";
import * as codemirror from "codemirror";
import { Controlled as CodeMirror } from "react-codemirror2";
import { useStore } from "../../store";
import { media } from "../../styles";

import "codemirror/lib/codemirror.css";
import "./theme.css";

import "codemirror/addon/comment/comment";
import "codemirror/addon/dialog/dialog";
import "codemirror/addon/display/placeholder";
import "codemirror/addon/edit/closebrackets";
import "codemirror/addon/edit/matchbrackets";
import "codemirror/addon/selection/active-line";
import "codemirror/mode/javascript/javascript";

const StyledEditor = styled.div`
  border: solid 1px ${props => props.theme.colours.accent};
  border-radius: 4px;
  overflow: hidden;

  height: 100%;
  ${media.phone`height: auto;`}
`;

const codemirrorOptions: codemirror.EditorConfiguration = {
  mode: "javascript",
  theme: "summerfruit",
  indentUnit: 2,
  tabSize: 2,
  indentWithTabs: true,
  lineNumbers: false,
  lineWrapping: false,
  autoCloseBrackets: true,
  placeholder: "Enter some shapes",
  styleActiveLine: true,
};

const Editor = () => {
  const { state, actions } = useStore();

  return (
    <StyledEditor className="editor">
      <CodeMirror
        value={state.code}
        onBeforeChange={(editor, data, value) => actions.changeCode(value)}
        options={codemirrorOptions}
      />
    </StyledEditor>
  );
};

export default Editor;
