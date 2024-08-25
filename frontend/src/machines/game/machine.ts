import { assertEvent, assign, setup } from "xstate";

import { startGame, playMove } from "./actors.ts";
import * as types from "./types";

const GameMachine = setup({
  types: {
    context: {} as types.GameContextProps,
    events: {} as types.GameEventProps,
  },
  actions: {
    setActiveGame: assign({
      game: ({ event }) => {
        assertEvent(event, [
          types.GameEvent.GameStarted,
          types.GameEvent.MovePlayed,
        ]);
        return event.output;
      },
    }),
    selectSquareToMoveFrom: assign({
      squareToMoveFrom: ({ event }) => {
        assertEvent(event, types.GameEvent.SelectSquareToMoveFrom);
        return event.square;
      },
    }),
  },
  actors: {
    startGame,
    playMove,
  },
  guards: {
    gameIsUnset: ({ context }) => {
      return !context.game;
    },
  },
}).createMachine({
  id: "game",
  context: { game: null, squareToMoveFrom: null },
  initial: types.GameState.Idle,
  predictableActionArguments: true,
  states: {
    [types.GameState.Idle]: {
      always: [
        { target: types.GameState.StartingGame, guard: "gameIsUnset" },
        { target: types.GameState.LocalPlayerTurn },
      ],
    },
    [types.GameState.StartingGame]: {
      invoke: {
        id: "startGame",
        src: "startGame",
        onDone: {
          actions: "setActiveGame",
          target: types.GameState.LocalPlayerTurn,
        },
        onError: {
          target: types.GameState.Unavailable,
        },
      },
    },
    [types.GameState.LocalPlayerTurn]: {
      on: {
        [types.GameEvent.SelectSquareToMoveFrom]: {
          actions: "selectSquareToMoveFrom",
        },
        [types.GameEvent.PlayMove]: {
          target: types.GameState.SubmittingMove,
        },
      },
    },
    [types.GameState.SubmittingMove]: {
      invoke: {
        id: "playMove",
        src: "playMove",
        input: ({ context, event }) => {
          assertEvent(event, types.GameEvent.PlayMove);
          return {
            gameId: context.game?.id,
            move: {
              fromSquare: event.fromSquare,
              toSquare: event.toSquare,
              player: context.game?.localPlayer,
            },
          };
        },
        onDone: {
          actions: "setActiveGame",
          target: types.GameState.OpponentPlayerTurn,
        },
        onError: {
          target: types.GameState.LocalPlayerTurn,
        },
      },
    },
    [types.GameState.OpponentPlayerTurn]: {},

    [types.GameState.Unavailable]: { type: "final" },
  },
});

export default GameMachine;
