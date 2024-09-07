import { ActionFunctionMap, ProvidedActor, assertEvent, assign } from "xstate";

import * as machineTypes from "./types.ts";
import * as types from "../../lib/types.ts";

export const actions: ActionFunctionMap<
  machineTypes.GameContextProps,
  machineTypes.GameEventProps,
  ProvidedActor
> = {
  [machineTypes.Action.SetActiveGame]: assign({
    game: ({ event }) => {
      assertEvent(event, [
        machineTypes.GameEvent.GameStarted,
        machineTypes.GameEvent.MovePlayed,
        machineTypes.GameEvent.MoveGeneratedAndPlayed,
      ]);
      return event.output;
    },
  }),
  [machineTypes.Action.SwapColours]: assign({
    localPlayerColour: ({ context, event }) => {
      assertEvent(event, machineTypes.GameEvent.SwapColours);
      const swapper = {
        [types.Colour.White]: types.Colour.Black,
        [types.Colour.Black]: types.Colour.White,
      };
      return swapper[context.localPlayerColour];
    },
  }),
  [machineTypes.Action.SetLocalPlayerToWhite]: assign({
    localPlayerColour: () => {
      return types.Colour.White;
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
