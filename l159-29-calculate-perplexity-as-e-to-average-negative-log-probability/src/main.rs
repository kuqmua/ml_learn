// Урок 29.3. Неопределённость языковой модели (перплексия): e в степени среднего отрицательного логарифма вероятности.
// Связь с принятой терминологией: Perplexity из средней кросс энтропии языковой модели.
// Зачем здесь эта тема: Среднюю кросс энтропию трудно читать как число вариантов продолжения.
// Почему код устроен так: Возводим e в среднюю ошибку и получаем perplexity на той же
//   последовательности.
// Представь: Одинаковая средняя ошибка может читаться как эффективное число возможных продолжений
//   через exp.
//
// Что изучаем: Perplexity.
// Зачем это нужно: Perplexity — экспонента средней cross-entropy; меньшая величина означает лучшее
// вероятностное предсказание.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let predicted_probability_error: f64 = 0.7;
    let mut term: f64 = 1.0;
    let mut _effective_choice_count: f64 = 1.0;
    for order in 1..=30 {
        term *= predicted_probability_error / order as f64;
        _effective_choice_count += term;
    }

    plot_perplexity_as_e_to_average_negative_log_probability();
}

// Строим график по результатам урока.
fn plot_perplexity_as_e_to_average_negative_log_probability() {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Perplexity",
        "cross-entropy",
        "perplexity",
        &[lesson_visualization::Series {
            name: "exp(loss)",

            points: &(0..=40)
                .map(|plot_step_index| {
                    let loss_value: f64 = plot_step_index as f64 / 10.0;
                    (loss_value, loss_value.exp())
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
