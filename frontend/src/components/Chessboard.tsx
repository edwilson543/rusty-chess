import { Piece } from "./Piece.tsx";
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

const fileSortOrder = { A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8 };

const sortPosition = (position: types.Square[]): types.Square[] => {
  const scoreSquare = (square: types.Square): number => {
    return square.rank * 8 + fileSortOrder[square.file];
  };

  return position.sort((a, b) => scoreSquare(a) - scoreSquare(b));
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

interface ChessboardSquareProps {
  square: types.Square;
}

const ChessboardSquare = (props: ChessboardSquareProps) => {
  const alternate = fileSortOrder[props.square.file] % 2 === 1 ? 0 : 1;
  const colour = props.square.rank % 2 === alternate ? "#aeeef2" : "#f2d4d0";

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        flex: 1,
        alignItems: "center",
        justifyContent: "center",
        aspectRatio: "1 / 1",
        border: "1px solid black",
        backgroundColor: colour,
      }}
    >
      {props.square.piece && <Piece piece={props.square.piece} />}
    </div>
  );
};
