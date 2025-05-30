import { GameSchema, LegalMovesSchema } from "./contract.ts";
import * as types from "../lib/types.ts";

export const parseGameSchemaToGame = (game: GameSchema): types.Game => {
  const chessboardPosition = [];

  for (const [key, value] of Object.entries(game.chessboard.position)) {
    const rank = positionKeyToRank(key);
    const file = positionKeyToFile(key);

    let piece = null;
    if (value !== null) {
      piece = positionValueToPiece(value);
    }

    chessboardPosition.push({
      rank: rank,
      file: file,
      piece: piece,
    });
  }

  return {
    id: game.id,
    chessboard: { position: chessboardPosition },
    toPlayColour: extractToPlayColour(game),
    outcome: extractGameOutcome(game),
  };
};

export const parseLegalMoves = (legalMoves: LegalMovesSchema): types.Move[] => {
  const parsedMoves: types.Move[] = [];
  legalMoves.forEach((legalMove) => {
    parsedMoves.push({
      fromSquare: positionKeyToEmptySquare(legalMove.from_square),
      toSquare: positionKeyToEmptySquare(legalMove.to_square),
      player: legalMove.player as types.Colour,
    });
  });
  return parsedMoves;
};

export const squareToString = (square: types.Square): string => {
  return `${square.file}${square.rank}`;
};

// Helpers.

const positionKeyToEmptySquare = (key: string): types.Square => {
  const rank = positionKeyToRank(key);
  const file = positionKeyToFile(key);
  return {
    rank: rank,
    file: file,
    piece: null,
  };
};

const positionKeyToRank = (key: string): types.Rank => {
  const rank = parseInt(key[1]) as unknown as types.Rank;
  if (Object.values(types.Rank).includes(rank)) {
    return rank as unknown as types.Rank;
  }
  throw new Error(`${rank} is not a recognised Rank.`);
};

const positionKeyToFile = (key: string): types.File => {
  const file = key[0] as unknown as types.File;
  if (Object.values(types.File).includes(file)) {
    return file;
  }
  throw new Error(`${file} is not a recognised File.`);
};

const positionValueToPiece = ({
  colour,
  piece_type,
}: {
  colour: string;
  piece_type: string;
}): types.Piece | null => {
  if (!Object.values(types.Colour).includes(colour)) {
    throw new Error(`${colour} is not a recognised Colour.`);
  }

  if (!Object.values(types.PieceType).includes(piece_type)) {
    throw new Error(`${piece_type} is not a recognised PieceType.`);
  }

  return {
    colour: colour as unknown as types.Colour,
    pieceType: piece_type as unknown as types.PieceType,
  };
};

const extractToPlayColour = (game: GameSchema): types.Colour | null => {
  switch (game.status) {
    case "ToPlayWhite":
      return types.Colour.White;
    case "ToPlayBlack":
      return types.Colour.Black;
    default:
      return null;
  }
};

const extractGameOutcome = (game: GameSchema): types.GameOutcome | null => {
  switch (game.status) {
    case "WonByWhite":
      return types.GameOutcome.WonByWhite;
    case "WonByBlack":
      return types.GameOutcome.WonByBlack;
    case "Drawn":
      return types.GameOutcome.Drawn;
    default:
      return null;
  }
};
