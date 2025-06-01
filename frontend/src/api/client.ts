import { initClient } from "@ts-rest/core";

import { contract, GameSchema, LegalMovesSchema } from "./contract.ts";
import * as serializers from "./serializers.ts";
import * as chess from "../domain/chess.ts";

export interface APIClient {
  loadGame({ publicGameId }: { publicGameId: number }): Promise<chess.Game>;

  startGame(): Promise<chess.Game>;

  playMove({
    gameId,
    move,
  }: {
    gameId: number;
    move: chess.Move;
  }): Promise<chess.Game>;

  generateAndPlayNextMove({
    gameId,
    engine,
  }: {
    gameId: number;
    engine: chess.Engine;
  }): Promise<chess.Game>;

  getLegalMoves({ gameId }: { gameId: number }): Promise<chess.Move[]>;
}

export const getApiClient = (): APIClient => {
  return new RestAPIClient();
};

const client = initClient(contract, {
  baseUrl: "http://127.0.0.1:8000/api",
  cors: false,
});

class RestAPIClient implements APIClient {
  loadGame({ publicGameId }: { publicGameId: number }): Promise<chess.Game> {
    const promise = client.getGameState({
      params: { publicGameId: publicGameId },
    }) as Promise<Response>;
    return promise.then((response: Response) => {
      switch (response.status) {
        case 200:
          return serializers.parseGameSchemaToGame(
            JSON.parse(response.body) as GameSchema,
          );

        case 404:
          throw new Error(`Game ${publicGameId} does not exist.`);
        default:
          throw new Error(`Unexpected error loading game ${publicGameId}.`);
      }
    });
  }

  startGame(): Promise<chess.Game> {
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
    move: chess.Move;
  }): Promise<chess.Game> {
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

  generateAndPlayNextMove({
    gameId,
    engine,
  }: {
    gameId: number;
    engine: chess.Engine;
  }): Promise<chess.Game> {
    const promise = client.generateAndPlayNextMove({
      params: { gameId: gameId },
      body: {
        engine: engine,
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

  getLegalMoves({ gameId }: { gameId: number }): Promise<chess.Move[]> {
    const promise = client.getLegalMoves({
      params: { gameId: gameId },
    }) as Promise<Response>;
    return promise.then((response: Response) => {
      switch (response.status) {
        case 200:
          return serializers.parseLegalMoves(
            JSON.parse(response.body) as LegalMovesSchema,
          );
        default:
          return [];
      }
    });
  }
}
