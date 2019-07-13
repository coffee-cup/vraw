import * as React from "react";
import ReactModal from "react-modal";
import styled from "../../styled-components";
import { margins, paddings } from "../../styles";

import "./index.scss";

export interface Props {
  isOpen: boolean;
  key: string;
  onRequestClose: () => any;
  children?: React.ReactNode;
}

const Container = styled.div`
  background-color: white;
  padding: ${paddings.medium};
`;

const CloseButton = styled.button`
  appearance: none;
  border: none;
  position: absolute;
  right: ${paddings.small};
  bottom: ${paddings.small};
  cursor: pointer;
  background: white;
  font--size: 0.7em;
  color: darkgrey;
`;

const Modal = (props: Props) => (
  <ReactModal
    isOpen={props.isOpen}
    closeTimeoutMS={100}
    style={{
      overlay: { zIndex: 1000, backgroundColor: "rgba(0,0,0,0.5)" },
      content: {
        top: "50%",
        left: "50%",
        right: "auto",
        bottom: "auto",
        marginRight: "-50%",
        padding: "0",
        maxWidth: "800px",
        maxHeight: "800px",
        transform: "translate(-50%, -50%)",
        border: "none",
        borderRadius: "8px",
        backgroundColor: "transparent",
      },
    }}
    onRequestClose={props.onRequestClose}
  >
    <Container>
      {props.children}
      <CloseButton onClick={props.onRequestClose}>close</CloseButton>
    </Container>
  </ReactModal>
);

export default Modal;
