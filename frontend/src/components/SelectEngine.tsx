import { Engine } from "../domain/chess.ts";
import { useActiveChessGame, useChessGameActions } from "../hooks/";

export const SelectEngine = () => {
  const { engine: selectedEngine } = useActiveChessGame();
  const { selectEngine: onEngineChange } = useChessGameActions();

  return (
    <form>
      <label htmlFor={"engine"}>
        <b>Engine: </b>
        <select
          onChange={(e) => onEngineChange(e.target.value)}
          defaultValue={selectedEngine}
          name={"engine"}
        >
          {Object.values(Engine).map((engine) => (
            <option key={engine} value={engine}>
              {engine}
            </option>
          ))}
        </select>
      </label>
    </form>
  );
};
