//! Поля Settings явно создаются через собственные типы с проверками TryFrom.
//! Settings::validate проверяет совместимость нескольких уже проверенных значений.
//! Числовые поля типов-обёрток закрыты: обойти проверки прямым присваиванием числа нельзя.
use candle_core::{Error, Result};
use candle_nn::ParamsAdamW;

macro_rules! positive_count {
    ($($type_name:ident),+ $(,)?) => {$ (
        #[derive(Clone, Copy, Debug)]
        pub(crate) struct $type_name(usize);
        impl TryFrom<usize> for $type_name {
            type Error = Error;
            fn try_from(raw_value: usize) -> Result<Self> {
                if raw_value == 0 { candle_core::bail!(concat!(stringify!($type_name), ": значение должно быть >0")); }
                Ok(Self(raw_value))
            }
        }
        impl $type_name { pub(crate) const fn get(self) -> usize { self.0 } }
    )+};
}
positive_count!(
    BlockSize,
    EmbeddingDimension,
    AttentionHeadCount,
    TransformerBlockCount,
    BatchSize,
    EvaluationInterval,
    EvaluationBatchCount
);

#[derive(Clone, Copy, Debug)]
pub(crate) struct DropoutProbability(f64);
impl TryFrom<f64> for DropoutProbability {
    type Error = Error;
    fn try_from(raw_value: f64) -> Result<Self> {
        if !(0.0..1.0).contains(&raw_value) {
            candle_core::bail!("Dropout: нужно конечное число в [0,1)");
        }
        Ok(Self(raw_value))
    }
}
impl DropoutProbability {
    pub(crate) const fn get(self) -> f64 {
        self.0
    }
}

// TrainingStepCount и GeneratedTokenCount допускают ноль.
// Из usize они создаются через From: ошибочного отрицательного значения у usize нет.
macro_rules! count_with_zero {
    ($($type_name:ident),+ $(,)?) => {$ (
        #[derive(Clone, Copy, Debug)]
        pub(crate) struct $type_name(usize);
        impl From<usize> for $type_name { fn from(raw_value: usize) -> Self { Self(raw_value) } }
        impl $type_name { pub(crate) const fn get(self) -> usize { self.0 } }
    )+};
}
count_with_zero!(TrainingStepCount, GeneratedTokenCount);

#[derive(Clone, Copy, Debug)]
pub(crate) struct PromptText(&'static str);
// PromptText хранит строку программы с временем жизни 'static.
// FromStr принимает &str с произвольным временем жизни; поэтому используем TryFrom<&'static str>.
#[allow(clippy::try_from_instead_of_from_str)]
impl TryFrom<&'static str> for PromptText {
    type Error = Error;
    fn try_from(raw_value: &'static str) -> Result<Self> {
        if raw_value.is_empty() {
            candle_core::bail!("Prompt не может быть пустым");
        }
        Ok(Self(raw_value))
    }
}
impl PromptText {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}

// LearningRate, WeightDecay и коэффициенты AdamW отклоняют NaN, бесконечности и значения вне своих диапазонов.
macro_rules! float_setting {
    ($type_name:ident, $validation_predicate:expr, $error_message:literal) => {
        #[derive(Clone, Copy, Debug)]
        pub(crate) struct $type_name(f64);
        impl TryFrom<f64> for $type_name {
            type Error = Error;
            fn try_from(raw_value: f64) -> Result<Self> {
                if !raw_value.is_finite() || !($validation_predicate)(raw_value) {
                    candle_core::bail!($error_message);
                }
                Ok(Self(raw_value))
            }
        }
        impl $type_name {
            pub(crate) const fn get(self) -> f64 {
                self.0
            }
        }
    };
}
float_setting!(
    LearningRate,
    |learning_rate_value: f64| -> bool { learning_rate_value > 0.0 },
    "LearningRate: нужно конечное число >0"
);
float_setting!(
    WeightDecay,
    |weight_decay_value: f64| -> bool { weight_decay_value >= 0.0 },
    "WeightDecay: нужно конечное число >=0"
);
float_setting!(
    GradientAverageDecay,
    |gradient_decay_coefficient: f64| -> bool { (0.0..1.0).contains(&gradient_decay_coefficient) },
    "AdamBeta1: нужно число в [0,1)"
);
float_setting!(
    SquaredGradientAverageDecay,
    |squared_gradient_decay_coefficient: f64| -> bool { (0.0..1.0).contains(&squared_gradient_decay_coefficient) },
    "AdamBeta2: нужно число в [0,1)"
);
float_setting!(
    NumericalStabilityEpsilon,
    |epsilon_value: f64| -> bool { epsilon_value > 0.0 },
    "AdamEpsilon: нужно конечное число >0"
);

