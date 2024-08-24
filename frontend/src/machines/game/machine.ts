import { assertEvent, assign, setup } from "xstate";

import { startGame } from "./actors.ts";
import * as types from "./types";

const GameMachine = setup({
  types: {
    context: {} as types.GameContextProps,
    events: {} as types.GameEventProps,
  },
  actions: {
    setActiveGame: assign({
      game: ({ event }) => {
        assertEvent(event, [types.GameEvent.StartGame]);
        return event.output;
      },
    }),
  },
  actors: {
    startGame,
  },
  guards: {
    gameIsUnset: ({ context }) => {
      return !context.game;
    },
  },
}).createMachine({
  id: "game",
  context: { game: null },
  initial: types.GameState.Idle,
  predictableActionArguments: true,
  states: {
    [types.GameState.Idle]: {
      always: [
        { target: types.GameState.StartingGame, guard: "gameIsUnset" },
        { target: types.GameState.PlayerTurn },
      ],
    },
    [types.GameState.StartingGame]: {
      invoke: {
        id: "startGame",
        src: "startGame",
        onDone: {
          actions: "setActiveGame",
          target: types.GameState.PlayerTurn,
        },
        onError: {
          target: types.GameState.Unavailable,
        },
      },
    },
    [types.GameState.PlayerTurn]: {},
    [types.GameState.Unavailable]: { type: "final" },
  },
});

export default GameMachine;
