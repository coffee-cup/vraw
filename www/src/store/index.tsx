import * as React from "react";

export interface State {
  code: string;
  count: number;
}

const initialCode = `
shape main() {
  svg(value: "hello")
}
`.trimLeft();

const initialState: State = {
  code: initialCode,
  count: 0,
};

export type Action =
  | {
      type: "inc";
    }
  | {
      type: "dec";
    }
  | {
      type: "CHANGE_CODE";
      value: string;
    };

export interface Actions {
  inc: () => any;
  changeCode: (code: string) => any;
}

export const StateContext = React.createContext<{
  state: State;
  actions: Actions;
}>({ state: initialState, actions: ({} as any) as Actions });

const setupActions = (dispatch: React.Dispatch<Action>): Actions => ({
  inc: () => dispatch({ type: "inc" }),
  changeCode: (code: string) => dispatch({ type: "CHANGE_CODE", value: code }),
});

const reducer = (state: State, action: Action): State => {
  if (action.type === "inc") {
    return { ...state, count: state.count + 1 };
  } else if (action.type === "CHANGE_CODE") {
    return { ...state, code: action.value };
  }

  return state;
};

export const useStore = (): { state: State; actions: Actions } => {
  const { state, actions } = React.useContext(StateContext);
  return { state, actions };
};

export const StateProvider = (props: { children: React.ReactNode }) => {
  const [state, dispatch] = React.useReducer(reducer, initialState);
  const actions = setupActions(dispatch);

  return (
    <StateContext.Provider value={{ state, actions }}>
      {props.children}
    </StateContext.Provider>
  );
};
