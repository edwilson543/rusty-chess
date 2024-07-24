use crate::domain::gameplay::chess_set;
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

// Piece.

impl serde::Serialize for chess_set::Colour {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl serde::Serialize for chess_set::PieceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

// Chessboard.

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
    fn serializes_piece_type_to_json() {
        let piece_type = chess_set::PieceType::Knight;

        let piece_type_json = serde_json::to_string(&piece_type);

        assert_eq!(piece_type_json.unwrap(), "\"N\"");
    }

    #[test]
    fn serializes_colour_to_json() {
        let colour = chess_set::Colour::Black;

        let colour_json = serde_json::to_string(&colour);

        assert_eq!(colour_json.unwrap(), "\"B\"");
    }

    #[test]
    fn serializes_chessboard_to_json() {
        let chessboard = factories::chessboard();

        let chessboard_json = serde_json::to_string(&chessboard).unwrap();

        assert!(chessboard_json.contains("position"));
        assert!(chessboard_json.contains(r#""A8":{"colour":"B","piece_type":"R"}"#));
        assert!(chessboard_json.contains(r#""D2":{"colour":"W","piece_type":"P"}"#));
        assert!(chessboard_json.contains(r#""G5":null"#));
    }
}
