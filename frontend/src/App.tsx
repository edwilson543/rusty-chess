import "./styles/App.css";

import { BrowserRouter } from "react-router-dom";

import { Game } from "./components/Game.tsx";
import { useParseGameParamsFromUrl } from "./hooks";
import { GameMachineContext } from "./machines/game";
import { inspect } from "./machines/inspector.ts";

function App() {
  return (
    <>
      <BrowserRouter>
        <GameMachineProvider>
          <Game />
        </GameMachineProvider>
      </BrowserRouter>
    </>
  );
}

export default App;

function GameMachineProvider({ children }: { children: JSX.Element }) {
  const { publicGameId, localPlayerColour } = useParseGameParamsFromUrl();

  return (
    <>
      <GameMachineContext.Provider
        options={{
          input: { publicGameId, localPlayerColour },
          inspect,
        }}
      >
        {children}
      </GameMachineContext.Provider>
    </>
  );
}
