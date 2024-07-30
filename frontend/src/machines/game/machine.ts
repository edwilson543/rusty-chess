import { setup, assign, assertEvent } from "xstate";

import * as types from "./types";

const GameMachine = setup({
  types: {
    context: {} as types.GameContextProps,
    events: {} as types.GameEventProps,
  },
  actions: {
    setActiveGame: assign({
      game: ({ event }) => {
        assertEvent(event, types.GameEvent.StartNewGame);
        return event.game;
      },
    }),
  },
}).createMachine({
  id: "game",
  initial: types.GameState.Idle,
  predictableActionArguments: true,
  states: {
    [types.GameState.Idle]: {
      on: {
        [types.GameEvent.StartNewGame]: {
          target: types.GameState.PlayerTurn,
        },
      },
    },
    [types.GameState.PlayerTurn]: {},
  },
});

export default GameMachine;
