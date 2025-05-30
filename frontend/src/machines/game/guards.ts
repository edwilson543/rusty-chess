import * as types from "./types";

export const guards = {
  [types.Guard.PublicGameIdIsSet]: ({
    context,
  }: {
    context: types.GameContextProps;
  }) => {
    return !!context.publicGameId;
  },
  [types.Guard.IsLocalPlayerTurn]: ({
    context,
  }: {
    context: types.GameContextProps;
  }) => {
    return context.game?.toPlayColour === context.localPlayerColour;
  },
  [types.Guard.GameIsUnset]: ({
    context,
  }: {
    context: types.GameContextProps;
  }) => {
    return !context.game;
  },
  [types.Guard.GameIsComplete]: ({
    context,
  }: {
    context: types.GameContextProps;
  }) => {
    return !!context.game?.outcome;
  },
};
