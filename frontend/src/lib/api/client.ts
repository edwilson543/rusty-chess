import { initClient } from "@ts-rest/core";

import { contract, GameSchema } from "./contract.ts";
import * as serializers from "./serializers.ts";
import * as types from "../types.ts";
import { Game } from "../types.ts";

export interface APIClient {
  startGame(): Promise<types.Game>;

  playMove({
    gameId,
    move,
  }: {
    gameId: number;
    move: types.Move;
  }): Promise<types.Game>;

  generateAndPlayNextMove({ gameId }: { gameId: number }): Promise<types.Game>;
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
          return serializers.parseGameSchemaToGame(
            JSON.parse(response.body) as GameSchema,
          );
        default:
          throw new Error(`Error starting game: ${response.status}`);
      }
    });
  }

  playMove({
    gameId,
    move,
  }: {
    gameId: number;
    move: types.Move;
  }): Promise<Game> {
    const promise = client.playMove({
      params: { gameId: gameId },
      body: {
        player: move.player,
        from_square: serializers.squareToString(move.fromSquare),
        to_square: serializers.squareToString(move.toSquare),
      },
    }) as Promise<Response>;
    return promise.then((response: Response) => {
      switch (response.status) {
        case 200:
          return serializers.parseGameSchemaToGame(
            JSON.parse(response.body) as GameSchema,
          );
        default:
          throw new Error(`Error playing move: ${response.status}`);
      }
    });
  }

  generateAndPlayNextMove({ gameId }: { gameId: number }): Promise<Game> {
    const promise = client.generateAndPlayNextMove({
      params: { gameId: gameId },
    }) as Promise<Response>;
    return promise.then((response: Response) => {
      switch (response.status) {
        case 200:
          return serializers.parseGameSchemaToGame(
            JSON.parse(response.body) as GameSchema,
          );
        default:
          throw new Error(`Error playing move: ${response.status}`);
      }
    });
  }
}
