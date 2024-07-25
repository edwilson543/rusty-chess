import "./styles/App.css";
import { useGameWebSocket } from "./lib/websocket";

function App() {
  const { sendMessage, messageHistory } = useGameWebSocket();

  return (
    <>
      <h1>Chess</h1>
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
