const innerSvg = `
<svg viewBox=\\"0 0 100 100\\" xmlns=\\"http://www.w3.org/2000/svg\\">
  <circle cx=\\"50\\" cy=\\"50\\" r=\\"50\\"/>
</svg>`.trim();

export const initialCode = `
shape main() {
  svg(value: "${innerSvg}")
}
`.trimLeft();

const loadModule = (): Promise<typeof import("../../crate/pkg")> => {
  return import("../../crate/pkg");
};

export const compile = async (value: string): Promise<string> => {
  const mod = await loadModule();
  const result = mod.compile(value);

  return result;
};
