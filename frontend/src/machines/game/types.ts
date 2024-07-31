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
  Black = "B",
  White = "W",
}

export enum PieceType {
  Pawn = "P",
  Knight = "N",
  Bishop = "B",
  Rook = "R",
  Queen = "Q",
  King = "K",
}

export interface Piece {
  colour: Colour;
  pieceType: PieceType;
}


export interface Chessboard {
  position: Record<File, Record<Rank, Piece | null>>;
}

export interface Game {
  id: number,
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
