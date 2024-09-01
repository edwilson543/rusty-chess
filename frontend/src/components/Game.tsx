import { faArrowsRotate } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useSelector } from "@xstate/react";

import { Chessboard } from "./Chessboard";
import { GameMachineContext } from "../context.ts";
import * as types from "../lib/types.ts";
import { GameEvent } from "../machines/game/types.ts";

export const Game = () => {
  const gameMachineRef = GameMachineContext.useActorRef();
  const game = useSelector(gameMachineRef, (state) => state.context.game);

  const startNewGame = () => {
    gameMachineRef.send({ type: GameEvent.StartNewGame });
  };

  return game && <GameDetails game={game} startNewGame={startNewGame} />;
};

interface GameDetailsProps {
  game: types.Game;
  startNewGame: () => void;
}

const GameDetails = (props: GameDetailsProps) => {
  const gameMachineRef = GameMachineContext.useActorRef();
  const game = useSelector(gameMachineRef, (state) => state.context.game);

  return (
    <>
      <div style={{ width: "1200px", height: "800px" }}>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            justifyContent: "center",
            padding: "5px",
          }}
        >
          {props.game.toPlayColour && (
            <span>
              <b>To play:</b> {props.game.toPlayColour}
            </span>
          )}
          {props.game.outcome && (
            <span>
              <b>Outcome:</b> {props.game.outcome}
            </span>
          )}
          <div style={{ marginLeft: "20px" }}>
            <b>New game </b>
            <FontAwesomeIcon
              onClick={props.startNewGame}
              icon={faArrowsRotate}
              cursor={"pointer"}
            />
          </div>
        </div>
        {game && <Chessboard chessboard={game.chessboard} />}
      </div>
    </>
  );
};
