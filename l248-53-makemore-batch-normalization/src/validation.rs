use std::fmt;
use std::ops::Deref;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidationError(&'static str);
impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
impl std::error::Error for ValidationError {}

// Поле закрыто; наружу доступен только неизменяемый срез проверенных имён.
#[derive(Debug)]
pub struct Names<'a>(Vec<&'a str>);
impl<'a> TryFrom<&'a str> for Names<'a> {
    type Error = ValidationError;
    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let names: Vec<&str> = text.lines().collect();
        if names.is_empty()
            || names.iter().any(|name| {
                name.is_empty() || !name.chars().all(|character| character.is_ascii_lowercase())
            })
        {
            return Err(ValidationError(
                "нужен непустой набор непустых имён из букв a–z",
            ));
        }
        Ok(Self(names))
    }
}
impl<'a> Deref for Names<'a> {
    type Target = [&'a str];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Names<'_> {
    // Перестановка сохраняет инвариант; произвольного изменяемого доступа нет.
    pub fn shuffled(mut self, generator: &mut crate::RandomGenerator) -> Self {
        generator.shuffle(&mut self.0);
        self
    }
}

// Прямоугольный батч с минимум двумя строками для выборочной дисперсии.
#[derive(Clone, Copy, Debug)]
pub struct BatchColumns<'a> {
    values: &'a [f32],
    width: usize,
}
impl<'a> TryFrom<(&'a [f32], usize)> for BatchColumns<'a> {
    type Error = ValidationError;
    fn try_from((values, width): (&'a [f32], usize)) -> Result<Self, Self::Error> {
        if width == 0 || !values.len().is_multiple_of(width) || values.len() / width < 2 {
            return Err(ValidationError(
                "BatchNorm требует положительную ширину и минимум две полные строки",
            ));
        }
        Ok(Self { values, width })
    }
}
impl BatchColumns<'_> {
    pub fn values(&self) -> &[f32] {
        self.values
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn rows(&self) -> usize {
        self.values.len() / self.width
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FiniteLoss(f32);
impl TryFrom<f32> for FiniteLoss {
    type Error = ValidationError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(ValidationError(
                "ошибка обучения должна быть конечным числом",
            ))
        }
    }
}
impl FiniteLoss {
    pub fn get(self) -> f32 {
        self.0
    }
}
impl fmt::Display for FiniteLoss {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_invalid_batches_and_nonfinite_loss() {
        for (values, width) in [
            (&[][..], 0),
            (&[][..], 1),
            (&[1.][..], 1),
            (&[1., 2., 3.][..], 2),
        ] {
            assert!(BatchColumns::try_from((values, width)).is_err());
        }
        let batch: BatchColumns<'_> = BatchColumns::try_from((&[1., 3.][..], 1)).unwrap();
        assert_eq!(
            (batch.rows(), batch.width(), batch.values()),
            (2, 1, &[1., 3.][..])
        );
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(FiniteLoss::try_from(value).is_err());
        }
        assert_eq!(FiniteLoss::try_from(0.).unwrap().get(), 0.);
        assert_eq!(FiniteLoss::try_from(2.5).unwrap().get(), 2.5);
        assert!(Names::try_from("").is_err());
        assert!(Names::try_from("Anna").is_err());
        assert_eq!(&*Names::try_from("anna\nbob").unwrap(), &["anna", "bob"]);
    }
}