#[derive(Debug)]
pub(crate) struct AdamWSettings {
    pub(crate) learning_rate: LearningRate,
    pub(crate) weight_decay: WeightDecay,
    pub(crate) gradient_average_decay: GradientAverageDecay,
    pub(crate) squared_gradient_average_decay: SquaredGradientAverageDecay,
    pub(crate) numerical_stability_epsilon: NumericalStabilityEpsilon,
}
impl AdamWSettings {
    // Переводим полные имена в поля Candle: learning_rate -> lr, gradient_average_decay -> beta1,
    // squared_gradient_average_decay -> beta2, numerical_stability_epsilon -> eps.
    // Возвращаем отдельный ParamsAdamW; его изменение не меняет значения в AdamWSettings.
    pub(crate) fn to_candle_parameters(&self) -> ParamsAdamW {
        ParamsAdamW {
            lr: self.learning_rate.get(),
            weight_decay: self.weight_decay.get(),
            beta1: self.gradient_average_decay.get(),
            beta2: self.squared_gradient_average_decay.get(),
            eps: self.numerical_stability_epsilon.get(),
        }
    }
}

pub(crate) struct Settings {
    pub(crate) block_size: BlockSize,
    pub(crate) embedding_dimension: EmbeddingDimension,
    pub(crate) attention_head_count: AttentionHeadCount,
    pub(crate) transformer_block_count: TransformerBlockCount,
    pub(crate) dropout_probability: DropoutProbability,
    pub(crate) batch_size: BatchSize,
    pub(crate) training_step_count: TrainingStepCount,
    pub(crate) evaluation_interval: EvaluationInterval,
    pub(crate) evaluation_batch_count: EvaluationBatchCount,
    pub(crate) adamw_settings: AdamWSettings,
    pub(crate) random_seed: u64, // Начальное состояние генераторов; допустим любой u64, в том числе 0.
    pub(crate) prompt_text: PromptText,
    pub(crate) generated_token_count: GeneratedTokenCount,
    pub(crate) demonstrate_only: bool, // true: выполнить только демонстрации; false: также обучить модель и сгенерировать текст.
}
impl Settings {
    pub(crate) fn validate(&self) -> Result<()> {
        // Проверяем связи между полями: embedding_dimension / attention_head_count
        // должно дать целую ширину головы, а размеры тензоров не должны переполнить индексы.
        if !self.embedding_dimension().is_multiple_of(self.attention_head_count()) {
            candle_core::bail!("embedding_dimension должно делиться на attention_head_count без остатка");
        }
        if u32::try_from(self.block_size()).is_err() {
            candle_core::bail!("block_size не помещается в u32 для индексов позиций");
        }
        if self.embedding_dimension().checked_mul(4).is_none() || self.batch_size().checked_mul(self.block_size()).is_none() {
            candle_core::bail!("Размеры GPT или batch вызывают переполнение usize");
        }
        Ok(())
    }
}

