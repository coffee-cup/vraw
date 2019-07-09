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

export const compile = async (value: string): Promise<string> => {
  const mod = await loadModule();
  const result = mod.compile(value);

  return result;
};
