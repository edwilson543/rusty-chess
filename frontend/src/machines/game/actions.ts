import { ActionFunctionMap, ProvidedActor, assertEvent, assign } from "xstate";

import * as machineTypes from "./types.ts";

export const actions: ActionFunctionMap<
  machineTypes.GameContextProps,
  machineTypes.GameEventProps,
  ProvidedActor
> = {
  setActiveGame: assign({
    game: ({ event }) => {
      assertEvent(event, [
        machineTypes.GameEvent.GameStarted,
        machineTypes.GameEvent.MovePlayed,
      ]);
      return event.output;
    },
  }),
  setSquareToMoveFrom: assign({
    squareToMoveFrom: ({ event }) => {
      assertEvent(event, machineTypes.GameEvent.SetSquareToMoveFrom);
      return event.square;
    },
  }),
};
