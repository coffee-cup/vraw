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
shape main() {
  circle(r: 100, cx:100, cy: 100, fill: \"rgba(0, 0, 255, 0.45)\")
  circle(r: 100, cx:200, cy: 100, fill: \"rgba(255, 0, 0, 0.5)\")
  circle(r: 100, cx:150, cy: 200, fill: \"rgba(0, 255, 0, 0.5)\")
}
`.trimLeft();

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
