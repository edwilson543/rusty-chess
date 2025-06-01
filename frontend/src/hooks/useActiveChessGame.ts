import { useSelector } from "@xstate/react";

import * as chess from "../domain/chess.ts";
import { GameMachineContext, GameState } from "../machines/game";

interface useActiveChessGameReturn {
  game: chess.Game | null;
  publicGameId: number | null;
  localPlayerColour: chess.Colour;
  isLocalPlayerTurn: boolean;
  legalMoves: chess.Move[];
  squareToMoveFrom: chess.Square;
  engine: chess.Engine;
}

export const useActiveChessGame = (): useActiveChessGameReturn => {
  const gameMachineRef = GameMachineContext.useActorRef();

  return useSelector(gameMachineRef, (state) => {
    return {
      game: state.context.game,
      publicGameId: state.context.publicGameId,
      localPlayerColour: state.context.localPlayerColour,
      isLocalPlayerTurn: state.value === GameState.LocalPlayerTurn,
      legalMoves: state.context.legalMoves,
      squareToMoveFrom: state.context.squareToMoveFrom,
      engine: state.context.engine,
    };
  });
};
