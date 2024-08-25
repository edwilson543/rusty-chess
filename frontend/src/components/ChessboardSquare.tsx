import { Piece } from "./Piece.tsx";
import * as types from "../lib/types.ts";

interface ChessboardSquareProps {
  square: types.Square;
}

export const ChessboardSquare = (props: ChessboardSquareProps) => {
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
        backgroundColor: getColourForSquare(props.square),
      }}
    >
      {props.square.piece && <Piece piece={props.square.piece} />}
    </div>
  );
};

const getColourForSquare = (square: types.Square): string => {
  const fileSortOrder = { A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8 };
  const alternate = fileSortOrder[square.file] % 2 === 1 ? 0 : 1;
  return square.rank % 2 === alternate ? "#aeeef2" : "#f2d4d0";
};
