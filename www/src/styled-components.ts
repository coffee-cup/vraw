import baseStyled, { ThemedStyledInterface } from "styled-components";
import { Theme } from "./styles";

export { ThemeProvider } from "styled-components";

const styled = baseStyled as ThemedStyledInterface<Theme>;
export default styled;
