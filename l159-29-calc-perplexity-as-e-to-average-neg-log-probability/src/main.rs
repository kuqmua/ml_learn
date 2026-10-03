// Урок 159. Переводим среднюю ошибку предсказания текста в условное число вариантов выбора.
// Для этого возводим число e примерно равное 2.718 в степень, равную средней ошибке.
// Результат 1 соответствует полной уверенности в правильных продолжениях; больше — хуже.
// Если модель каждый раз даёт правильному продолжению вероятность 1/4, результат равен 4.
// Это удобная шкала ошибки, а не буквальное число слов в словаре.

fn main() {
    let average_neg_log_correct_token_probability: f64 = 0.7;
    let mut term: f64 = 1.0;
    let mut _perplexity_as_effective_choice_count_where_1_means_certainty_on_correct_token_and_larger_means_worse: f64 = 1.0;
    for order in 1..=30 {
        term *= average_neg_log_correct_token_probability / order as f64;
        _perplexity_as_effective_choice_count_where_1_means_certainty_on_correct_token_and_larger_means_worse += term;
    }

    plot_perplexity_as_e_to_average_neg_log_probability();
}

// Строим график по результатам урока.
fn plot_perplexity_as_e_to_average_neg_log_probability() {
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
                    let average_neg_log_correct_token_probability: f64 =
                        plot_step_index as f64 / 10.0;
                    (
                        average_neg_log_correct_token_probability,
                        average_neg_log_correct_token_probability.exp(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
