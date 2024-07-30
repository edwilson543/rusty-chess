import "./styles/App.css";
import { Game } from "./components/Game.tsx";
import { GameMachineContext } from "./context.ts";

function App() {
  return (
    <>
      <h1>Chess</h1>
      <GameMachineContext.Provider>
        <Game />
      </GameMachineContext.Provider>
    </>
  );
}

export default App;
