import { useSelector } from "@xstate/react";

import { Chessboard } from "./Chessboard";
import { GameMachineContext } from "../context.ts";
import * as types from "../lib/types.ts";

export const Game = () => {
  const gameMachineRef = GameMachineContext.useActorRef();
  const game = useSelector(gameMachineRef, (state) => state.context.game);

  return game && <GameDetails game={game} />;
};

interface GameDetailsProps {
  game: types.Game;
}

const GameDetails = (props: GameDetailsProps) => {
  const gameMachineRef = GameMachineContext.useActorRef();
  const game = useSelector(gameMachineRef, (state) => state.context.game);

  return (
    <>
      <div style={{ width: "1200px", height: "800px" }}>
        {props.game.toPlayColour && (
          <div style={{ padding: "5px" }}>
            <b>To play:</b> {props.game.toPlayColour}
          </div>
        )}
        {props.game.outcome && (
          <div style={{ padding: "5px" }}>
            <b>Outcome:</b> {props.game.outcome}
          </div>
        )}
        {game && <Chessboard chessboard={game.chessboard} />}
      </div>
    </>
  );
};
