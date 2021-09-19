export const useReducer = (
    reducer: (state: {}, action: { type: string; data?: {} }) => {},
    initialState = {},
    middlewares = []
) => {
    let state = initialState;

    const dispatch = (action) => {
        // console.log("state before", state, action);
        state = reducer(state, action);
        // console.log("state after", state, action);
    };

    return [state, dispatch];
};
