// Context
import * as types from "../../lib/types.ts";

export interface GameContextProps {
  game: types.Game | null;
  localPlayerColour: types.Colour;
  squareToMoveFrom: types.Square | null;
  legalMoves: types.Move[];
  engine: types.Engine;
}

// Events.

export enum GameEvent {
  StartNewGame = "start-new-game",
  PlayMove = "play-move",
  SetSquareToMoveFrom = "set-square-to-move-from",
  SwapColours = "swap-colours",
  SetEngine = "set-engine",
  // Events that set the active game.
  GameStarted = "xstate.done.actor.startGame",
  MovePlayed = "xstate.done.actor.playMove",
  MoveGeneratedAndPlayed = "xstate.done.actor.generateAndPlayNextMove",
  SetLegalMoves = "xstate.done.actor.getLegalMoves",
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

interface SetLegalMoves {
  type: GameEvent.SetLegalMoves;
  output: types.Move[];
}

export interface SetEngine {
  type: GameEvent.SetEngine;
  engine: types.Engine;
}

export type GameEventProps =
  | PlayMoveEvent
  | SelectSquareToMoveFrom
  | SetActiveGameEvent
  | SetLegalMoves
  | SetEngine;

// States.

export enum GameState {
  Idle = "idle",
  LocalPlayerTurn = "local-play-turn",
  OpponentPlayerTurn = "opponent-turn",
  GameComplete = "game-complete",
  Unavailable = "unavailable",
  // Loading states.
  StartingGame = "starting-game",
  SubmittingLocalPlayerMove = "submitting-local-player-move",
  SubmittingOpponentPlayerMove = "submitting-opponent--player-move",
}

// Actions.

export enum Action {
  SetActiveGame = "set-active-game",
  SetLocalPlayerToWhite = "set-local-player-to-white",
  SwapColours = "swap-colours",
  SetEngine = "set-engine",
  // Square to play from.
  SetSquareToMoveFrom = "set-square-to-move-from",
  ClearSquareToPlayFrom = "clear-square-to-play-from",
  // Legal moves.
  SetLegalMoves = "set-legal-moves",
  ClearLegalMoves = "clear-legal-moves",
}

// Guards.

export enum Guard {
  GameIsUnset = "game-is-unset",
  GameIsComplete = "game-is-complete",
}
