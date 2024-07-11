use crate::domain::gameplay::chess_set;
use serde;

// Chess set.

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

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::serde::json;

    #[test]
    fn serializes_rank_to_string_integer() {
        let rank = chess_set::Rank::Three;

        let rank_json = json::to_string(&rank);

        assert_eq!(rank_json.unwrap(), "\"3\"");
    }

    #[test]
    fn serializes_file_to_string_letter() {
        let file = chess_set::File::F;

        let file_json = json::to_string(&file);

        assert_eq!(file_json.unwrap(), "\"F\"");
    }
}
