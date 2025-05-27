import { fromPromise } from "xstate";

import { getApiClient } from "../../lib/api/client.ts";
import * as types from "../../lib/types.ts";

export const loadGame = fromPromise(
  ({ input }: { input: { publicGameId: number } }) => {
    const apiClient = getApiClient();
    return apiClient.loadGame({ publicGameId: input.publicGameId });
  },
);

export const startGame = fromPromise(() => {
  const apiClient = getApiClient();
  return apiClient.startGame();
});

export const playMove = fromPromise(
  ({ input }: { input: { gameId: number; move: types.Move } }) => {
    const apiClient = getApiClient();
    return apiClient.playMove({ gameId: input.gameId, move: input.move });
  },
);

export const generateAndPlayNextMove = fromPromise(
  ({ input }: { input: { gameId: number; engine: types.Engine } }) => {
    const apiClient = getApiClient();
    return apiClient.generateAndPlayNextMove({
      gameId: input.gameId,
      engine: input.engine,
    });
  },
);

export const getLegalMoves = fromPromise(
  ({ input }: { input: { gameId: number } }) => {
    const apiClient = getApiClient();
    return apiClient.getLegalMoves({ gameId: input.gameId });
  },
);
