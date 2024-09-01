// Context
import * as types from "../../lib/types.ts";

export interface GameContextProps {
  game: types.Game | null;
  localPlayerColour: types.Colour;
  squareToMoveFrom: types.Square | null;
}

// Events.

export enum GameEvent {
  PlayMove = "play-move",
  SetSquareToMoveFrom = "set-square-to-move-from",
  // Events that set the active game.
  GameStarted = "xstate.done.actor.startGame",
  MovePlayed = "xstate.done.actor.playMove",
  MoveGeneratedAndPlayed = "xstate.done.actor.generateAndPlayNextMove",
}

export interface PlayMoveEvent {
  type: GameEvent.PlayMove;
  fromSquare: types.Square;
  toSquare: types.Square;
}

export interface SelectSquareToMoveFrom {
  type: GameEvent.SetSquareToMoveFrom;
  square: types.Square | null;
}

interface SetActiveGameEvent {
  type:
    | GameEvent.GameStarted
    | GameEvent.MovePlayed
    | GameEvent.MoveGeneratedAndPlayed;
  output: types.Game;
}

export type GameEventProps =
  | PlayMoveEvent
  | SelectSquareToMoveFrom
  | SetActiveGameEvent;

// States.

export enum GameState {
  Idle = "idle",
  LocalPlayerTurn = "local-play-turn",
  OpponentPlayerTurn = "opponent-turn",
  Unavailable = "unavailable",
  // Loading states.
  StartingGame = "starting-game",
  SubmittingMove = "submitting-move",
  SubmittingOpponentMove = "submitting-opponent-move",
}

// Actions.

export enum Action {
  SetActiveGame = "set-active-game",
  SetSquareToMoveFrom = "set-square-to-move-from",
}

// Guards.

export enum Guard {
  GameIsUnset = "game-is-unset",
}
