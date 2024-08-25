import { useSelector } from "@xstate/react";

import { Chessboard } from "./Chessboard";
import { FixedMoveButton } from "./FixedMoveButton.tsx";
import { GameMachineContext } from "../context.ts";

export const Game = () => {
  const gameMachineRef = GameMachineContext.useActorRef();
  const game = useSelector(gameMachineRef, (state) => state.context.game);

  return (
    <>
      <FixedMoveButton />
      <div style={{ width: "1200px", height: "1200px" }}>
        {game && <Chessboard chessboard={game.chessboard} />}
      </div>
    </>
  );
};
