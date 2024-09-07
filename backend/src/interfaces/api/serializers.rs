use crate::domain::{chess_set, game, rulebook};
use serde;
use serde::ser::SerializeStruct;

// Square.

impl serde::Serialize for chess_set::Rank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let rank_str = format!("{}", self.index());
        serializer.serialize_str(rank_str.as_str())
    }
}

impl serde::Serialize for chess_set::File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl serde::Serialize for chess_set::Square {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut file = self.get_file().to_string();
        let rank = format!("{}", &self.get_rank().index()).clone();
        file.push_str(rank.as_str()); // e.g. A1; B3; F5
        serializer.serialize_str(file.as_str())
    }
}

// Chessboard & Game.

impl serde::Serialize for chess_set::Chessboard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("chess_set::Chessboard", 1)?;
        state.serialize_field("position", &self.position)?;
        state.end()
    }
}

impl serde::Serialize for game::Game {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("game::Game", 1)?;
        state.serialize_field("id", &self.get_id())?;
        state.serialize_field("status", &self.get_status())?;
        state.serialize_field("chessboard", &self.current_chessboard())?;
        state.end()
    }
}

impl serde::Serialize for rulebook::Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("rulebook::Move", 1)?;
        state.serialize_field("from_square", &self.from_square)?;
        state.serialize_field("to_square", &self.to_square)?;
        state.serialize_field("player", self.piece.get_colour())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::factories;
    use serde_json;

    #[test]
    fn serializes_rank_to_json() {
        let rank = chess_set::Rank::Three;

        let rank_json = serde_json::to_string(&rank);

        assert_eq!(rank_json.unwrap(), "\"3\"");
    }

    #[test]
    fn serializes_file_to_json() {
        let file = chess_set::File::F;

        let file_json = serde_json::to_string(&file);

        assert_eq!(file_json.unwrap(), "\"F\"");
    }

    #[test]
    fn serializes_square_to_json() {
        let square = chess_set::Square::new(chess_set::Rank::Eight, chess_set::File::E);

        let square_json = serde_json::to_string(&square);

        assert_eq!(square_json.unwrap(), "\"E8\"");
    }

    #[test]
    fn serializes_chessboard_to_json() {
        let chessboard = factories::chessboard();

        let chessboard_json = serde_json::to_string(&chessboard).unwrap();

        assert!(chessboard_json.contains("position"));
        assert!(chessboard_json.contains(r#""A8":{"colour":"Black","piece_type":"Rook"}"#));
        assert!(chessboard_json.contains(r#""D2":{"colour":"White","piece_type":"Pawn"}"#));
        assert!(chessboard_json.contains(r#""G5":null"#));
    }

    #[test]
    fn serializes_game_to_json() {
        let game = game::Game::new(1);

        let game_json = serde_json::to_string(&game).unwrap();

        assert!(
            game_json.starts_with(r#"{"id":1,"status":"ToPlayWhite","chessboard":{"position":{"#)
        );
    }

    #[test]
    fn serializes_move_to_json() {
        let from_square = chess_set::Square::new(chess_set::Rank::Eight, chess_set::File::E);
        let to_square = chess_set::Square::new(chess_set::Rank::Five, chess_set::File::C);
        let piece = chess_set::Piece::new(chess_set::Colour::White, chess_set::PieceType::King);

        let chess_move = rulebook::Move::new(piece, from_square, to_square);

        let move_json = serde_json::to_string(&chess_move).unwrap();

        assert_eq!(
            move_json,
            "{\"from_square\":\"E8\",\"to_square\":\"C5\",\"player\":\"White\"}"
        );
    }
}
