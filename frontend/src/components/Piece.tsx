import {
  faChessBishop,
  faChessKing,
  faChessKnight,
  faChessPawn,
  faChessQueen,
  faChessRook,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import * as chess from "../domain/chess.ts";

interface PieceProps {
  piece: chess.Piece;
  canBeSelected: boolean;
  isSelected: boolean;
  onClick: () => void;
}

export const Piece = (props: PieceProps) => {
  // Styling.
  const colour = props.piece.colour === chess.Colour.White ? "white" : "black";
  const cursor = props.canBeSelected ? "pointer" : "not-allowed";

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
        size={props.isSelected ? "4x" : "3x"}
        style={{ color: colour, cursor: cursor }}
        onClick={props.onClick}
        title={`${props.piece.colour} ${props.piece.pieceType}`}
      />
    </div>
  );
};

const iconMapping = {
  [chess.PieceType.Pawn]: faChessPawn,
  [chess.PieceType.Knight]: faChessKnight,
  [chess.PieceType.Bishop]: faChessBishop,
  [chess.PieceType.Rook]: faChessRook,
  [chess.PieceType.Queen]: faChessQueen,
  [chess.PieceType.King]: faChessKing,
};
