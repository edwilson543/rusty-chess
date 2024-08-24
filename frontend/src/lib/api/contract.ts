import { initContract, ClientInferRequest } from "@ts-rest/core";
import { z } from "zod";

// Helpers.

const black = z.literal("B");
const white = z.literal("W");
const colour = z.union([black, white]);

const pieceType = z.enum(["P", "N", "B", "R", "Q", "K"]);

const chessboardSquare = z.object({
  colour: colour,
  piece_type: pieceType,
});

const chessboard = z.object({
  position: z.object({
    A1: chessboardSquare.nullable(),
    A2: chessboardSquare.nullable(),
    A3: chessboardSquare.nullable(),
    A4: chessboardSquare.nullable(),
    A5: chessboardSquare.nullable(),
    A6: chessboardSquare.nullable(),
    A7: chessboardSquare.nullable(),
    A8: chessboardSquare.nullable(),
    B1: chessboardSquare.nullable(),
    B2: chessboardSquare.nullable(),
    B3: chessboardSquare.nullable(),
    B4: chessboardSquare.nullable(),
    B5: chessboardSquare.nullable(),
    B6: chessboardSquare.nullable(),
    B7: chessboardSquare.nullable(),
    B8: chessboardSquare.nullable(),
    C1: chessboardSquare.nullable(),
    C2: chessboardSquare.nullable(),
    C3: chessboardSquare.nullable(),
    C4: chessboardSquare.nullable(),
    C5: chessboardSquare.nullable(),
    C6: chessboardSquare.nullable(),
    C7: chessboardSquare.nullable(),
    C8: chessboardSquare.nullable(),
    D1: chessboardSquare.nullable(),
    D2: chessboardSquare.nullable(),
    D3: chessboardSquare.nullable(),
    D4: chessboardSquare.nullable(),
    D5: chessboardSquare.nullable(),
    D6: chessboardSquare.nullable(),
    D7: chessboardSquare.nullable(),
    D8: chessboardSquare.nullable(),
    E1: chessboardSquare.nullable(),
    E2: chessboardSquare.nullable(),
    E3: chessboardSquare.nullable(),
    E4: chessboardSquare.nullable(),
    E5: chessboardSquare.nullable(),
    E6: chessboardSquare.nullable(),
    E7: chessboardSquare.nullable(),
    E8: chessboardSquare.nullable(),
    F1: chessboardSquare.nullable(),
    F2: chessboardSquare.nullable(),
    F3: chessboardSquare.nullable(),
    F4: chessboardSquare.nullable(),
    F5: chessboardSquare.nullable(),
    F6: chessboardSquare.nullable(),
    F7: chessboardSquare.nullable(),
    F8: chessboardSquare.nullable(),
    G1: chessboardSquare.nullable(),
    G2: chessboardSquare.nullable(),
    G3: chessboardSquare.nullable(),
    G4: chessboardSquare.nullable(),
    G5: chessboardSquare.nullable(),
    G6: chessboardSquare.nullable(),
    G7: chessboardSquare.nullable(),
    G8: chessboardSquare.nullable(),
    H1: chessboardSquare.nullable(),
    H2: chessboardSquare.nullable(),
    H3: chessboardSquare.nullable(),
    H4: chessboardSquare.nullable(),
    H5: chessboardSquare.nullable(),
    H6: chessboardSquare.nullable(),
    H7: chessboardSquare.nullable(),
    H8: chessboardSquare.nullable(),
  }),
});

const gameStatus = z.enum([
  "ToPlayWhite",
  "ToPlayBlack",
  "WonByWhite",
  "WonByBlack",
  "Drawn",
]);

const game = z.object({
  id: z.number(),
  status: gameStatus,
  chessboard: chessboard,
});

// Contract.

const c = initContract();

export const contract = c.router({
  startGame: {
    method: "POST",
    path: "/games/start/",
    responses: {
      201: game,
    },
  },
  getGameState: {
    method: "GET",
    path: "/games/:gameId/",
    pathParams: z.object({
      gameId: z.number(),
    }),
    responses: {
      200: game,
      404: z.object({}),
    },
  },
  playMove: {
    method: "POST",
    path: "/games/:gameId/play-move/",
    pathParams: z.object({
      gameId: z.number(),
    }),
    responses: {
      200: game,
      400: z.object({ error: z.string() }),
    },
  },
});

export type StartGameRequest = ClientInferRequest<typeof contract.startGame>;
export type GetGameState = ClientInferRequest<typeof contract.getGameState>;
export type PlayMove = ClientInferRequest<typeof contract.playMove>;
