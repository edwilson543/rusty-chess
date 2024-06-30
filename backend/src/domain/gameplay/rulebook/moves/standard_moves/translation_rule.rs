use super::translations;

/// Mechanism for defining whether a certain translation is allowed..
pub trait TranslationRule {
    fn allows_translation(&self, translation: &translations::Translation) -> bool;
}

pub struct BasicTranslationRule {
    vector: translations::ChessVector,
    scalable: bool,
}

impl BasicTranslationRule {
    pub fn new(vector: translations::ChessVector, scalable: bool) -> Self {
        Self { vector, scalable }
    }
}

impl TranslationRule for BasicTranslationRule {
    // Queries.
    fn allows_translation(&self, translation: &translations::Translation) -> bool {
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

            let translation_rule = BasicTranslationRule::new(vector.clone(), scalable);

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
            let translation_rule = BasicTranslationRule::new(different_vector, scalable);

            assert!(!translation_rule.allows_translation(&translation));
        }

        #[test]
        fn allows_scaled_translation_when_translation_rule_is_scalable() {
            let vector = translations::ChessVector::new(1, 0);
            let translation = translations::Translation::new(vector, 7);

            let translation_rule = BasicTranslationRule::new(vector.clone(), true);

            assert!(translation_rule.allows_translation(&translation));
        }

        #[test]
        fn disallows_scaled_translation_for_when_translation_rule_is_not_scalable() {
            let vector = translations::ChessVector::new(0, -1);
            let translation = translations::Translation::new(vector, 3);

            let translation_rule = BasicTranslationRule::new(vector.clone(), false);

            assert!(!translation_rule.allows_translation(&translation));
        }

        #[test]
        fn disallows_scaled_translation_when_vector_does_not_match_translation_rule() {
            let vector = translations::ChessVector::new(0, -1);
            let translation = translations::Translation::new(vector, 3);

            let different_vector = translations::ChessVector::new(-1, 0);
            let translation_rule = BasicTranslationRule::new(different_vector, true);

            assert!(!translation_rule.allows_translation(&translation));
        }
    }
}
