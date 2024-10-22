import {
  faChessBishop,
  faChessKing,
  faChessKnight,
  faChessPawn,
  faChessQueen,
  faChessRook,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import * as types from "../lib/types.ts";

interface PieceProps {
  piece: types.Piece;
  canBeSelected: boolean;
  isSelected: boolean;
  onClick: () => void;
}

export const Piece = (props: PieceProps) => {
  // Styling.
  const colour = props.piece.colour === types.Colour.White ? "white" : "black";
  const cursor = props.canBeSelected ? "pointer" : "default";

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
        style={{ color: colour, cursor: cursor }}
        border={props.isSelected}
        onClick={props.onClick}
      />
    </div>
  );
};

const iconMapping = {
  [types.PieceType.Pawn]: faChessPawn,
  [types.PieceType.Knight]: faChessKnight,
  [types.PieceType.Bishop]: faChessBishop,
  [types.PieceType.Rook]: faChessRook,
  [types.PieceType.Queen]: faChessQueen,
  [types.PieceType.King]: faChessKing,
};
