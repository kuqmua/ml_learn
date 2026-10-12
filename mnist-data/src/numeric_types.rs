// Проверенные числовые типы для загрузки MNIST и вычислений моделей.
// Нормализованная яркость PNG: от 0 до 1, хранится в f64.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64(f64);
impl TryFrom<f64> for NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64 {
    type Error = String;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(format!(
                "Недопустимое значение для NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64: {value}"
            ));
        }
        Ok(Self(value))
    }
}
impl NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f64 {
        self.0
    }
}
// Вес пикселя: конечное число, может быть отрицательным.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PixelWeightFiniteF32(f32);
impl TryFrom<f32> for PixelWeightFiniteF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для PixelWeightFiniteF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl PixelWeightFiniteF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Смещение оценки: конечное число любого знака.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BiasFiniteF32(f32);
impl TryFrom<f32> for BiasFiniteF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для BiasFiniteF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl BiasFiniteF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Оценка цифры до softmax: конечное число любого знака, не вероятность.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DigitScoreFiniteF32(f32);
impl TryFrom<f32> for DigitScoreFiniteF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для DigitScoreFiniteF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl DigitScoreFiniteF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Производная средней cross-entropy по оценке цифры до softmax:
// (вероятность - целевое значение 0 или 1) / размер непустой группы.
// Проверяем общую границу [-1, 1]; более узкая [-1/N, 1/N] зависит от размера группы.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScoreGradientFiniteFromMinus1To1InclusiveF32(f32);
impl TryFrom<f32> for ScoreGradientFiniteFromMinus1To1InclusiveF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(-1.0..=1.0).contains(&value) {
            return Err(format!(
                "Недопустимое значение для ScoreGradientFiniteFromMinus1To1InclusiveF32: {value}; ожидается конечное число от -1 до 1"
            ));
        }
        Ok(Self(value))
    }
}
impl ScoreGradientFiniteFromMinus1To1InclusiveF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Производная ошибки: конечное число любого знака.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientFiniteF32(f32);
impl TryFrom<f32> for GradientFiniteF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для GradientFiniteF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl GradientFiniteF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Вероятность: конечное число от 0 до 1; сумма проверяется формулой softmax.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProbabilityFiniteFrom0To1InclusiveF32(f32);
impl TryFrom<f32> for ProbabilityFiniteFrom0To1InclusiveF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(format!("Недопустимое значение для ProbabilityFiniteFrom0To1InclusiveF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl ProbabilityFiniteFrom0To1InclusiveF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Доля правильных предсказаний: конечное число от 0 до 1 включительно.
// Например, 0.92 означает 92% правильных ответов.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CorrectPredictionFractionFiniteFrom0To1InclusiveF32(f32);
impl TryFrom<f32> for CorrectPredictionFractionFiniteFrom0To1InclusiveF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(format!(
                "Недопустимое значение для CorrectPredictionFractionFiniteFrom0To1InclusiveF32: {value}; ожидается конечное число от 0 до 1"
            ));
        }
        Ok(Self(value))
    }
}
impl CorrectPredictionFractionFiniteFrom0To1InclusiveF32 {
    pub fn value(self) -> f32 {
        self.0
    }
}
// Экспонента или среднее квадратов: конечное число >= 0.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonNegativeFiniteF32(f32);
impl TryFrom<f32> for NonNegativeFiniteF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() || value < 0.0 {
            return Err(format!(
                "Недопустимое значение для NonNegativeFiniteF32: {value}"
            ));
        }
        Ok(Self(value))
    }
}
impl NonNegativeFiniteF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Яркость после проверенного преобразования для вычислений в f32.
// Все 256 уровней PNG различимы, но дробь byte/255 может округлиться.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PixelBrightnessForComputationFiniteFrom0To1InclusiveF32(f32);
impl TryFrom<NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64> for PixelBrightnessForComputationFiniteFrom0To1InclusiveF32 {
    type Error = String;
    fn try_from(value: NormalizedPixelBrightnessFiniteFrom0To1InclusiveF64) -> Result<Self, Self::Error> {
        // Диапазон уже проверен при загрузке; здесь разрешаем округление f64 → f32.
        Ok(Self(value.value() as f32))
    }
}
impl PixelBrightnessForComputationFiniteFrom0To1InclusiveF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
