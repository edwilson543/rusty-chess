import "./styles/App.css";
import { Game } from "./components/Game.tsx";
import { GameMachineContext } from "./context.ts";
import { useGameWebSocket } from "./lib/websocket";

function App() {
  const { sendMessage, messageHistory, connectionStatus } = useGameWebSocket();

  return (
    <>
      <h1>Chess</h1>
      <div>Connection status: {connectionStatus}</div>
      <GameMachineContext.Provider>
        <Game />
      </GameMachineContext.Provider>
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
}

export default App;
