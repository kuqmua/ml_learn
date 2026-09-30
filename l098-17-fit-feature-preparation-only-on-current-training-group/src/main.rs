// Урок 17.4. Расчёт подготовки признаков только по текущей обучающей группе.
// Связь с принятой терминологией: Утечка при подготовке признаков между блоками кросс-валидации.
// Зачем здесь эта тема: Разделить строки недостаточно, если нормализация обучена сразу на всех
//   блоках.
// Почему код устроен так: Переобучаем преобразование внутри каждого train-блока, не используя
//   соответствующий validation-блок.
// Представь: Среднее для нормализации первого fold нельзя считать с участием его проверочных строк.
//
// Что изучаем: Утечка при подготовке признаков.
// Зачем это нужно: В каждом fold среднее и другие статистики вычисляем только по обучающей части, а затем
// применяем к validation.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let training_data: [f64; 3] = [1.0, 2.0, 3.0];
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    let validation: [f64; 1] = [100.0];
    let training_mean: f64 = training_data.iter().sum::<f64>() / training_data.len() as f64;
    let validation_centered: f64 = validation[0] - training_mean;

    plot_training_mean_and_centered_validation_value(training_mean, validation_centered);
}

// Строим график по результатам урока.
fn plot_training_mean_and_centered_validation_value(training_mean: f64, validation_centered: f64) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Центрирование по train",
        "значение",
        &[
            ("train mean", training_mean),
            ("validation centered", validation_centered),
        ],
    )
    .expect("не удалось сохранить график");
}
