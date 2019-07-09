import * as React from "react";
import styled from "../styled-components";
import { paddings } from "../styles";

export interface Props {
  children: React.ReactNode;
}

const StyledTitle = styled.h1`
  margin-top: 0;
  padding-top: ${paddings.large};
`;

const Title = (props: Props) => <StyledTitle>{props.children}</StyledTitle>;

export default Title;
