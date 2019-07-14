import * as codemirror from "codemirror";
import * as React from "react";
import { Controlled as CodeMirror } from "react-codemirror2";
import { CompileError } from "../../compiler";
import styled from "../../styled-components";
import LineErrors from "./LineErrors";
import { media, paddings } from "../../styles";

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
  overflow: hidden;
  padding: ${paddings.small};

  height: 100%;
  ${media.phone`height: auto;`}

  transition: border 250ms ease-in-out;

  .react-codemirror2 {
    height: 100%;
  }

  .CodeMirror {
    height: 100%;
  }

  .CodeMirror-placeholder {
    color: grey !important;
  }

  .line-error {
    background-color: ${props => props.theme.colours.error};
    padding: 0 ${paddings.small};
  }
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
  error: CompileError | null;
  setCode: (value: string) => any;
}

const Editor = (props: Props) => {
  const [editor, setEditor] = React.useState<codemirror.Editor | null>(null);

  return (
    <StyledEditor className="editor">
      <CodeMirror
        value={props.code}
        onBeforeChange={(editor, data, value) => props.setCode(value)}
        options={codemirrorOptions}
        editorDidMount={editor => setEditor(editor)}
      />

      {editor != null && props.error != null && (
        <LineErrors editor={editor} error={props.error} />
      )}
    </StyledEditor>
  );
};

export default Editor;
