import * as chess from "../domain/chess.ts";
import { useActiveChessGame } from "../hooks/useActiveChessGame.ts";
import { GameMachineContext } from "../machines/game";
import { GameEvent } from "../machines/game/types.ts";

export const SelectEngine = () => {
  const { engine: selectedEngine } = useActiveChessGame();
  const gameMachineRef = GameMachineContext.useActorRef();

  const onEngineChange = (engine: chess.Engine) => {
    gameMachineRef.send({ type: GameEvent.SetEngine, engine: engine });
  };

  return (
    <form>
      <label htmlFor={"engine"}>
        <b>Engine: </b>
        <select
          onChange={(e) => onEngineChange(e.target.value)}
          defaultValue={selectedEngine}
          name={"engine"}
        >
          {Object.values(chess.Engine).map((engine) => (
            <option key={engine} value={engine}>
              {engine}
            </option>
          ))}
        </select>
      </label>
    </form>
  );
};
