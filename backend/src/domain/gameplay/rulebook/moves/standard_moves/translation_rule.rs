use super::translations;

/// Mechanism for defining whether a certain translation is allowed..
pub trait TranslationRule {
    fn allows_translation(&self, translation: &translations::Translation) -> bool;
}

// Re-usable translation rules.

pub struct SingleSquareTranslation {
    vector: translations::ChessVector,
}

pub struct MultiSquareTranslation {
    vector: translations::ChessVector,
}

impl SingleSquareTranslation {
    pub fn new(vector: translations::ChessVector) -> Self {
        Self { vector }
    }
}

impl TranslationRule for SingleSquareTranslation {
    fn allows_translation(&self, translation: &translations::Translation) -> bool {
        self.vector == translation.vector && translation.scalar == 1 && !translation.is_obstructed()
    }
}

impl MultiSquareTranslation {
    pub fn new(vector: translations::ChessVector) -> Self {
        Self { vector}
    }
}

impl TranslationRule for MultiSquareTranslation {
    fn allows_translation(&self, translation: &translations::Translation) -> bool {
        self.vector == translation.vector && !translation.is_obstructed()
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod single_square_translation_tests {
        use super::super::*;
        use rstest::rstest;

        #[rstest]
        #[case::forwards(translations::ChessVector::new(0, 1))]
        #[case::diagonal(translations::ChessVector::new(-1, 1))]
        fn allows_single_square_translation_matching_vector(
            #[case] vector: translations::ChessVector,
        ) {
            let translation = translations::Translation::new(vector, 1);

            let translation_rule = SingleSquareTranslation::new(vector.clone());

            assert!(translation_rule.allows_translation(&translation));
        }

        #[test]
        fn disallows_single_square_translation_not_matching_vector() {
            let vector = translations::ChessVector::new(1, -1);
            let translation = translations::Translation::new(vector, 1);

            let different_vector = translations::ChessVector::new(-1, 1);
            let translation_rule = SingleSquareTranslation::new(different_vector);

            assert!(!translation_rule.allows_translation(&translation));
        }

        #[test]
        fn disallows_multi_square_translation_matching_vector() {
            let vector = translations::ChessVector::new(1, -1);
            let translation = translations::Translation::new(vector, 3);

            let translation_rule = SingleSquareTranslation::new(vector.clone());

            assert!(!translation_rule.allows_translation(&translation));
        }
    }

    #[cfg(test)]
    mod multi_square_translation_tests {
        use super::super::*;
        use rstest::rstest;

        #[rstest]
        #[case::single_square_forwards(translations::ChessVector::new(0, 1), 1)]
        #[case::multi_square_right(translations::ChessVector::new(1, 0), 7)]
        #[case::multi_square_diagonal(translations::ChessVector::new(-1, 1), 3)]
        fn allows_single_square_translation_matching_vector(
            #[case] vector: translations::ChessVector, #[case] scalar: u8
        ) {
            let translation = translations::Translation::new(vector, scalar);

            let translation_rule = MultiSquareTranslation::new(vector.clone());

            assert!(translation_rule.allows_translation(&translation));
        }

        #[test]
        fn disallows_translation_not_matching_vector() {
            let vector = translations::ChessVector::new(0, -1);
            let translation = translations::Translation::new(vector, 3);

            let different_vector = translations::ChessVector::new(1, 0);
            let translation_rule = MultiSquareTranslation::new(different_vector);

            assert!(!translation_rule.allows_translation(&translation));
        }
    }
}
