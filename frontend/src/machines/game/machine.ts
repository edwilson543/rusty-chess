import { assertEvent, setup } from "xstate";

import { actions } from "./actions.ts";
import {
  startGame,
  playMove,
  generateAndPlayNextMove,
  getLegalMoves,
} from "./actors.ts";
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
    generateAndPlayNextMove,
    getLegalMoves,
  },
  delays: {
    opponentThinkingTimeMs: 500,
  },
  guards: guards,
}).createMachine({
  id: "game",
  context: {
    game: null,
    legalMoves: [],
    localPlayerColour: types.Colour.White,
    squareToMoveFrom: null,
  },
  initial: machineTypes.GameState.Idle,
  predictableActionArguments: true,
  on: {
    [machineTypes.GameEvent.StartNewGame]: {
      target: `.${machineTypes.GameState.StartingGame}`,
    },
  },
  states: {
    [machineTypes.GameState.Idle]: {
      always: [
        {
          target: machineTypes.GameState.StartingGame,
          guard: machineTypes.Guard.GameIsUnset,
        },
        { target: machineTypes.GameState.LocalPlayerTurn },
      ],
    },
    [machineTypes.GameState.StartingGame]: {
      invoke: {
        id: "startGame",
        src: "startGame",
        onDone: {
          actions: [
            machineTypes.Action.SetActiveGame,
            machineTypes.Action.SetLocalPlayerToWhite,
          ],
          target: machineTypes.GameState.LocalPlayerTurn,
        },
        onError: {
          target: machineTypes.GameState.Unavailable,
        },
      },
    },
    [machineTypes.GameState.LocalPlayerTurn]: {
      invoke: {
        id: "getLegalMoves",
        src: "getLegalMoves",
        input: ({ context }) => {
          return { gameId: context.game?.id };
        },
        onDone: {
          actions: [machineTypes.Action.SetLegalMoves],
          target: machineTypes.GameState.LocalPlayerTurn,
        },
      },
      always: {
        target: machineTypes.GameState.GameComplete,
        guard: machineTypes.Guard.GameIsComplete,
      },
      on: {
        [machineTypes.GameEvent.SetSquareToMoveFrom]: {
          actions: machineTypes.Action.SetSquareToMoveFrom,
        },
        [machineTypes.GameEvent.PlayMove]: {
          target: machineTypes.GameState.SubmittingLocalPlayerMove,
        },
        [machineTypes.GameEvent.SwapColours]: {
          actions: machineTypes.Action.SwapColours,
          target: machineTypes.GameState.OpponentPlayerTurn,
        },
      },
      exit: [{ type: machineTypes.Action.ClearLegalMoves }],
    },
    [machineTypes.GameState.SubmittingLocalPlayerMove]: {
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
          actions: machineTypes.Action.SetActiveGame,
          target: machineTypes.GameState.OpponentPlayerTurn,
        },
        onError: {
          target: machineTypes.GameState.LocalPlayerTurn,
        },
      },
    },
    [machineTypes.GameState.OpponentPlayerTurn]: {
      always: {
        target: machineTypes.GameState.GameComplete,
        guard: machineTypes.Guard.GameIsComplete,
      },
      after: {
        opponentThinkingTimeMs:
          machineTypes.GameState.SubmittingOpponentPlayerMove,
      },
    },
    [machineTypes.GameState.SubmittingOpponentPlayerMove]: {
      invoke: {
        id: "generateAndPlayNextMove",
        src: "generateAndPlayNextMove",
        input: ({ context }) => {
          return { gameId: context.game?.id };
        },
        onDone: {
          actions: machineTypes.Action.SetActiveGame,
          target: machineTypes.GameState.LocalPlayerTurn,
        },
        onError: {
          target: machineTypes.GameState.Unavailable,
        },
      },
    },
    [machineTypes.GameState.GameComplete]: {},
    [machineTypes.GameState.Unavailable]: {},
  },
});

export default GameMachine;
