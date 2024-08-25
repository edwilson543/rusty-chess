// Context
import * as types from "../../lib/types.ts";

export interface GameContextProps {
  game: types.Game | null;
}

// Events

export enum GameEvent {
  PlayMove = "play-move",
  // Events that set the active game.
  GameStarted = "xstate.done.actor.startGame",
  MovePlayed = "xstate.done.actor.playMove",
}

interface SetActiveGameEvent {
  type: GameEvent.GameStarted | GameEvent.MovePlayed;
  output: types.Game;
}

export interface PlayMoveEvent {
  type: GameEvent.PlayMove;
  fromSquare: types.Square;
  toSquare: types.Square;
}

export type GameEventProps = SetActiveGameEvent | PlayMoveEvent;

// States

export enum GameState {
  Idle = "idle",
  LocalPlayerTurn = "local-play-turn",
  OpponentPlayerTurn = "opponent-turn",
  Unavailable = "unavailable",
  // Loading states.
  StartingGame = "starting-game",
  SubmittingMove = "submitting-move",
  FetchingOpponentMove = "fetching-opponent-move",
}
