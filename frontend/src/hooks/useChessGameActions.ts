import { useMemo } from "react";

import * as chess from "../domain/chess";
import { GameMachineContext } from "../machines/game";
import { GameEvent } from "../machines/game/types.ts";

interface useChessGameActionsReturn {
  startNewGame: () => void;
  swapColours: () => void;
  selectEngine: (engine: chess.Engine) => void;
  selectPiece: (square: chess.Square) => void;
  deselectPiece: (square: chess.Square) => void;
  playMove: (fromSquare: chess.Square, toSquare: chess.Square) => void;
}

export const useChessGameActions = (): useChessGameActionsReturn => {
  const gameMachineRef = GameMachineContext.useActorRef();

  return useMemo(
    () => ({
      startNewGame: () => gameMachineRef.send({ type: GameEvent.StartNewGame }),
      swapColours: () => gameMachineRef.send({ type: GameEvent.SwapColours }),
      selectEngine: (engine) =>
        gameMachineRef.send({ type: GameEvent.SetEngine, engine }),
      selectPiece: (square) =>
        gameMachineRef.send({
          type: GameEvent.SetSquareToMoveFrom,
          square,
        }),
      deselectPiece: (square) =>
        gameMachineRef.send({
          type: GameEvent.SetSquareToMoveFrom,
          square,
        }),
      playMove: (fromSquare, toSquare) =>
        gameMachineRef.send({
          type: GameEvent.PlayMove,
          fromSquare: fromSquare,
          toSquare: toSquare,
        }),
    }),
    [gameMachineRef],
  );
};
