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
import * as types from "./types";
import * as chess from "../../domain/chess.ts";

export const gameMachine = setup({
  types: {
    context: {} as types.GameContextProps,
    events: {} as types.GameEventProps,
    input: {} as types.GameInput,
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
    localPlayerColour: chess.Colour.White,
    squareToMoveFrom: null,
    engine: chess.Engine.Random,
  }),
  predictableActionArguments: true,
  on: {
    [types.GameEvent.StartNewGame]: {
      target: `.${types.GameState.Initialising}`,
      actions: types.Action.ClearActiveGame,
    },
    [types.GameEvent.SetEngine]: {
      actions: [types.Action.SetEngine],
    },
  },
  initial: types.GameState.Initialising,
  states: {
    [types.GameState.Initialising]: {
      initial: types.GameState.StartingNewGame,
      states: {
        [types.GameState.StartingNewGame]: {
          always: {
            target: `#game.${types.GameState.Initialising}.${types.GameState.LoadingExistingGame}`,
            guard: types.Guard.PublicGameIdIsSet,
          },
          invoke: {
            id: "startGame",
            src: "startGame",
            onDone: {
              actions: [
                types.Action.SetActiveGame,
                types.Action.SetLocalPlayerToWhite,
              ],
              target: types.GameState.AssigningPlayerTurn,
            },
            onError: {
              target: `#game.${types.GameState.Unavailable}`,
            },
          },
        },
        [types.GameState.LoadingExistingGame]: {
          invoke: {
            id: "loadGame",
            src: "loadGame",
            input: ({ context }) => {
              return { publicGameId: context.publicGameId };
            },
            onDone: {
              actions: [
                types.Action.SetActiveGame,
                types.Action.SetLocalPlayerToWhite,
              ],
              target: types.GameState.AssigningPlayerTurn,
            },
            onError: {
              target: `#game.${types.GameState.Unavailable}`,
            },
          },
        },
        [types.GameState.AssigningPlayerTurn]: {
          always: [
            {
              target: `#game.${types.GameState.LocalPlayerTurn}`,
              guard: types.Guard.IsLocalPlayerTurn,
            },
            { target: `#game.${types.GameState.OpponentPlayerTurn}` },
          ],
        },
      },
    },
    [types.GameState.LocalPlayerTurn]: {
      invoke: {
        id: "getLegalMoves",
        src: "getLegalMoves",
        input: ({ context }) => {
          return { gameId: context.game?.id };
        },
        onDone: {
          actions: [types.Action.SetLegalMoves],
          target: types.GameState.LocalPlayerTurn,
        },
      },
      always: {
        target: types.GameState.GameComplete,
        guard: types.Guard.GameIsComplete,
      },
      on: {
        [types.GameEvent.SetSquareToMoveFrom]: {
          actions: types.Action.SetSquareToMoveFrom,
        },
        [types.GameEvent.PlayMove]: {
          target: types.GameState.SubmittingLocalPlayerMove,
        },
        [types.GameEvent.SwapColours]: {
          actions: types.Action.SwapColours,
          target: types.GameState.OpponentPlayerTurn,
        },
      },
      exit: [
        { type: types.Action.ClearSquareToPlayFrom },
        { type: types.Action.ClearLegalMoves },
      ],
    },
    [types.GameState.SubmittingLocalPlayerMove]: {
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
              player: context.localPlayerColour,
            },
          };
        },
        onDone: {
          actions: types.Action.SetActiveGame,
          target: types.GameState.OpponentPlayerTurn,
        },
        onError: {
          target: types.GameState.LocalPlayerTurn,
        },
      },
    },
    [types.GameState.OpponentPlayerTurn]: {
      always: {
        target: types.GameState.GameComplete,
        guard: types.Guard.GameIsComplete,
      },
      after: {
        opponentThinkingTimeMs: types.GameState.SubmittingOpponentPlayerMove,
      },
    },
    [types.GameState.SubmittingOpponentPlayerMove]: {
      invoke: {
        id: "generateAndPlayNextMove",
        src: "generateAndPlayNextMove",
        input: ({ context }) => {
          return { gameId: context.game?.id, engine: context.engine };
        },
        onDone: {
          actions: types.Action.SetActiveGame,
          target: types.GameState.LocalPlayerTurn,
        },
        onError: {
          target: types.GameState.Unavailable,
        },
      },
    },
    [types.GameState.GameComplete]: {},
    [types.GameState.Unavailable]: {},
  },
});
