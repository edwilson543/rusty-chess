import { initContract } from "@ts-rest/core";
import { z } from "zod";

// Helpers.

const black = z.literal("Black");
const white = z.literal("White");
const colour = z.union([black, white]);

const pieceType = z.enum([
  "Pawn",
  "Knight",
  "Black",
  "Rook",
  "Queen",
  "Knight",
]);

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

export const gameStatus = z.enum([
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

const move = z.object({
  player: colour,
  from_square: z.string(),
  to_square: z.string(),
});
const legal_moves = z.array(move);

// Contract.

const c = initContract();

export const contract = c.router({
  startGame: {
    method: "POST",
    path: "/games/start/",
    // TODO - allow to select either colour...
    responses: {
      201: game,
    },
  },
  playMove: {
    method: "POST",
    path: "/games/:gameId/play-move/",
    pathParams: z.object({
      gameId: z.number(),
    }),
    body: move,
    responses: {
      200: game,
      400: z.object({ error: z.string() }),
    },
  },
  generateAndPlayNextMove: {
    method: "POST",
    path: "/games/:gameId/generate-and-play-next-move/",
    pathParams: z.object({
      gameId: z.number(),
    }),
    body: z.object({ engine: z.enum(["Random", "Minimax", "MCTS"]) }),
    responses: {
      200: game,
      400: z.object({ error: z.string() }),
    },
  },
  getLegalMoves: {
    method: "GET",
    path: "/games/:gameId/get-legal-moves/",
    pathParams: z.object({
      gameId: z.number(),
    }),
    responses: {
      200: legal_moves,
      400: z.object({}),
      404: z.object({}),
    },
  },
  getGameState: {
    method: "GET",
    path: "/games/:publicGameId/",
    pathParams: z.object({
      publicGameId: z.number(),
    }),
    responses: {
      200: game,
      404: z.object({}),
    },
  },
});

export type GameSchema = z.infer<typeof game>;
export type LegalMovesSchema = z.infer<typeof legal_moves>;
