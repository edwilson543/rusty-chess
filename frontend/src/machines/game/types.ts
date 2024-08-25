// Context
import * as types from "../../lib/types.ts";

export interface GameContextProps {
  game: types.Game | null;
}

// Events

export enum GameEvent {
  StartGame = "xstate.done.actor.startGame",
  PlayMove = "play-move",
}

interface SetActiveGameEvent {
  type: GameEvent.StartGame;
  output: types.Game;
}

export interface PlayMoveEvent {
  type: GameEvent.PlayMove;
  move: types.Move;
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
