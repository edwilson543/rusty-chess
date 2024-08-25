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
  StartingGame = "STARTING_GAME",
  PlayerTurn = "PLAYER_TURN",
  OpponentTurn = "OPPONENT_TURN",
  Unavailable = "UNAVAILABLE",
}
