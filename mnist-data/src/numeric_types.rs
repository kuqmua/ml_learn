// Проверенные числовые типы для загрузки MNIST и вычислений моделей.
// Нормализованная яркость PNG: от 0 до 1, хранится в f64.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalizedPixelBrightnessF64(f64);
impl TryFrom<f64> for NormalizedPixelBrightnessF64 {
    type Error = String;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(format!(
                "Недопустимое значение для NormalizedPixelBrightnessF64: {value}"
            ));
        }
        Ok(Self(value))
    }
}
impl NormalizedPixelBrightnessF64 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f64 {
        self.0
    }
}
// Вес пикселя: конечное число, может быть отрицательным.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PixelWeightF32(f32);
impl TryFrom<f32> for PixelWeightF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для PixelWeightF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl PixelWeightF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Смещение оценки: конечное число любого знака.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BiasF32(f32);
impl TryFrom<f32> for BiasF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для BiasF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl BiasF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Оценка цифры до softmax: конечное число любого знака, не вероятность.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DigitScoreF32(f32);
impl TryFrom<f32> for DigitScoreF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для DigitScoreF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl DigitScoreF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Производная ошибки: конечное число любого знака.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientF32(f32);
impl TryFrom<f32> for GradientF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для GradientF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl GradientF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Вероятность: конечное число от 0 до 1; сумма проверяется формулой softmax.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProbabilityF32(f32);
impl TryFrom<f32> for ProbabilityF32 {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(format!("Недопустимое значение для ProbabilityF32: {value}"));
        }
        Ok(Self(value))
    }
}
impl ProbabilityF32 {
    pub const ZERO: Self = Self(0.0);
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
pub struct PixelBrightnessForComputationF32(f32);
impl TryFrom<NormalizedPixelBrightnessF64> for PixelBrightnessForComputationF32 {
    type Error = String;
    fn try_from(value: NormalizedPixelBrightnessF64) -> Result<Self, Self::Error> {
        // Диапазон уже проверен при загрузке; здесь разрешаем округление f64 → f32.
        Ok(Self(value.value() as f32))
    }
}
impl PixelBrightnessForComputationF32 {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
