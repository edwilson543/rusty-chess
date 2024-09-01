import { useSelector } from "@xstate/react";

import { Chessboard } from "./Chessboard";
import { GameMachineContext } from "../context.ts";

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
        {game && <Chessboard chessboard={game.chessboard} />}
      </div>
    </>
  );
};
