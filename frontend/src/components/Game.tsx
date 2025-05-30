import {
  faArrowsRotate,
  faLink,
  faShuffle,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { Chessboard } from "./Chessboard";
import { SelectEngine } from "./SelectEngine.tsx";
import { useActiveChessGame, useChessGameActions } from "../hooks/";

export const Game = () => {
  const { game, localPlayerColour } = useActiveChessGame();
  const { startNewGame, swapColours } = useChessGameActions();

  const copyGameLink = () => {
    void navigator.clipboard.writeText(window.location.href);
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
              <i data-testid={"to-play-colour"}>{game.toPlayColour}</i>
            </span>
          )}
          {game.outcome && (
            <span>
              <b>Outcome:</b> {game.outcome}
            </span>
          )}
          <span>
            <b>You are: </b>
            <i data-testid={"local-player-colour"}>{localPlayerColour}</i>
            <FontAwesomeIcon
              onClick={swapColours}
              icon={faArrowsRotate}
              style={{ marginLeft: "5px", cursor: "pointer" }}
              title={"Swap colours"}
              role={"button"}
            />
          </span>
          <span>
            <b>New game</b>
            <FontAwesomeIcon
              onClick={startNewGame}
              icon={faShuffle}
              style={{ marginLeft: "5px", cursor: "pointer" }}
              title={"Start new game"}
            />
          </span>
          <SelectEngine />
        </div>
        {game && (
          <Chessboard
            chessboard={game.chessboard}
            localPlayerColour={localPlayerColour}
          />
        )}
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
            justifyContent: "flex-start",
          }}
        >
          <span>
            <FontAwesomeIcon
              onClick={copyGameLink}
              icon={faLink}
              style={{ cursor: "pointer" }}
              title={"Share game"}
              role={"button"}
            />
            <b> Share game</b>
          </span>
        </div>
      </div>
    </>
  );
};
