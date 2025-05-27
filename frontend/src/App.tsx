import "./styles/App.css";
import { Game } from "./components/Game.tsx";
import { GameMachineContext } from "./context.ts";

// import { inspect } from "./lib/inspector.ts";

function App() {
  const searchParams = new URLSearchParams(window.location.search);
  const publicGameId = getPublicGameId(searchParams);

  return (
    <>
      <GameMachineContext.Provider
        options={{
          input: { publicGameId: publicGameId },
        }}
      >
        <Game />
      </GameMachineContext.Provider>
    </>
  );
}

export default App;

const getPublicGameId = (searchParams: URLSearchParams): number | null => {
  const id = searchParams.get("gameId");
  return id ? parseInt(id) : null;
};
