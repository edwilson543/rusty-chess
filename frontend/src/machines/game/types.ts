// Context
import { Game } from "../../lib/types.ts";

export interface GameContextProps {
  game: Game | null;
}

// Events

export enum GameEvent {
  StartGame = "xstate.done.actor.startGame",
}

interface SetActiveGameEvent {
  type: GameEvent.StartGame;
  output: Game;
}

export type GameEventProps = SetActiveGameEvent;

// States

export enum GameState {
  Idle = "IDLE",
  LocalPlayerTurn = "LOCAL_PLAYER_TURN",
  OpponentPlayerTurn = "OPPONENT_TURN",
  Unavailable = "UNAVAILABLE",
  // Loading states.
  StartingGame = "STARTING_GAME",
  SubmittingMove = "SUBMITTING_MOVE",
  FetchingOpponentMove = "FETCHING_OPPONENT_MOVE",
}
