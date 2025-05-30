import { fromPromise } from "xstate";

import { getApiClient } from "../../api/client.ts";
import * as chess from "../../domain/chess.ts";

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
  ({ input }: { input: { gameId: number; move: chess.Move } }) => {
    const apiClient = getApiClient();
    return apiClient.playMove({ gameId: input.gameId, move: input.move });
  },
);

export const generateAndPlayNextMove = fromPromise(
  ({ input }: { input: { gameId: number; engine: chess.Engine } }) => {
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
