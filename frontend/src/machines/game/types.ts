// Context.
import * as chess from "../../domain/chess.ts";

export interface GameContextProps {
  game: chess.Game | null;
  publicGameId: number | null;
  localPlayerColour: chess.Colour;
  squareToMoveFrom: chess.Square | null;
  legalMoves: chess.Move[];
  engine: chess.Engine;
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
  fromSquare: chess.Square;
  toSquare: chess.Square;
}

export interface SelectSquareToMoveFrom {
  type: GameEvent.SetSquareToMoveFrom;
  square: chess.Square | null;
}

interface SetActiveGameEvent {
  type:
    | GameEvent.GameLoaded
    | GameEvent.GameStarted
    | GameEvent.MovePlayed
    | GameEvent.MoveGeneratedAndPlayed;
  output: chess.Game;
}

interface SetLegalMoves {
  type: GameEvent.SetLegalMoves;
  output: chess.Move[];
}

interface SetEngine {
  type: GameEvent.SetEngine;
  engine: chess.Engine;
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
