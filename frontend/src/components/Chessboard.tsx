import { ChessboardSquare } from "./ChessboardSquare.tsx";
import * as chess from "../domain/chess.ts";

interface ChessboardProps {
  chessboard: chess.Chessboard;
  localPlayerColour: chess.Colour;
}

const RANK_WIDTH = 8;

export const Chessboard = (props: ChessboardProps) => {
  const ranks = Object.values(chess.Rank).filter(
    (item) => typeof item === "number",
  );

  if (props.localPlayerColour === chess.Colour.White) {
    ranks.reverse();
  }

  const position = sortPosition(props.chessboard.position);
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
      }}
    >
      {ranks.map((rank) => {
        return (
          <ChessboardRank
            key={rank}
            rank={position.slice((rank - 1) * RANK_WIDTH, rank * RANK_WIDTH)}
          />
        );
      })}
    </div>
  );
};

interface ChessboardRankProps {
  rank: chess.Square[];
}

const ChessboardRank = (props: ChessboardRankProps) => {
  return (
    <div style={{ display: "flex", flexDirection: "row" }}>
      {props.rank.map((square: chess.Square) => {
        return (
          <ChessboardSquare
            key={`${square.file}${square.rank}`}
            square={square}
          />
        );
      })}
    </div>
  );
};

const fileSortOrder = { A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8 };

const sortPosition = (position: chess.Square[]): chess.Square[] => {
  const scoreSquare = (square: chess.Square): number => {
    return square.rank * 8 + fileSortOrder[square.file];
  };

  return position.sort((a, b) => scoreSquare(a) - scoreSquare(b));
};
