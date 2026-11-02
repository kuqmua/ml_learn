//! Каждый параметр явно создаётся через свой тип; связи между полями проверяет validate.
//! Числовые поля newtype закрыты: некорректное отдельное значение создать нельзя.
use candle_core::{Error, Result};
use candle_nn::ParamsAdamW;

macro_rules! positive_count {
    ($($name:ident),+ $(,)?) => {$ (
        #[derive(Clone, Copy, Debug)]
        pub(crate) struct $name(usize);
        impl TryFrom<usize> for $name {
            type Error = Error;
            fn try_from(value: usize) -> Result<Self> {
                if value == 0 { candle_core::bail!(concat!(stringify!($name), ": значение должно быть >0")); }
                Ok(Self(value))
            }
        }
        impl $name { pub(crate) const fn get(self) -> usize { self.0 } }
    )+};
}
positive_count!(BlockSize, Channels, HeadCount, LayerCount, BatchSize, EvaluationInterval, EvaluationBatches);

#[derive(Clone, Copy, Debug)]
pub(crate) struct Dropout(f64);
impl TryFrom<f64> for Dropout {
    type Error = Error;
    fn try_from(value: f64) -> Result<Self> {
        if !(0.0..1.0).contains(&value) {
            candle_core::bail!("Dropout: нужно конечное число в [0,1)");
        }
        Ok(Self(value))
    }
}
impl Dropout {
    pub(crate) const fn get(self) -> f64 {
        self.0
    }
}

// Ноль допустим: можно пропустить обучение или генерацию.
macro_rules! count_with_zero {
    ($($name:ident),+ $(,)?) => {$ (
        #[derive(Clone, Copy, Debug)]
        pub(crate) struct $name(usize);
        impl From<usize> for $name { fn from(value: usize) -> Self { Self(value) } }
        impl $name { pub(crate) const fn get(self) -> usize { self.0 } }
    )+};
}
count_with_zero!(TrainingSteps, GeneratedTokens);

#[derive(Clone, Copy, Debug)]
pub(crate) struct Prompt(&'static str);
// FromStr принимает произвольный &str и не может сохранить его как &'static str.
#[allow(clippy::try_from_instead_of_from_str)]
impl TryFrom<&'static str> for Prompt {
    type Error = Error;
    fn try_from(value: &'static str) -> Result<Self> {
        if value.is_empty() {
            candle_core::bail!("Prompt не может быть пустым");
        }
        Ok(Self(value))
    }
}
impl Prompt {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}

// Все f64 внутри AdamW тоже проверяем отдельными типами.
macro_rules! float_setting {
    ($name:ident, $condition:expr, $message:literal) => {
        #[derive(Clone, Copy, Debug)]
        pub(crate) struct $name(f64);
        impl TryFrom<f64> for $name {
            type Error = Error;
            fn try_from(value: f64) -> Result<Self> {
                if !value.is_finite() || !($condition)(value) {
                    candle_core::bail!($message);
                }
                Ok(Self(value))
            }
        }
        impl $name {
            pub(crate) const fn get(self) -> f64 {
                self.0
            }
        }
    };
}
float_setting!(LearningRate, |v: f64| -> bool { v > 0.0 }, "LearningRate: нужно конечное число >0");
float_setting!(WeightDecay, |v: f64| -> bool { v >= 0.0 }, "WeightDecay: нужно конечное число >=0");
float_setting!(AdamBeta1, |v: f64| -> bool { (0.0..1.0).contains(&v) }, "AdamBeta1: нужно число в [0,1)");
float_setting!(AdamBeta2, |v: f64| -> bool { (0.0..1.0).contains(&v) }, "AdamBeta2: нужно число в [0,1)");
float_setting!(AdamEpsilon, |v: f64| -> bool { v > 0.0 }, "AdamEpsilon: нужно конечное число >0");

#[derive(Debug)]
pub(crate) struct AdamWSettings {
    pub(crate) lr: LearningRate,
    pub(crate) weight_decay: WeightDecay,
    pub(crate) beta1: AdamBeta1,
    pub(crate) beta2: AdamBeta2,
    pub(crate) eps: AdamEpsilon,
}
impl AdamWSettings {
    // Возвращаем копию для библиотеки, а не &mut к проверенному состоянию.
    pub(crate) fn params(&self) -> ParamsAdamW {
        ParamsAdamW {
            lr: self.lr.get(),
            weight_decay: self.weight_decay.get(),
            beta1: self.beta1.get(),
            beta2: self.beta2.get(),
            eps: self.eps.get(),
        }
    }
}

