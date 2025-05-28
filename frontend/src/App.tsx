import "./styles/App.css";

import { useMemo } from "react";

import { useSearchParams, BrowserRouter } from "react-router-dom";

import { Game } from "./components/Game.tsx";
import { GameMachineContext } from "./context.ts";
// import { inspect } from "./lib/inspector.ts";

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
  const [searchParams] = useSearchParams();
  const publicGameId = useMemo<number | null>(
    () => getPublicGameId(searchParams),
    [searchParams],
  );

  return (
    <>
      <GameMachineContext.Provider
        options={{
          input: { publicGameId: publicGameId },
          // inspect,
        }}
      >
        {children}
      </GameMachineContext.Provider>
    </>
  );
}

const getPublicGameId = (searchParams: URLSearchParams): number | null => {
  const id = searchParams.get("gameId");
  return id ? parseInt(id) : null;
};
