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

  // Properties.
  const canSquareBeMovedTo =
    squareToMoveFrom && !(squareToMoveFrom === props.square);

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

  // Styling.
  const cursor = canSquareBeMovedTo ? "pointer" : "default";

  return (
    <div
      onClick={onSquareClick}
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        width: "80px",
        height: "80px",
        border: "1px solid black",
        backgroundColor: getColourForSquare(props.square),
        cursor: cursor,
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
