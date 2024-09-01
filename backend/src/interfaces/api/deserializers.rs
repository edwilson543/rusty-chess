use crate::domain::chess_set;
use serde;

#[derive(serde::Deserialize)]
pub struct Move<'request> {
    player: &'request str,
    from_square: &'request str,
    to_square: &'request str,
}

impl<'request> Move<'request> {
    pub fn get_player(&'request self) -> chess_set::Colour {
        match self.player {
            "White" => chess_set::Colour::White,
            "Black" => chess_set::Colour::Black,
            _ => panic!("Invalid colour!"),
        }
    }

    pub fn get_from_square(&'request self) -> chess_set::Square {
        deserialize_to_square(self.from_square)
    }

    pub fn get_to_square(&'request self) -> chess_set::Square {
        deserialize_to_square(self.to_square)
    }
}

/// Convert `A1` to the square in file A, and rank 1.
fn deserialize_to_square(value: &str) -> chess_set::Square {
    let chars: Vec<char> = value.chars().collect();
    let file = chess_set::File::from_letter(chars[0]);
    let rank = chess_set::Rank::from_index(chars[1].to_digit(10).unwrap() as i8);
    chess_set::Square::new(rank, file)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::domain::chess_set::{Colour, File, Rank, Square};

    #[test]
    fn can_get_domain_objects_from_move() {
        let move_ = Move {
            player: "White",
            from_square: "A2",
            to_square: "A4",
        };

        assert_eq!(move_.get_player(), Colour::White);

        let expected_from_square = Square::new(Rank::Two, File::A);
        assert_eq!(move_.get_from_square(), expected_from_square);

        let expected_to_square = Square::new(Rank::Four, File::A);
        assert_eq!(move_.get_to_square(), expected_to_square);
    }

    #[test]
    fn can_deserialize_str_to_square() {
        let string = "C7";

        let square = deserialize_to_square(string);

        assert_eq!(square.get_rank(), &Rank::Seven);
        assert_eq!(square.get_file(), &File::C);
    }
}
