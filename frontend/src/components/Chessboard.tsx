import {
  faChessRook,
  faChessKing,
  faChessPawn,
  faChessQueen,
  faChessBishop,
  faChessKnight,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { GameTypes } from "../machines/game";

interface ChessboardProps {
  chessboard: GameTypes.Chessboard;
}

const RANK_WIDTH = 8;

export const Chessboard = (props: ChessboardProps) => {
  const ranks = Object.values(GameTypes.Rank)
    .filter((item) => typeof item === "number")
    .reverse();

  const position = sortPosition(props.chessboard.position);
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
      }}
    >
      {ranks.map((rank) => {
        return (
          <ChessboardRank
            key={rank}
            rank={position.slice((rank - 1) * RANK_WIDTH, rank * RANK_WIDTH)}
          />
        );
      })}
    </div>
  );
};

const fileSortOrder = { A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8 };

const sortPosition = (position: GameTypes.Square[]): GameTypes.Square[] => {
  const scoreSquare = (square: GameTypes.Square): number => {
    return square.rank * 8 + fileSortOrder[square.file];
  };

  return position.sort((a, b) => scoreSquare(a) - scoreSquare(b));
};

interface ChessboardRankProps {
  rank: GameTypes.Square[];
}

const ChessboardRank = (props: ChessboardRankProps) => {
  return (
    <div style={{ display: "flex", flexDirection: "row", width: "50%" }}>
      {props.rank.map((square: GameTypes.Square) => {
        return (
          <ChessboardSquare
            key={`${square.file}${square.rank}`}
            square={square}
          />
        );
      })}
    </div>
  );
};

interface ChessboardSquareProps {
  square: GameTypes.Square;
}

const ChessboardSquare = (props: ChessboardSquareProps) => {
  const alternate = fileSortOrder[props.square.file] % 2 === 1 ? 0 : 1;
  const colour = props.square.rank % 2 === alternate ? "#aeeef2" : "#f2d4d0";

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        flex: 1,
        alignItems: "center",
        justifyContent: "center",
        aspectRatio: "1 / 1",
        border: "1px solid black",
        backgroundColor: colour,
      }}
    >
      {props.square.piece && <Piece piece={props.square.piece} />}
    </div>
  );
};

interface PieceProps {
  piece: GameTypes.Piece;
}

const Piece = (props: PieceProps) => {
  const colour =
    props.piece.colour === GameTypes.Colour.White ? "white" : "black";

  const iconMapping = {
    [GameTypes.PieceType.Pawn]: faChessPawn,
    [GameTypes.PieceType.Knight]: faChessKnight,
    [GameTypes.PieceType.Bishop]: faChessBishop,
    [GameTypes.PieceType.Rook]: faChessRook,
    [GameTypes.PieceType.Queen]: faChessQueen,
    [GameTypes.PieceType.King]: faChessKing,
  };

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        flex: 1,
      }}
    >
      <FontAwesomeIcon
        icon={iconMapping[props.piece.pieceType]}
        size={"2xl"}
        style={{ color: colour }}
      />
    </div>
  );
};
