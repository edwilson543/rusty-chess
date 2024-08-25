import { GameMachineContext } from "../context.ts";
import * as types from "../lib/types.ts";
import { GameEvent } from "../machines/game/types.ts";

export const FixedMoveButton = () => {
  const gameMachineRef = GameMachineContext.useActorRef();

  const playMove = () => {
    // TODO -> add `FromSquare` context prop. So this func will just need `toSquare.
    const fromSquare: types.Square = {
      rank: types.Rank.Two,
      file: types.File.C,
      piece: null, // TODO -> make unnecessary.
    };
    const toSquare: types.Square = {
      rank: types.Rank.Three,
      file: types.File.C,
      piece: null, // TODO -> make unnecessary.
    };

    gameMachineRef.send({
      type: GameEvent.PlayMove,
      fromSquare: fromSquare,
      toSquare: toSquare,
    });
  };

  return <button onClick={playMove}>Play move</button>;
};
