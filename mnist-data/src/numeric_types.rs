// Проверенные числовые типы для загрузки MNIST и вычислений моделей.
// Нормализованная яркость PNG: от 0 до 1, хранится в f64.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalizedPixelBrightness(f64);
impl TryFrom<f64> for NormalizedPixelBrightness {
    type Error = String;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(format!(
                "Недопустимое значение для NormalizedPixelBrightness: {value}"
            ));
        }
        Ok(Self(value))
    }
}
impl NormalizedPixelBrightness {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f64 {
        self.0
    }
}
// Вес пикселя: конечное число, может быть отрицательным.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PixelWeight(f32);
impl TryFrom<f32> for PixelWeight {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для PixelWeight: {value}"));
        }
        Ok(Self(value))
    }
}
impl PixelWeight {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Смещение оценки: конечное число любого знака.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bias(f32);
impl TryFrom<f32> for Bias {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для Bias: {value}"));
        }
        Ok(Self(value))
    }
}
impl Bias {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Оценка цифры до softmax: конечное число любого знака, не вероятность.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DigitScore(f32);
impl TryFrom<f32> for DigitScore {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для DigitScore: {value}"));
        }
        Ok(Self(value))
    }
}
impl DigitScore {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Производная ошибки: конечное число любого знака.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gradient(f32);
impl TryFrom<f32> for Gradient {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(format!("Недопустимое значение для Gradient: {value}"));
        }
        Ok(Self(value))
    }
}
impl Gradient {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
// Вероятность: конечное число от 0 до 1; сумма проверяется формулой softmax.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Probability(f32);
impl TryFrom<f32> for Probability {
    type Error = String;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(format!("Недопустимое значение для Probability: {value}"));
        }
        Ok(Self(value))
    }
}
impl Probability {
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
pub struct PixelBrightnessForComputation(f32);
impl TryFrom<NormalizedPixelBrightness> for PixelBrightnessForComputation {
    type Error = String;
    fn try_from(value: NormalizedPixelBrightness) -> Result<Self, Self::Error> {
        // Диапазон уже проверен при загрузке; здесь разрешаем округление f64 → f32.
        Ok(Self(value.value() as f32))
    }
}
impl PixelBrightnessForComputation {
    pub const ZERO: Self = Self(0.0);
    pub fn value(self) -> f32 {
        self.0
    }
}
