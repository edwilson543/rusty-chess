import { useSelector } from "@xstate/react";

import { GameMachineContext } from "../context.ts";
import * as types from "../lib/types.ts";
import { GameEvent } from "../machines/game/types.ts";

export const SelectEngine = () => {
  const gameMachineRef = GameMachineContext.useActorRef();
  const selectedEngine = useSelector(
    gameMachineRef,
    (state) => state.context.engine,
  );

  const onEngineChange = (engine: types.Engine) => {
    gameMachineRef.send({ type: GameEvent.SetEngine, engine: engine });
  };

  return (
    <span>
      <b>Engine: </b>
      <select
        onChange={(e) => onEngineChange(e.target.value)}
        defaultValue={selectedEngine}
      >
        {Object.values(types.Engine).map((engine) => (
          <option key={engine} value={engine}>
            {engine}
          </option>
        ))}
      </select>
    </span>
  );
};
