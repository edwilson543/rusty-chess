import { useSelector } from "@xstate/react";

import { Chessboard } from "./Chessboard";
import { GameMachineContext } from "../context.ts";

export const Game = () => {
  const gameMachineRef = GameMachineContext.useActorRef();
  const game = useSelector(gameMachineRef, (state) => state.context.game);

  return (
    <>
      <div style={{ width: "1200px", height: "1200px" }}>
        {game && <Chessboard chessboard={game.chessboard} />}
      </div>
    </>
  );
};
