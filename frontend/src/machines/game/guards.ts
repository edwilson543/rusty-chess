import * as machineTypes from "./types";

export const guards = {
  [machineTypes.Guard.GameIsUnset]: (
    context: machineTypes.GameContextProps,
  ) => {
    return !context.game;
  },
};