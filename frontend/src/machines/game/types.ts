// Context

enum Colour {
  Black = "BLACK",
  White = "WHITE",
}

enum PieceType {
  Pawn = "PAWN",
  Knight = "Knight",
  Bishop = "Bishop",
  Rook = "Rook",
  Queen = "Queen",
  King = "King",
}

interface ChessboardSquare {
  colour: Colour;
  pieceType: PieceType;
}

interface Chessboard {
  position: Record<string, ChessboardSquare>;
}

interface Game {
  chessboard: Chessboard;
  player: Colour;
}

export interface GameContextProps {
  game?: Game;
}

// Events

export enum GameEvent {
  StartNewGame = "START_NEW_GAME",
}

interface SetActiveGameEvent {
  type: GameEvent.StartNewGame;
  game: Game;
}

export type GameEventProps = SetActiveGameEvent;

// States

export enum GameState {
  Idle = "IDLE",
  PlayerTurn = "PLAYER_TURNS",
  OpponentTurn = "OPPONENT_TURN",
}
