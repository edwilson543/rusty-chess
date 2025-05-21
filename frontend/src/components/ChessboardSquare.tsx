import { useSelector } from "@xstate/react";

import { Piece } from "./Piece.tsx";
import { GameMachineContext } from "../context.ts";
import * as types from "../lib/types.ts";
import { GameState, GameEvent } from "../machines/game/types.ts";

interface ChessboardSquareProps {
  square: types.Square;
}

export const ChessboardSquare = (props: ChessboardSquareProps) => {
  // State.
  const gameMachineRef = GameMachineContext.useActorRef();
  const squareToMoveFrom = useSelector(
    gameMachineRef,
    (state) => state.context.squareToMoveFrom,
  );
  const localPlayerColour = useSelector(
    gameMachineRef,
    (state) => state.context.localPlayerColour,
  );
  const isLocalPlayerTurn = useSelector(gameMachineRef, (state) =>
    state.matches(GameState.LocalPlayerTurn),
  );
  const legalMoves = useSelector(
    gameMachineRef,
    (state) => state.context.legalMoves,
  );

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
    gameMachineRef.send({
      type: GameEvent.PlayMove,
      fromSquare: squareToMoveFrom,
      toSquare: props.square,
    });
  };

  const onPieceClick = () => {
    if (!canPieceBeSelected) {
      return null;
    }
    gameMachineRef.send({
      type: GameEvent.SetSquareToMoveFrom,
      square: isPieceSelected ? null : props.square,
    });
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

const getColourForSquare = (square: types.Square): string => {
  const fileSortOrder = { A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8 };
  const alternate = fileSortOrder[square.file] % 2 === 1 ? 0 : 1;
  return square.rank % 2 === alternate ? "#aeeef2" : "#f2d4d0";
};
