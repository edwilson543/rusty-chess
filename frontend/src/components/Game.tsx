import { useGameWebSocket } from "../lib/websocket";

export const Game = () => {
  const { sendMessage, messageHistory, connectionStatus } = useGameWebSocket();
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
