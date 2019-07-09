import { css } from "styled-components";

export type Colour = string;
export type Font = string;

export interface Theme {
  colours: {
    bg: Colour;
    text: Colour;
    primary: Colour;
    secondary: Colour;
    accent: Colour;
  };
  fonts: {
    text: Font;
  };
}

export const theme: Theme = {
  colours: {
    bg: "white",
    text: "#333",
    primary: "peachpuff",
    secondary: "hotpink",
    accent: "hotpink",
  },
  fonts: {
    text: `-apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", Roboto, Ubuntu`,
  },
};

export const paddings = {
  none: "0",
  small: "0.5rem",
  medium: "1rem",
  large: "2rem",
};

export const margins = paddings;

export const NarrowScreenWidth = 768;

export const forNarrowScreen = (first: any, ...interpolations: any[]) => css`
  @media only screen ${NarrowScreenWidth != null &&
      css` and (max-width: ${NarrowScreenWidth}px)`} {
    ${css(first, ...interpolations)}
  }
`;

export const forWideScreen = (first: any, ...interpolations: any[]) => css`
  @media only screen ${NarrowScreenWidth != null &&
      css` and (min-width: ${NarrowScreenWidth}px)`} {
    ${css(first, ...interpolations)}
  }
`;
