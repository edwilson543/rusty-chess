// Context

export enum Rank {
  One = 1,
  Two = 2,
  Three = 3,
  Four = 4,
  Five = 5,
  Six = 6,
  Seven = 7,
  Eight = 8,
}

export enum File {
  A = "A",
  B = "B",
  C = "C",
  D = "D",
  E = "E",
  F = "F",
  G = "G",
  H = "H",
}

export enum Colour {
  Black = "Black",
  White = "White",
}

export enum PieceType {
  Pawn = "Pawn",
  Knight = "Knight",
  Bishop = "Bishop",
  Rook = "Rook",
  Queen = "Queen",
  King = "King",
}

export interface Piece {
  colour: Colour;
  pieceType: PieceType;
}

export interface Square {
  rank: Rank;
  file: File;
  piece: Piece | null;
}

export interface Chessboard {
  position: Square[];
}

export interface Game {
  id: number;
  chessboard: Chessboard;
  player: Colour;
}

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
