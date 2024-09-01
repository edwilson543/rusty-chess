import { faArrowsRotate, faShuffle } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useSelector } from "@xstate/react";

import { Chessboard } from "./Chessboard";
import { GameMachineContext } from "../context.ts";
import { GameEvent } from "../machines/game/types.ts";

export const Game = () => {
  const gameMachineRef = GameMachineContext.useActorRef();
  const game = useSelector(gameMachineRef, (state) => state.context.game);
  const localPlayerColour = useSelector(
    gameMachineRef,
    (state) => state.context.localPlayerColour,
  );

  const startNewGame = () => {
    gameMachineRef.send({ type: GameEvent.StartNewGame });
  };

  const swapColours = () => {
    gameMachineRef.send({ type: GameEvent.SwapColours });
  };

  if (game === null) {
    return <></>;
  }

  return (
    <>
      <div>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            justifyContent: "space-between",
            alignItems: "center",
            padding: "5px",
          }}
        >
          {game.toPlayColour && (
            <span>
              <b>To play: </b>
              <i>{game.toPlayColour}</i>
            </span>
          )}
          {game.outcome && (
            <span>
              <b>Outcome:</b> {game.outcome}
            </span>
          )}
          <span>
            <b>You are: </b>
            <i>{localPlayerColour}</i>
            <FontAwesomeIcon
              onClick={swapColours}
              icon={faArrowsRotate}
              style={{ marginLeft: "5px", cursor: "pointer" }}
            />
          </span>
          <span>
            <b>New game</b>
            <FontAwesomeIcon
              onClick={startNewGame}
              icon={faShuffle}
              style={{ marginLeft: "5px", cursor: "pointer" }}
            />
          </span>
        </div>
        {game && (
          <Chessboard
            chessboard={game.chessboard}
            localPlayerColour={localPlayerColour}
          />
        )}
      </div>
    </>
  );
};
