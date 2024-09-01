import "./styles/App.css";
import { Game } from "./components/Game.tsx";
import { GameMachineContext } from "./context.ts";
import { inspect } from "./lib/inspector.ts";

function App() {
  return (
    <>
      {/*<h1>Chess</h1>*/}
      <GameMachineContext.Provider options={{ inspect }}>
        <Game />
      </GameMachineContext.Provider>
    </>
  );
}

export default App;
