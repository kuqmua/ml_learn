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

#[derive(Clone, Copy, Debug)]
pub struct TrainingSteps(std::num::NonZeroUsize);
impl TryFrom<usize> for TrainingSteps {
    type Error = ValidationError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self)
            .ok_or(ValidationError("число шагов должно быть положительным"))
    }
}
impl TrainingSteps {
    pub fn get(self) -> usize {
        self.0.get()
    }
}
impl fmt::Display for TrainingSteps {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// Храним заимствование, поэтому проверка не копирует обучающие примеры.
#[derive(Debug)]
pub struct NonEmptySlice<'a, T>(&'a [T]);
impl<T> Copy for NonEmptySlice<'_, T> {}
impl<T> Clone for NonEmptySlice<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T> TryFrom<&'a [T]> for NonEmptySlice<'a, T> {
    type Error = ValidationError;
    fn try_from(values: &'a [T]) -> Result<Self, Self::Error> {
        if values.is_empty() {
            Err(ValidationError(
                "набор обучающих примеров не должен быть пустым",
            ))
        } else {
            Ok(Self(values))
        }
    }
}
impl<T> Deref for NonEmptySlice<'_, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_invalid_inputs_and_preserves_valid_values() {
        assert!(TrainingSteps::try_from(0).is_err());
        assert_eq!(TrainingSteps::try_from(1).unwrap().get(), 1);
        for text in ["", "anna\n\nbob", "Anna", "a1", "анна"] {
            assert!(Names::try_from(text).is_err());
        }
        assert_eq!(&*Names::try_from("anna\nbob\n").unwrap(), &["anna", "bob"]);
        assert!(NonEmptySlice::<u8>::try_from(&[][..]).is_err());
        assert_eq!(&*NonEmptySlice::try_from(&[7u8][..]).unwrap(), &[7]);
    }
}
