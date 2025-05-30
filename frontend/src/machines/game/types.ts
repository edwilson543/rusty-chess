// Context.
import * as types from "../../lib/types.ts";

export interface GameContextProps {
  game: types.Game | null;
  publicGameId: number | null;
  localPlayerColour: types.Colour;
  squareToMoveFrom: types.Square | null;
  legalMoves: types.Move[];
  engine: types.Engine;
}

// Input.

export interface GameInput {
  publicGameId: number | null;
}

// Events.

export enum GameEvent {
  StartNewGame = "start-new-game",
  PlayMove = "play-move",
  SetSquareToMoveFrom = "set-square-to-move-from",
  SwapColours = "swap-colours",
  SetEngine = "set-engine",
  // Events that set the active game.
  GameLoaded = "xstate.done.actor.loadGame",
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
    | GameEvent.GameLoaded
    | GameEvent.GameStarted
    | GameEvent.MovePlayed
    | GameEvent.MoveGeneratedAndPlayed;
  output: types.Game;
}

interface SetLegalMoves {
  type: GameEvent.SetLegalMoves;
  output: types.Move[];
}

interface SetEngine {
  type: GameEvent.SetEngine;
  engine: types.Engine;
}

interface StartNewGame {
  type: GameEvent.StartNewGame;
}

interface SwapColours {
  type: GameEvent.SwapColours;
}

export type GameEventProps =
  | PlayMoveEvent
  | SelectSquareToMoveFrom
  | SetActiveGameEvent
  | SetLegalMoves
  | SetEngine
  | StartNewGame
  | SwapColours;

// States.

export enum GameState {
  // Loading states.
  Initialising = "initialising",
  LoadingExistingGame = "loading-existing-game",
  StartingNewGame = "starting-new-game",
  AssigningPlayerTurn = "assigning-player-turn",
  // Turns.
  LocalPlayerTurn = "local-play-turn",
  OpponentPlayerTurn = "opponent-turn",
  SubmittingLocalPlayerMove = "submitting-local-player-move",
  SubmittingOpponentPlayerMove = "submitting-opponent--player-move",
  // Final states.
  GameComplete = "game-complete",
  Unavailable = "unavailable",
}

// Actions.

export enum Action {
  ClearActiveGame = "clear-active-game",
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
  IsLocalPlayerTurn = "is-local-player-colour",
  PublicGameIdIsSet = "public-game-id-is-set",
  GameIsUnset = "game-is-unset",
  GameIsComplete = "game-is-complete",
}
