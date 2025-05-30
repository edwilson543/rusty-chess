import {
  ActionFunctionMap,
  ProvidedActor,
  assertEvent,
  assign,
  enqueueActions,
} from "xstate";

import * as machineTypes from "./types.ts";
import * as chess from "../../domain/chess.ts";

export const actions: ActionFunctionMap<
  machineTypes.GameContextProps,
  machineTypes.GameEventProps,
  ProvidedActor
> = {
  [machineTypes.Action.SetActiveGame]: enqueueActions(({ enqueue, event }) => {
    assertEvent(event, [
      machineTypes.GameEvent.GameLoaded,
      machineTypes.GameEvent.GameStarted,
      machineTypes.GameEvent.MovePlayed,
      machineTypes.GameEvent.MoveGeneratedAndPlayed,
    ]);

    enqueue.assign({
      game: event.output,
      publicGameId: event.output.id,
    });

    enqueue(() => {
      const url = new URL(window.location.href);
      const currentPublicGameId = event.output.id.toString();

      if (url.searchParams.get("gameId") !== currentPublicGameId) {
        url.searchParams.set("gameId", currentPublicGameId);
        window.history.pushState({}, "", url);
      }
    });
  }),
  [machineTypes.Action.ClearActiveGame]: assign({
    game: null,
    publicGameId: null,
  }),
  [machineTypes.Action.SwapColours]: assign({
    localPlayerColour: ({ context, event }) => {
      assertEvent(event, machineTypes.GameEvent.SwapColours);
      const swapper = {
        [chess.Colour.White]: chess.Colour.Black,
        [chess.Colour.Black]: chess.Colour.White,
      };
      return swapper[context.localPlayerColour];
    },
  }),
  [machineTypes.Action.SetLocalPlayerToWhite]: assign({
    localPlayerColour: () => {
      return chess.Colour.White;
    },
  }),
  [machineTypes.Action.SetEngine]: assign({
    engine: ({ event }) => {
      assertEvent(event, machineTypes.GameEvent.SetEngine);
      return event.engine;
    },
  }),
  // Square to play from.
  [machineTypes.Action.SetSquareToMoveFrom]: assign({
    squareToMoveFrom: ({ event }) => {
      assertEvent(event, machineTypes.GameEvent.SetSquareToMoveFrom);
      return event.square;
    },
  }),
  [machineTypes.Action.ClearSquareToPlayFrom]: assign({
    squareToMoveFrom: () => {
      return null;
    },
  }),
  // Legal moves.
  [machineTypes.Action.SetLegalMoves]: assign({
    legalMoves: ({ event }) => {
      assertEvent(event, machineTypes.GameEvent.SetLegalMoves);
      return event.output;
    },
  }),
  [machineTypes.Action.ClearLegalMoves]: assign({
    legalMoves: () => {
      return [];
    },
  }),
};
