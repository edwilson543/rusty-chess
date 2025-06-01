import { Piece } from "./Piece.tsx";
import * as chess from "../domain/chess.ts";
import { useActiveChessGame, useChessGameActions } from "../hooks/";

interface ChessboardSquareProps {
  square: chess.Square;
}

export const ChessboardSquare = (props: ChessboardSquareProps) => {
  const { squareToMoveFrom, localPlayerColour, isLocalPlayerTurn, legalMoves } =
    useActiveChessGame();
  const { selectPiece, deselectPiece, playMove } = useChessGameActions();

  // Properties.
  const canSquareBeMovedTo =
    squareToMoveFrom &&
    legalMoves.some(
      (move) =>
        move.fromSquare.rank === squareToMoveFrom.rank &&
        move.fromSquare.file === squareToMoveFrom.file &&
        move.toSquare.rank === props.square.rank &&
        move.toSquare.file === props.square.file,
    );

  const isPieceSelected = props.square === squareToMoveFrom;
  const canPieceBeSelected =
    isLocalPlayerTurn && props.square.piece?.colour === localPlayerColour;

  // Interactions.
  const onSquareClick = () => {
    if (!canSquareBeMovedTo) {
      return null;
    }
    playMove(squareToMoveFrom, props.square);
  };

  const onPieceClick = () => {
    if (!canPieceBeSelected) {
      return null;
    }

    isPieceSelected ? deselectPiece() : selectPiece(props.square);
  };

  return (
    <div
      onClick={onSquareClick}
      data-testid={`square-${props.square.file}${props.square.rank}`}
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        width: "80px",
        height: "80px",
        border: "1px solid black",
        backgroundColor: getColourForSquare(props.square),
        // Highlight squares that can be moved to.
        boxShadow: canSquareBeMovedTo ? "inset 0 0 10px #f8a100" : "",
        cursor: canSquareBeMovedTo ? "pointer" : "default",
      }}
    >
      {props.square.piece && (
        <Piece
          piece={props.square.piece}
          isSelected={isPieceSelected}
          canBeSelected={canPieceBeSelected}
          onClick={onPieceClick}
        />
      )}
    </div>
  );
};

const getColourForSquare = (square: chess.Square): string => {
  const fileSortOrder = { A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8 };
  const alternate = fileSortOrder[square.file] % 2 === 1 ? 0 : 1;
  return square.rank % 2 === alternate ? "#aeeef2" : "#f2d4d0";
};
