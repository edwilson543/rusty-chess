import { assertEvent, setup } from "xstate";

import { actions } from "./actions.ts";
import { startGame, playMove } from "./actors.ts";
import { guards } from "./guards.ts";
import * as machineTypes from "./types";
import * as types from "../../lib/types.ts";

const GameMachine = setup({
  types: {
    context: {} as machineTypes.GameContextProps,
    events: {} as machineTypes.GameEventProps,
  },
  actions: actions,
  actors: {
    startGame,
    playMove,
  },
  guards: guards,
}).createMachine({
  id: "game",
  context: {
    game: null,
    // TODO -> allow playing as either colour.
    localPlayerColour: types.Colour.White,
    squareToMoveFrom: null,
  },
  initial: machineTypes.GameState.Idle,
  predictableActionArguments: true,
  states: {
    [machineTypes.GameState.Idle]: {
      always: [
        { target: machineTypes.GameState.StartingGame, guard: "gameIsUnset" },
        { target: machineTypes.GameState.LocalPlayerTurn },
      ],
    },
    [machineTypes.GameState.StartingGame]: {
      invoke: {
        id: "startGame",
        src: "startGame",
        onDone: {
          actions: "setActiveGame",
          target: machineTypes.GameState.LocalPlayerTurn,
        },
        onError: {
          target: machineTypes.GameState.Unavailable,
        },
      },
    },
    [machineTypes.GameState.LocalPlayerTurn]: {
      on: {
        [machineTypes.GameEvent.SetSquareToMoveFrom]: {
          actions: "setSquareToMoveFrom",
        },
        [machineTypes.GameEvent.PlayMove]: {
          target: machineTypes.GameState.SubmittingMove,
        },
      },
    },
    [machineTypes.GameState.SubmittingMove]: {
      invoke: {
        id: "playMove",
        src: "playMove",
        input: ({ context, event }) => {
          assertEvent(event, machineTypes.GameEvent.PlayMove);
          return {
            gameId: context.game?.id,
            move: {
              fromSquare: event.fromSquare,
              toSquare: event.toSquare,
              player: context.localPlayerColour,
            },
          };
        },
        onDone: {
          actions: "setActiveGame",
          target: machineTypes.GameState.OpponentPlayerTurn,
        },
        onError: {
          target: machineTypes.GameState.LocalPlayerTurn,
        },
      },
    },
    [machineTypes.GameState.OpponentPlayerTurn]: {},

    [machineTypes.GameState.Unavailable]: { type: "final" },
  },
});

export default GameMachine;
