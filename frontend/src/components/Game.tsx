import {useSelector} from "@xstate/react";

import { GameMachineContext } from "../context.ts";
import { useGameWebSocket } from "../lib/websocket";


export const Game = () => {
  const { sendMessage, messageHistory, connectionStatus } = useGameWebSocket();
  const gameMachineRef = GameMachineContext.useActorRef();

  const game = useSelector(gameMachineRef, (state) => state.context.game);

  // TODO -> send new game message into game machine.


  return (
    <>
      <div>
        <p>I am a chess game.</p>
      </div>

      <div>Connection status: {connectionStatus}</div>
      <div className="card">
        <button onClick={() => sendMessage("Hello server")}>
          Send message
        </button>
      </div>
      <div>
        {messageHistory
          .slice()
          .reverse()
          .map((message, index) => {
            return (
              <>
                <code key={index}>{message.data}</code>
                <br />
              </>
            );
          })}
      </div>
    </>
  );
};
