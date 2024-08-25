import { fromPromise } from "xstate";

import { APIClient } from "../../lib/api/client.ts";
import { GameSchema } from "../../lib/api/contract.ts";
import { parseGameSchemaToGame } from "../../lib/api/deserializers.ts";

export const startGame = fromPromise(() => {
  const promise = APIClient.startGame() as Promise<Response>;
  return promise.then((response: Response) => {
    switch (response.status) {
      case 201:
        return parseGameSchemaToGame(JSON.parse(response.body) as GameSchema);
      default:
        throw new Error(`Error starting game: ${response.status}`);
    }
  });
});
