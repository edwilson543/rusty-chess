import { GameSchema, LegalMovesSchema } from "./contract.ts";
import * as chess from "../domain/chess.ts";

export const parseGameSchemaToGame = (game: GameSchema): chess.Game => {
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

export const parseLegalMoves = (legalMoves: LegalMovesSchema): chess.Move[] => {
  const parsedMoves: chess.Move[] = [];
  legalMoves.forEach((legalMove) => {
    parsedMoves.push({
      fromSquare: positionKeyToEmptySquare(legalMove.from_square),
      toSquare: positionKeyToEmptySquare(legalMove.to_square),
      player: legalMove.player as chess.Colour,
    });
  });
  return parsedMoves;
};

export const squareToString = (square: chess.Square): string => {
  return `${square.file}${square.rank}`;
};

// Helpers.

const positionKeyToEmptySquare = (key: string): chess.Square => {
  const rank = positionKeyToRank(key);
  const file = positionKeyToFile(key);
  return {
    rank: rank,
    file: file,
    piece: null,
  };
};

const positionKeyToRank = (key: string): chess.Rank => {
  const rank = parseInt(key[1]) as unknown as chess.Rank;
  if (Object.values(chess.Rank).includes(rank)) {
    return rank as unknown as chess.Rank;
  }
  throw new Error(`${rank} is not a recognised Rank.`);
};

const positionKeyToFile = (key: string): chess.File => {
  const file = key[0] as unknown as chess.File;
  if (Object.values(chess.File).includes(file)) {
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
}): chess.Piece | null => {
  if (!Object.values(chess.Colour).includes(colour)) {
    throw new Error(`${colour} is not a recognised Colour.`);
  }

  if (!Object.values(chess.PieceType).includes(piece_type)) {
    throw new Error(`${piece_type} is not a recognised PieceType.`);
  }

  return {
    colour: colour as unknown as chess.Colour,
    pieceType: piece_type as unknown as chess.PieceType,
  };
};

const extractToPlayColour = (game: GameSchema): chess.Colour | null => {
  switch (game.status) {
    case "ToPlayWhite":
      return chess.Colour.White;
    case "ToPlayBlack":
      return chess.Colour.Black;
    default:
      return null;
  }
};

const extractGameOutcome = (game: GameSchema): chess.GameOutcome | null => {
  switch (game.status) {
    case "WonByWhite":
      return chess.GameOutcome.WonByWhite;
    case "WonByBlack":
      return chess.GameOutcome.WonByBlack;
    case "Drawn":
      return chess.GameOutcome.Drawn;
    default:
      return null;
  }
};
