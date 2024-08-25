import { initClient } from "@ts-rest/core";

import { contract, GameSchema } from "./contract.ts";
import { parseGameSchemaToGame } from "./deserializers.ts";
import * as types from "../types.ts";
import { Game } from "../types.ts";

export interface APIClient {
  startGame(): Promise<types.Game>;
}

export const getApiClient = (): APIClient => {
  return new RestAPIClient();
};

const client = initClient(contract, {
  baseUrl: "http://127.0.0.1:8000/api",
  cors: false,
});

class RestAPIClient implements APIClient {
  startGame(): Promise<Game> {
    const promise = client.startGame() as Promise<Response>;
    return promise.then((response: Response) => {
      switch (response.status) {
        case 201:
          return parseGameSchemaToGame(JSON.parse(response.body) as GameSchema);
        default:
          throw new Error(`Error starting game: ${response.status}`);
      }
    });
  }
}
