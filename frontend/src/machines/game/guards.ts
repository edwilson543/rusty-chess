import { GameContextProps } from "./types.ts";

export const guards = {
  gameIsUnset: (context: GameContextProps) => {
    return !context.game;
  },
};
