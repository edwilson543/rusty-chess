import { useSelector } from "@xstate/react";

import { Chessboard } from "./Chessboard";
import { GameMachineContext } from "../context.ts";
// import { useGameWebSocket } from "../lib/websocket";

export const Game = () => {
  // const { sendMessage, messageHistory, connectionStatus } = useGameWebSocket();
  const gameMachineRef = GameMachineContext.useActorRef();
  const game = useSelector(gameMachineRef, (state) => state.context.game);

  return (
    <div style={{ width: "1200px", height: "1200px" }}>
      {game && <Chessboard chessboard={game.chessboard} />}
    </div>
  );
};
