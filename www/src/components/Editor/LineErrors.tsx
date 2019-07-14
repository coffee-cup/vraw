import * as React from "react";
import { CompileError } from "../../compiler";
import * as codemirror from "codemirror";

export interface Props {
  error: CompileError;
  editor: codemirror.Editor;
}

const createErrorWidget = (error: CompileError) => {
  const line = document.createElement("div");
  line.className = "line-error";
  line.innerHTML = error.message;
  return line;
};

const LineErrors = (props: Props) => {
  React.useEffect(() => {
    const widget = props.editor.addLineWidget(
      props.error.line - 1,
      createErrorWidget(props.error),
    );

    return () => {
      widget.clear();
    };
  }, [props.error, props.editor]);

  return null;
};

export default LineErrors;
