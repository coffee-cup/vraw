export interface CompileError {
  line: number;
  column: number;
  message: string;
}

export interface CompileResult {
  svg?: string;
  error?: CompileError;
}

export const initialCode = `
shape circle() {
  svg(value: "<circle cx=\\"50\\" cy=\\"50\\" r=\\"10\\" fill=\\"hotpink\\" />")
}

shape main() {
  circle()
}`.trimLeft();

const loadModule = (): Promise<typeof import("../../crate/pkg")> => {
  return import("../../crate/pkg");
};

export const compile = async (value: string): Promise<CompileResult> => {
  const mod = await loadModule();
  const result = mod.compile(value);

  const error = result.get_error();
  const compileResult: CompileResult = {
    svg: result.get_svg(),
    error:
      error == null
        ? undefined
        : {
            line: error.line,
            column: error.column,
            message: error.get_message(),
          },
  };

  return compileResult;
};
