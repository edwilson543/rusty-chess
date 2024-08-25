import { ChessboardSquare } from "./ChessboardSquare.tsx";
import * as types from "../lib/types.ts";

interface ChessboardProps {
  chessboard: types.Chessboard;
}

const RANK_WIDTH = 8;

export const Chessboard = (props: ChessboardProps) => {
  const ranks = Object.values(types.Rank)
    .filter((item) => typeof item === "number")
    .reverse();

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
  rank: types.Square[];
}

const ChessboardRank = (props: ChessboardRankProps) => {
  return (
    <div style={{ display: "flex", flexDirection: "row", width: "50%" }}>
      {props.rank.map((square: types.Square) => {
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

const sortPosition = (position: types.Square[]): types.Square[] => {
  const scoreSquare = (square: types.Square): number => {
    return square.rank * 8 + fileSortOrder[square.file];
  };

  return position.sort((a, b) => scoreSquare(a) - scoreSquare(b));
};
