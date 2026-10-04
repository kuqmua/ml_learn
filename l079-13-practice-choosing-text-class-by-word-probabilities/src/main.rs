// Урок 079. Определять класс текста по сумме логарифмов частот слов со сглаживанием.
// Строим словарь по обучающим текстам и учитываем случай, когда все слова нового текста незнакомы.

fn main() {
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
            let mut logarithm_series_sum: f64 = 0.0;
            for term_index in 0..40 {
                logarithm_series_sum += term / (2 * term_index + 1) as f64;
                term *= ratio_squared;
            }
            2.0 * logarithm_series_sum
        }

        approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(scaled)
            + power_of_two as f64
                * approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(2.0)
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
        let mut scores: [f64; 2] = [0.0; 2];
        for (class, score) in scores.iter_mut().enumerate() {
            let class_documents: Vec<&(&str, bool)> = training_examples
                .iter()
                .filter(|(_, target)| *target == (class == 1))
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
                (class_documents.len() as f64 + 1.0) / (training_examples.len() as f64 + 2.0),
            );
            for text_unit in text.split_whitespace() {
                if known_text_units.contains(text_unit) {
                    *score += approximate_natural_log_by_scaling_and_summing_odd_powers(
                        (*text_unit_counts.get(text_unit).unwrap_or(&0) as f64 + 1.0)
                            / (total_text_units + known_text_units.len()) as f64,
                    );
                }
            }
        }
        (scores[1] > scores[0], scores)
    }

    let training_examples: [(&str, bool); 4] = [
        ("хороший фильм", true),
        ("отличный фильм", true),
        ("плохой фильм", false),
        ("ужасный фильм", false),
    ];
    let _ = &(choose_class_by_summed_log_probabilities_of_words(
        &training_examples,
        "хороший отличный",
    ));
    let _ = &(choose_class_by_summed_log_probabilities_of_words(&training_examples, "неизвестное"));

    // Выполняем вычисления из примера.
    let _ = training_examples;

    for (text, expected) in [("хороший отличный", true), ("плохой ужасный", false)]
    {
        let (class, scores) =
            choose_class_by_summed_log_probabilities_of_words(&training_examples, text);
        println!("Текст={text:?}: оценки классов={scores:?}, выбран={class}");
        assert_eq!(class, expected);
    }
    let (_, unknown_scores) =
        choose_class_by_summed_log_probabilities_of_words(&training_examples, "неизвестное");
    println!("Только незнакомые слова: {unknown_scores:?}; остались исходные оценки классов");
    assert_eq!(unknown_scores[0], unknown_scores[1]);
}

// Чему учит этот урок:
// Учимся определять класс текста по сумме логарифмов частот слов со сглаживанием.
// Строим словарь по обучающим текстам и учитываем случай, когда все слова нового текста незнакомы.
