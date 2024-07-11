use crate::domain::gameplay::chess_set;
use rocket::serde::json;
use serde;

impl serde::Serialize for chess_set::Rank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let rank_str = format!("{}", self.index());
        serializer.serialize_str(rank_str.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_rank_to_string_integer() {
        let rank = chess_set::Rank::Three;

        let rank_json = json::to_string(&rank);

        assert_eq!(rank_json.unwrap(), "\"3\"");
    }
}