macro_rules! getters {
    ($($field_name:ident: $return_type:ty),+ $(,)?) => {impl Settings {$ (
        pub(crate) fn $field_name(&self) -> $return_type { self.$field_name.get() }
    )+}};
}
getters!(block_size: usize, embedding_dimension: usize, attention_head_count: usize, transformer_block_count: usize, dropout_probability: f64,
    batch_size: usize, training_step_count: usize, evaluation_interval: usize, evaluation_batch_count: usize,
    prompt_text: &'static str, generated_token_count: usize);
impl Settings {
    pub(crate) fn optimizer_parameters(&self) -> ParamsAdamW {
        self.adamw_settings.to_candle_parameters()
    }
    pub(crate) fn random_seed(&self) -> u64 {
        self.random_seed
    }
    pub(crate) fn demonstrate_only(&self) -> bool {
        self.demonstrate_only
    }
}

#[cfg(test)]
pub(crate) fn test_settings() -> Result<Settings> {
    Ok(Settings {
        block_size: BlockSize::try_from(4)?,
        embedding_dimension: EmbeddingDimension::try_from(8)?,
        attention_head_count: AttentionHeadCount::try_from(2)?,
        transformer_block_count: TransformerBlockCount::try_from(2)?,
        dropout_probability: DropoutProbability::try_from(0.2)?,
        batch_size: BatchSize::try_from(2)?,
        training_step_count: TrainingStepCount::from(60),
        evaluation_interval: EvaluationInterval::try_from(10)?,
        evaluation_batch_count: EvaluationBatchCount::try_from(2)?,
        adamw_settings: AdamWSettings {
            learning_rate: LearningRate::try_from(0.001)?,
            weight_decay: WeightDecay::try_from(0.01)?,
            gradient_average_decay: GradientAverageDecay::try_from(0.9)?,
            squared_gradient_average_decay: SquaredGradientAverageDecay::try_from(0.999)?,
            numerical_stability_epsilon: NumericalStabilityEpsilon::try_from(1e-8)?,
        },
        random_seed: 1337,
        prompt_text: PromptText::try_from("\n")?,
        generated_token_count: GeneratedTokenCount::from(20),
        demonstrate_only: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scalar_types_reject_invalid_boundaries() {
        macro_rules! positive { ($($parameter_type:ty),+) => {$ (
            assert!(<$parameter_type>::try_from(0usize).is_err());
            assert_eq!(<$parameter_type>::try_from(1usize).unwrap().get(),1);
        )+}; }
        positive!(
            BlockSize,
            EmbeddingDimension,
            AttentionHeadCount,
            TransformerBlockCount,
            BatchSize,
            EvaluationInterval,
            EvaluationBatchCount
        );
        [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.1, 1.0]
            .into_iter()
            .for_each(|invalid_value: f64| {
                assert!(DropoutProbability::try_from(invalid_value).is_err());
                assert!(GradientAverageDecay::try_from(invalid_value).is_err());
                assert!(SquaredGradientAverageDecay::try_from(invalid_value).is_err());
            });
        assert_eq!(DropoutProbability::try_from(0.0).unwrap().get(), 0.0);
        assert!(DropoutProbability::try_from(1.0 - f64::EPSILON).is_ok());
        [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0, 0.0]
            .into_iter()
            .for_each(|invalid_value: f64| {
                assert!(LearningRate::try_from(invalid_value).is_err());
                assert!(NumericalStabilityEpsilon::try_from(invalid_value).is_err());
            });
        [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0].into_iter().for_each(|invalid_value: f64| {
            assert!(WeightDecay::try_from(invalid_value).is_err());
        });
        assert!(WeightDecay::try_from(0.0).is_ok());
        assert!(PromptText::try_from("").is_err());
        assert_eq!(PromptText::try_from("\n").unwrap().get(), "\n");
    }
    #[test]
    fn settings_reject_invalid_combinations_and_keep_zero_steps() -> Result<()> {
        assert!(
            Settings {
                embedding_dimension: EmbeddingDimension::try_from(7)?,
                ..test_settings()?
            }
            .validate()
            .is_err()
        );
        assert!(
            Settings {
                block_size: BlockSize::try_from(usize::MAX)?,
                ..test_settings()?
            }
            .validate()
            .is_err()
        );
        assert!(
            Settings {
                batch_size: BatchSize::try_from(usize::MAX)?,
                ..test_settings()?
            }
            .validate()
            .is_err()
        );
        let settings: Settings = Settings {
            training_step_count: TrainingStepCount::from(0),
            generated_token_count: GeneratedTokenCount::from(0),
            random_seed: 0,
            ..test_settings()?
        };
        settings.validate()?;
        assert_eq!(settings.training_step_count(), 0);
        assert_eq!(settings.generated_token_count(), 0);
        assert_eq!(settings.random_seed(), 0);
        let mut optimizer_parameter_copy: ParamsAdamW = settings.optimizer_parameters();
        optimizer_parameter_copy.lr = -1.0;
        assert!(optimizer_parameter_copy.lr < 0.0 && settings.optimizer_parameters().lr > 0.0);
        Ok(())
    }
}
