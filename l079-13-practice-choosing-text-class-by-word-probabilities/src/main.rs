// Урок 13.4. Практика: выбор класса текста по вероятностям слов.
// Связь с принятой терминологией: Наивный Байес с априорными вероятностями и сглаживанием.
// Зачем здесь эта тема: Классификатор Байеса должен соединить prior, условные вероятности и
//   сглаживание.
// Почему код устроен так: Складываем логарифмы вместо умножения малых вероятностей и проверяем
//   неизвестное слово.
// Представь: Много маленьких вероятностей трудно перемножать; сумма их логарифмов сохраняет порядок
//   сравнения классов.
//
// Что повторяем вместе: условная независимость, априорные вероятности, сглаживание Лапласа.
// Зачем это нужно: Наивный Байес сравнивает вероятности слов в классах; логарифмы превращают результат умножения
//   малых чисел в сумму.
// Что показывает программа: Составляем маленький размеченный корпус положительных и отрицательных текстов.
//   Считаем логарифмические оценки классов для известных слов. Отдельно проверяем, что неизвестное слово не
//   ломает классификатор.
// Что проверить при изменении примера: Проверь неизвестные слова и класс с редкими токенами; считай
//   вероятности в log-пространстве.
// Дополнительная практика: Обучи мультиномиальный классификатор коротких текстов по счётчикам слов.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let training_examples: [(&str, bool); 4] = [
        ("хороший фильм", true),
        ("отличный фильм", true),
        ("плохой фильм", false),
        ("ужасный фильм", false),
    ];

    /// ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1).
    /// Учебный аналог `f64::ln`; показывает вычисление ряда и может работать медленнее.
    /// Здесь неположительный вход вызывает panic, а `ln` возвращает NaN или −∞.
    /// Натуральный логарифм: приводим аргумент к [1, 2), суммируем ряд нечётных степеней и возвращаем вклад степеней двойки.
    fn approximate_natural_log_by_scaling_and_summing_odd_powers(value: f64) -> f64 {
        assert!(
            value > 0.0,
            "логарифм определён только для положительных чисел"
        );
        if value == f64::INFINITY {
            return f64::INFINITY;
        }
        let mut scaled: f64 = value;
        let mut power_of_two: i32 = 0i32;
        while scaled >= 2.0 {
            scaled /= 2.0;
            power_of_two += 1;
        }
        while scaled < 1.0 {
            scaled *= 2.0;
            power_of_two -= 1;
        }
        /// Ряд для ln(x): 2·(t + t³/3 + t⁵/5 + …), где t = (x−1)/(x+1).
        fn approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
            value: f64,
        ) -> f64 {
            let ratio: f64 = (value - 1.0) / (value + 1.0);
            let ratio_squared: f64 = ratio * ratio;
            let mut term: f64 = ratio;
            let mut result: f64 = 0.0;
            for term_index in 0..40 {
                result += term / (2 * term_index + 1) as f64;
                term *= ratio_squared;
            }
            2.0 * result
        }
        let logarithm_of_two: f64 =
            approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(2.0);
        approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(scaled)
            + power_of_two as f64 * logarithm_of_two
    }

    /// Наивный Байес: для каждого класса складываем логарифм его вероятности и логарифмы вероятностей слов со сглаживанием +1.
    fn choose_class_by_summed_log_probabilities_of_words(
        training_examples: &[(&str, bool)],

        text: &str,
    ) -> (bool, [f64; 2]) {
        let known_text_units: std::collections::HashSet<&str> = training_examples
            .iter()
            .flat_map(|(document_text, _)| document_text.split_whitespace())
            .collect();
        let mut scores: [f64; 2] = [0.; 2];
        for (class, score) in scores.iter_mut().enumerate() {
            let class_documents: Vec<&(&str, bool)> = training_examples
                .iter()
                .filter(|(_, label)| *label == (class == 1))
                .collect();
            let mut text_unit_counts: std::collections::HashMap<&str, usize> =
                std::collections::HashMap::new();
            let mut total_text_units: usize = 0;
            for (document_text, _) in &class_documents {
                for text_unit in document_text.split_whitespace() {
                    *text_unit_counts.entry(text_unit).or_insert(0usize) += 1;
                    total_text_units += 1;
                }
            }
            *score = approximate_natural_log_by_scaling_and_summing_odd_powers(
                (class_documents.len() as f64 + 1.) / (training_examples.len() as f64 + 2.),
            );
            for text_unit in text.split_whitespace() {
                if known_text_units.contains(text_unit) {
                    *score += approximate_natural_log_by_scaling_and_summing_odd_powers(
                        (*text_unit_counts.get(text_unit).unwrap_or(&0) as f64 + 1.)
                            / (total_text_units + known_text_units.len()) as f64,
                    );
                }
            }
        }
        (scores[1] > scores[0], scores)
    }

    let _ = &(choose_class_by_summed_log_probabilities_of_words(
        &training_examples,
        "хороший отличный",
    ));
    let _ = &(choose_class_by_summed_log_probabilities_of_words(&training_examples, "неизвестное"));

    plot_number_of_training_documents_in_each_class(training_examples);
}

// Строим график по результатам урока.
fn plot_number_of_training_documents_in_each_class(training_examples: [(&str, bool); 4]) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Классы обучающих документов",
        "число текстов",
        &[
            (
                "положительные",
                training_examples.iter().filter(|(_, class)| *class).count() as f64,
            ),
            (
                "отрицательные",
                training_examples
                    .iter()
                    .filter(|(_, class)| !*class)
                    .count() as f64,
            ),
        ],
    )
    .expect("не удалось сохранить график");
}
