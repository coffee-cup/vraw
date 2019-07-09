import { css, FlattenSimpleInterpolation } from "styled-components";

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
    code: Font;
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

    code: `inconsolata, monospace;`,
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

export type DeviceType = "massive" | "desktop" | "laptop" | "tablet" | "phone";

export interface DeviceQuery {
  minWidth?: number;
  maxWidth?: number;
}

export const devices: { [D in DeviceType]: DeviceQuery } = {
  massive: { minWidth: 1801 },
  desktop: { minWidth: 1201, maxWidth: 1800 },
  laptop: { minWidth: 992, maxWidth: 1200 },
  tablet: { minWidth: 769, maxWidth: 991 },
  phone: { maxWidth: 768 },
};

const mediaFn = ({ minWidth, maxWidth }: DeviceQuery) => (
  first: any,
  ...interpolations: any[]
) =>
  css`
      @media only screen ${minWidth != null &&
        ` and (min-width: ${minWidth}px)`}${maxWidth != null &&
    ` and (max-width: ${maxWidth}px)`} {
        ${css(first, ...interpolations)}
      }
    `;

type MediaType = {
  [D in DeviceType]: (
    first: any,
    ...interpolations: any[]
  ) => FlattenSimpleInterpolation;
};

export const media = Object.keys(devices).reduce(
  (acc, key) => ({
    ...acc,
    [key]: mediaFn(devices[key]),
  }),
  ({} as any) as MediaType,
);
