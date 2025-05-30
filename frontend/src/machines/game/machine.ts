import { assertEvent, setup } from "xstate";

import { actions } from "./actions.ts";
import {
  generateAndPlayNextMove,
  getLegalMoves,
  loadGame,
  playMove,
  startGame,
} from "./actors.ts";
import { guards } from "./guards.ts";
import * as machineTypes from "./types";
import * as types from "../../domain/types.ts";

export const gameMachine = setup({
  types: {
    context: {} as machineTypes.GameContextProps,
    events: {} as machineTypes.GameEventProps,
    input: {} as machineTypes.GameInput,
  },
  actions: actions,
  actors: {
    loadGame,
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
  context: ({ input }) => ({
    game: null,
    publicGameId: input.publicGameId,
    legalMoves: [],
    localPlayerColour: types.Colour.White,
    squareToMoveFrom: null,
    engine: types.Engine.Random,
  }),
  predictableActionArguments: true,
  on: {
    [machineTypes.GameEvent.StartNewGame]: {
      target: `.${machineTypes.GameState.Initialising}`,
      actions: machineTypes.Action.ClearActiveGame,
    },
    [machineTypes.GameEvent.SetEngine]: {
      actions: [machineTypes.Action.SetEngine],
    },
  },
  initial: machineTypes.GameState.Initialising,
  states: {
    [machineTypes.GameState.Initialising]: {
      initial: machineTypes.GameState.StartingNewGame,
      states: {
        [machineTypes.GameState.StartingNewGame]: {
          always: {
            target: `#game.${machineTypes.GameState.Initialising}.${machineTypes.GameState.LoadingExistingGame}`,
            guard: machineTypes.Guard.PublicGameIdIsSet,
          },
          invoke: {
            id: "startGame",
            src: "startGame",
            onDone: {
              actions: [
                machineTypes.Action.SetActiveGame,
                machineTypes.Action.SetLocalPlayerToWhite,
              ],
              target: machineTypes.GameState.AssigningPlayerTurn,
            },
            onError: {
              target: `#game.${machineTypes.GameState.Unavailable}`,
            },
          },
        },
        [machineTypes.GameState.LoadingExistingGame]: {
          invoke: {
            id: "loadGame",
            src: "loadGame",
            input: ({ context }) => {
              return { publicGameId: context.publicGameId };
            },
            onDone: {
              actions: [
                machineTypes.Action.SetActiveGame,
                machineTypes.Action.SetLocalPlayerToWhite,
              ],
              target: machineTypes.GameState.AssigningPlayerTurn,
            },
            onError: {
              target: `#game.${machineTypes.GameState.Unavailable}`,
            },
          },
        },
        [machineTypes.GameState.AssigningPlayerTurn]: {
          always: [
            {
              target: `#game.${machineTypes.GameState.LocalPlayerTurn}`,
              guard: machineTypes.Guard.IsLocalPlayerTurn,
            },
            { target: `#game.${machineTypes.GameState.OpponentPlayerTurn}` },
          ],
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
      exit: [
        { type: machineTypes.Action.ClearSquareToPlayFrom },
        { type: machineTypes.Action.ClearLegalMoves },
      ],
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
          return { gameId: context.game?.id, engine: context.engine };
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