pub(crate) struct Settings {
    pub(crate) block: BlockSize,
    pub(crate) channels: Channels,
    pub(crate) heads: HeadCount,
    pub(crate) layers: LayerCount,
    pub(crate) dropout: Dropout,
    pub(crate) batch: BatchSize,
    pub(crate) steps: TrainingSteps,
    pub(crate) eval_every: EvaluationInterval,
    pub(crate) eval_batches: EvaluationBatches,
    pub(crate) optimizer: AdamWSettings,
    pub(crate) seed: u64, // Любой u64 — допустимый seed.
    pub(crate) prompt: Prompt,
    pub(crate) generate: GeneratedTokens,
    pub(crate) demo_only: bool, // Оба значения bool допустимы.
}
impl Settings {
    pub(crate) fn validate(&self) -> Result<()> {
        // Это связи между полями, их невозможно проверить одним отдельным newtype.
        if !self.channels().is_multiple_of(self.heads()) {
            candle_core::bail!("channels должно делиться на heads без остатка");
        }
        if u32::try_from(self.block()).is_err() {
            candle_core::bail!("block не помещается в u32 для индексов позиций");
        }
        if self.channels().checked_mul(4).is_none() || self.batch().checked_mul(self.block()).is_none() {
            candle_core::bail!("Размеры GPT или batch вызывают переполнение usize");
        }
        Ok(())
    }
}

macro_rules! getters {
    ($($name:ident: $return_type:ty),+ $(,)?) => {impl Settings {$ (
        pub(crate) fn $name(&self) -> $return_type { self.$name.get() }
    )+}};
}
getters!(block: usize, channels: usize, heads: usize, layers: usize, dropout: f64,
    batch: usize, steps: usize, eval_every: usize, eval_batches: usize,
    prompt: &'static str, generate: usize);
impl Settings {
    pub(crate) fn optimizer(&self) -> ParamsAdamW {
        self.optimizer.params()
    }
    pub(crate) fn seed(&self) -> u64 {
        self.seed
    }
    pub(crate) fn demo_only(&self) -> bool {
        self.demo_only
    }
}

#[cfg(test)]
pub(crate) fn test_settings() -> Result<Settings> {
    Ok(Settings {
        block: BlockSize::try_from(4)?,
        channels: Channels::try_from(8)?,
        heads: HeadCount::try_from(2)?,
        layers: LayerCount::try_from(2)?,
        dropout: Dropout::try_from(0.2)?,
        batch: BatchSize::try_from(2)?,
        steps: TrainingSteps::from(60),
        eval_every: EvaluationInterval::try_from(10)?,
        eval_batches: EvaluationBatches::try_from(2)?,
        optimizer: AdamWSettings {
            lr: LearningRate::try_from(0.001)?,
            weight_decay: WeightDecay::try_from(0.01)?,
            beta1: AdamBeta1::try_from(0.9)?,
            beta2: AdamBeta2::try_from(0.999)?,
            eps: AdamEpsilon::try_from(1e-8)?,
        },
        seed: 1337,
        prompt: Prompt::try_from("\n")?,
        generate: GeneratedTokens::from(20),
        demo_only: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scalar_types_reject_invalid_boundaries() {
        macro_rules! positive { ($($t:ty),+) => {$ (
            assert!(<$t>::try_from(0usize).is_err());
            assert_eq!(<$t>::try_from(1usize).unwrap().get(),1);
        )+}; }
        positive!(BlockSize, Channels, HeadCount, LayerCount, BatchSize, EvaluationInterval, EvaluationBatches);
        [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.1, 1.0].into_iter().for_each(|invalid: f64| {
            assert!(Dropout::try_from(invalid).is_err());
            assert!(AdamBeta1::try_from(invalid).is_err());
            assert!(AdamBeta2::try_from(invalid).is_err());
        });
        assert_eq!(Dropout::try_from(0.0).unwrap().get(), 0.0);
        assert!(Dropout::try_from(1.0 - f64::EPSILON).is_ok());
        [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0, 0.0].into_iter().for_each(|invalid: f64| {
            assert!(LearningRate::try_from(invalid).is_err());
            assert!(AdamEpsilon::try_from(invalid).is_err());
        });
        [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0].into_iter().for_each(|invalid: f64| {
            assert!(WeightDecay::try_from(invalid).is_err());
        });
        assert!(WeightDecay::try_from(0.0).is_ok());
        assert!(Prompt::try_from("").is_err());
        assert_eq!(Prompt::try_from("\n").unwrap().get(), "\n");
    }
    #[test]
    fn settings_reject_invalid_combinations_and_keep_zero_steps() -> Result<()> {
        assert!(
            Settings {
                channels: Channels::try_from(7)?,
                ..test_settings()?
            }
            .validate()
            .is_err()
        );
        assert!(
            Settings {
                block: BlockSize::try_from(usize::MAX)?,
                ..test_settings()?
            }
            .validate()
            .is_err()
        );
        assert!(
            Settings {
                batch: BatchSize::try_from(usize::MAX)?,
                ..test_settings()?
            }
            .validate()
            .is_err()
        );
        let s: Settings = Settings {
            steps: TrainingSteps::from(0),
            generate: GeneratedTokens::from(0),
            seed: 0,
            ..test_settings()?
        };
        s.validate()?;
        assert_eq!(s.steps(), 0);
        assert_eq!(s.generate(), 0);
        assert_eq!(s.seed(), 0);
        let mut copy: ParamsAdamW = s.optimizer();
        copy.lr = -1.0;
        assert!(copy.lr < 0.0 && s.optimizer().lr > 0.0);
        Ok(())
    }
}
