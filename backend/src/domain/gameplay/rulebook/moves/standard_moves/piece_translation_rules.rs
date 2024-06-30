use super::translations;
use crate::domain::gameplay::chess_set;

// etc.

// TODO -> maybe not translation in the signature - maybe just square.
// pub fn is_translation_legal(
//     translation: translations::Translation,
//     piece_type: &chess_set::PieceType,
// ) -> bool {
//     let translation_rules = get_translation_rules_for_piece(piece_type);
//     let permitted_by_translation_rules = translation_rules
//         .into_iter()
//         .any(|rule| rule.allows_translation(translation));
//
//     let can_jump = piece_type == chess_set::PieceType::Knight;
//     let is_obstructed = translation.is_obstructed() && !can_jump;
//
//     permitted_by_translation_rules && !is_obstructed
// }

const PAWN_TRANSLATION_RULES: Vec<translations::Translation> = vec![];

// fn get_translation_rules_for_piece(
//     piece_type: &chess_set::PieceType,
// ) -> Vec<translations::Translation> {
// }

/// Mechanism for defining what direction each piece can move in, and how far.
struct TranslationRule {
    vector: translations::ChessVector,
    scalable: bool,
}

impl TranslationRule {
    // Factories.
    pub fn new(vector: translations::ChessVector, scalable: bool) -> Self {
        Self { vector, scalable }
    }

    // Queries.
    pub fn allows_translation(&self, translation: &translations::Translation) -> bool {
        let vectors_match = self.vector == translation.vector;
        let scale_allowed = self.scalable || translation.scalar == 1;
        vectors_match && scale_allowed
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod translation_rule_is_allowed_tests {
        use super::super::*;
        use rstest::rstest;

        #[rstest]
        #[case::scalable_single_square(true, translations::ChessVector::new(0, -1))]
        #[case::not_scalable_single_square(false, translations::ChessVector::new(0, -1))]
        #[case::scalable_multi_square(true, translations::ChessVector::new(2, 1))]
        #[case::not_scalable_multi_square(false, translations::ChessVector::new(2, 1))]
        fn allows_unscaled_translation_when_vector_matches_translation_rule(
            #[case] scalable: bool,
            #[case] vector: translations::ChessVector,
        ) {
            let translation = translations::Translation::new(vector, 1);

            let translation_rule = TranslationRule::new(vector.clone(), scalable);

            assert!(translation_rule.allows_translation(&translation));
        }

        #[rstest]
        #[case::sclable(true)]
        #[case::not_scalable(false)]
        fn disallows_unscaled_translation_when_vector_does_not_match_translation_rule(
            #[case] scalable: bool,
        ) {
            let vector = translations::ChessVector::new(1, -1);
            let translation = translations::Translation::new(vector, 1);

            let different_vector = translations::ChessVector::new(-1, 1);
            let translation_rule = TranslationRule::new(different_vector, scalable);

            assert!(!translation_rule.allows_translation(&translation));
        }

        #[test]
        fn allows_scaled_translation_when_translation_rule_is_scalable() {
            let vector = translations::ChessVector::new(1, 0);
            let translation = translations::Translation::new(vector, 7);

            let translation_rule = TranslationRule::new(vector.clone(), true);

            assert!(translation_rule.allows_translation(&translation));
        }

        #[test]
        fn disallows_scaled_translation_for_when_translation_rule_is_not_scalable() {
            let vector = translations::ChessVector::new(0, -1);
            let translation = translations::Translation::new(vector, 3);

            let translation_rule = TranslationRule::new(vector.clone(), false);

            assert!(!translation_rule.allows_translation(&translation));
        }

        #[test]
        fn disallows_scaled_translation_when_vector_does_not_match_translation_rule() {
            let vector = translations::ChessVector::new(0, -1);
            let translation = translations::Translation::new(vector, 3);

            let different_vector = translations::ChessVector::new(-1, 0);
            let translation_rule = TranslationRule::new(different_vector, true);

            assert!(!translation_rule.allows_translation(&translation));
        }
    }
}
