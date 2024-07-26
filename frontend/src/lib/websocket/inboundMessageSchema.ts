import { z } from "zod";

// Helpers.

const black = z.literal("B");
const white = z.literal("W");
const colour = z.union([black, white]);

const pieceType = z.union([
  z.literal("P"),
  z.literal("N"),
  z.literal("B"),
  z.literal("R"),
  z.literal("Q"),
  z.literal("K"),
]);

const chessboardSquare = z.object({
  colour: colour,
  pieceType: pieceType,
});

const chessboard = z.object({
  position: z.map(z.string(), chessboardSquare),
});

// Messages.

export const newGameMessage = z.object({
  name: z.literal("NewGame"),
  payload: z.object({
    id: z.number(),
    status: z.object({ ToPlay: white }),
    chessboard: chessboard,
  }),
});
