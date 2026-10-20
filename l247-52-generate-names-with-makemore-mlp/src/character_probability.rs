/// Вероятность символа в диапазоне [0, 1]; NaN и бесконечности запрещены.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CharacterProbabilityAsF32(f32);

impl CharacterProbabilityAsF32 {
    pub fn value(self) -> f32 {
        self.0
    }
}

impl TryFrom<f32> for CharacterProbabilityAsF32 {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err("вероятность символа должна быть от 0 до 1")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CharacterProbabilityAsF32;

    #[test]
    fn accepts_endpoints_and_values_inside_range() {
        for value in [0.0, -0.0, f32::MIN_POSITIVE, 0.5, 1.0] {
            assert_eq!(
                CharacterProbabilityAsF32::try_from(value).unwrap().value(),
                value
            );
        }
    }

    #[test]
    fn rejects_values_outside_range_and_nan() {
        for value in [
            -f32::MIN_POSITIVE,
            1.0 + f32::EPSILON,
            f32::NEG_INFINITY,
            f32::INFINITY,
            f32::NAN,
        ] {
            assert!(CharacterProbabilityAsF32::try_from(value).is_err());
        }
    }
}
