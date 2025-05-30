import * as machineTypes from "./types";

export const guards = {
  [machineTypes.Guard.PublicGameIdIsSet]: ({
    context,
  }: {
    context: machineTypes.GameContextProps;
  }) => {
    return !!context.publicGameId;
  },
  [machineTypes.Guard.IsLocalPlayerTurn]: ({
    context,
  }: {
    context: machineTypes.GameContextProps;
  }) => {
    return context.game?.toPlayColour === context.localPlayerColour;
  },
  [machineTypes.Guard.GameIsUnset]: ({
    context,
  }: {
    context: machineTypes.GameContextProps;
  }) => {
    return !context.game;
  },
  [machineTypes.Guard.GameIsComplete]: ({
    context,
  }: {
    context: machineTypes.GameContextProps;
  }) => {
    return !!context.game?.outcome;
  },
};
