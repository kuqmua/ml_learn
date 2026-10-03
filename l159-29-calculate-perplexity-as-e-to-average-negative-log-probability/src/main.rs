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
    let average_negative_log_correct_token_probability: f64 = 0.7;
    let mut term: f64 = 1.0;
    let mut _perplexity_as_effective_choice_count_where_1_means_certainty_on_correct_token_and_larger_means_worse: f64 = 1.0;
    for order in 1..=30 {
        term *= average_negative_log_correct_token_probability / order as f64;
        _perplexity_as_effective_choice_count_where_1_means_certainty_on_correct_token_and_larger_means_worse += term;
    }

    plot_perplexity_as_e_to_average_negative_log_probability();
}

// Строим график по результатам урока.
fn plot_perplexity_as_e_to_average_negative_log_probability() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Perplexity",
        "cross-entropy",
        "perplexity",
        &[lesson_visualization::Series {
            name: "exp(loss)",

            points: &(0..=40)
                .map(|plot_step_index| {
                    let average_negative_log_correct_token_probability: f64 =
                        plot_step_index as f64 / 10.0;
                    (
                        average_negative_log_correct_token_probability,
                        average_negative_log_correct_token_probability.exp(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
