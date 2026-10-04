// Урок 37.2. Сжатие весов модели: масштабирование и округление до восьмибитных целых чисел.
// Зачем здесь эта тема: Большие веса занимают память и замедляют перенос модели.
// Почему код устроен так: Отображаем вещественные веса в INT8 с масштабом и измеряем ошибку
//   восстановления.
// Представь: Число 0,25 хранится как целый код и масштаб; после восстановления оно может немного
//   отличаться.
// Масштабируем веса в i8 и измеряем погрешность после восстановления.

fn main() {
    let weights: [f64; 5] = [-1.0, -0.5, 0.0, 0.25, 1.0];

    let weight_value_per_integer_step_setting_rounding_error_bound: f64 =
        weights.iter().copied().map(f64::abs).fold(0.0, f64::max) / 127.0;

    assert!(
        weights
            .iter()
            .zip(
                &weights
                    .map(|weight_value| (weight_value
                        / weight_value_per_integer_step_setting_rounding_error_bound)
                        .round()
                        .clamp(-127.0, 127.0) as i8)
                    .map(
                        |reduced_precision_weight| f64::from(reduced_precision_weight)
                            * weight_value_per_integer_step_setting_rounding_error_bound
                    )
            )
            .map(|(value1, value2)| (value1 - value2).abs())
            .fold(0.0, f64::max)
            <= weight_value_per_integer_step_setting_rounding_error_bound / 2.0 + 1e-12
    );
}

// Чему учит этот урок:
// Учимся хранить веса приближённо в восьмибитных целых числах с общим масштабом.
// Восстанавливаем значения и проверяем границу ошибки округления, чтобы видеть цену сокращения
// размера представления.
