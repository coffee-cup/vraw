import * as React from "react";

export interface State {
  code: string;
  count: number;
}

const initialState: State = {
  code: "",
  count: 0,
};

export type Action =
  | {
      type: "inc";
    }
  | {
      type: "dec";
    };

export interface Actions {
  inc: () => any;
}

export const StateContext = React.createContext<{
  state: State;
  actions: Actions;
}>({ state: initialState, actions: ({} as any) as Actions });

const reducer = (state: State, action: Action): State => {
  if (action.type === "inc") {
    return { ...state, count: state.count + 1 };
  }

  return state;
};

const setupActions = (dispatch: React.Dispatch<Action>): Actions => ({
  inc: () => dispatch({ type: "inc" }),
});

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
