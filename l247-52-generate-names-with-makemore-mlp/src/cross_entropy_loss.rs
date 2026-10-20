/// Ошибка предсказания от 0 до +∞; отрицательные значения и NaN запрещены.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CrossEntropyLossAsNonNegativeF32(f32);

impl CrossEntropyLossAsNonNegativeF32 {
    pub fn value(self) -> f32 {
        self.0
    }
}

impl TryFrom<f32> for CrossEntropyLossAsNonNegativeF32 {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value >= 0.0 {
            Ok(Self(value))
        } else {
            Err("cross-entropy loss должен быть неотрицательным и не NaN")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CrossEntropyLossAsNonNegativeF32;

    #[test]
    fn accepts_nonnegative_values_including_infinity() {
        for value in [0.0, -0.0, f32::MIN_POSITIVE, 3.296, f32::MAX, f32::INFINITY] {
            assert_eq!(
                CrossEntropyLossAsNonNegativeF32::try_from(value)
                    .unwrap()
                    .value(),
                value
            );
        }
    }

    #[test]
    fn rejects_negative_values_and_nan() {
        for value in [-f32::MIN_POSITIVE, -1.0, f32::NEG_INFINITY, f32::NAN] {
            assert!(CrossEntropyLossAsNonNegativeF32::try_from(value).is_err());
        }
    }
}
