import { ActionFunctionMap, ProvidedActor, assertEvent, assign } from "xstate";

import * as types from "./types.ts";
import * as chess from "../../domain/chess.ts";

export const actions: ActionFunctionMap<
  types.GameContextProps,
  types.GameEventProps,
  ProvidedActor
> = {
  [types.Action.SetActiveGame]: assign(({ event }) => {
    assertEvent(event, [
      types.GameEvent.GameLoaded,
      types.GameEvent.GameStarted,
      types.GameEvent.MovePlayed,
      types.GameEvent.MoveGeneratedAndPlayed,
    ]);
    return {
      game: event.output,
      publicGameId: event.output.id,
    };
  }),
  [types.Action.ClearActiveGame]: assign({
    game: null,
    publicGameId: null,
  }),
  [types.Action.SwapColours]: assign({
    localPlayerColour: ({ context, event }) => {
      assertEvent(event, types.GameEvent.SwapColours);
      const swapper = {
        [chess.Colour.White]: chess.Colour.Black,
        [chess.Colour.Black]: chess.Colour.White,
      };
      return swapper[context.localPlayerColour];
    },
  }),
  [types.Action.SetEngine]: assign({
    engine: ({ event }) => {
      assertEvent(event, types.GameEvent.SetEngine);
      return event.engine;
    },
  }),
  // Square to play from.
  [types.Action.SetSquareToMoveFrom]: assign({
    squareToMoveFrom: ({ event }) => {
      assertEvent(event, types.GameEvent.SetSquareToMoveFrom);
      return event.square;
    },
  }),
  [types.Action.ClearSquareToPlayFrom]: assign({
    squareToMoveFrom: () => {
      return null;
    },
  }),
  // Legal moves.
  [types.Action.SetLegalMoves]: assign({
    legalMoves: ({ event }) => {
      assertEvent(event, types.GameEvent.SetLegalMoves);
      return event.output;
    },
  }),
  [types.Action.ClearLegalMoves]: assign({
    legalMoves: () => {
      return [];
    },
  }),
};
