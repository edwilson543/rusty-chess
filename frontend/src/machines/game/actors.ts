import { fromPromise } from "xstate";

import { getApiClient } from "../../lib/api/client.ts";

export const startGame = fromPromise(() => {
  const apiClient = getApiClient();
  return apiClient.startGame();
});
