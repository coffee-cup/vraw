import * as codemirror from "codemirror";
import * as React from "react";
import { Controlled as CodeMirror } from "react-codemirror2";
import styled from "../../styled-components";
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
  border: solid 1px ${props => props.theme.colours.border};
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
  lineWrapping: true,
  autoCloseBrackets: true,
  placeholder: "Enter some shapes",
  styleActiveLine: true,
};

export interface Props {
  code: string;
  setCode: (value: string) => any;
}

const Editor = (props: Props) => {
  return (
    <StyledEditor className="editor">
      <CodeMirror
        value={props.code}
        onBeforeChange={(editor, data, value) => props.setCode(value)}
        options={codemirrorOptions}
      />
    </StyledEditor>
  );
};

export default Editor;
