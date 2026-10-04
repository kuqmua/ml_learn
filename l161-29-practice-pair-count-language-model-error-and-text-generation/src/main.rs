// Урок 29.5. Практика: модель текста по частотам пар, оценка ошибки и генерация продолжения.
// Зачем здесь эта тема: Биграммная модель позволяет увидеть полный цикл: счётчики, вероятности,
//   loss и генерация.
// Почему код устроен так: Оставляем контекст длиной один, чтобы каждый шаг можно было проверить по
//   таблице частот.
// Представь: Из частот пар получаем вероятности, по ним считаем ошибку, затем выбираем следующий
//   токен.
//
// Что повторяем вместе: предсказание следующего токена, cross-entropy, perplexity, sampling.
// Зачем это нужно: Биграммная модель оценивает следующее слово по предыдущему; perplexity измеряет качество
//   вероятностного прогноза.
// Что показывает программа: Задаём маленький корпус для подсчёта биграмм. Считаем частоты переходов и
//   словарь возможных следующих токенов. Сравниваем perplexity на обучающих и новых сочетаниях слов.
// Что проверить при изменении примера: Сравни train/validation perplexity и покажи влияние temperature и
//   seed.
// Дополнительная практика: Обучи маленькую n-gram модель или tiny decoder на игрушечном корпусе; реализуй
//   генерацию.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let training_sentences: [&str; 3] = ["кот спит", "кот ест", "пёс спит"];

    let (bigram_counts, known_text_units): (std::collections::BTreeMap<(String, String), usize>, std::collections::BTreeSet<String>) =

        (|| -> (std::collections::BTreeMap<(String, String), usize>, std::collections::BTreeSet<String>) {
            let sentences: &[&str] = &training_sentences;
            let mut counts: std::collections::BTreeMap<(String, String), usize> = std::collections::BTreeMap::new();
            let mut known_text_units: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            for sentence in sentences {
                let mut previous: &str = "<s>";
                for word in sentence.split_whitespace().chain(["</s>"]) {
                    *counts.entry((previous.into(), word.into())).or_insert(0) += 1;
                    known_text_units.insert(word.into());
                    previous = word;
                }
            }
            (counts, known_text_units)
        })();

    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    /// Экспонента eˣ: ряд Тейлора 1 + x + x²/2! + x³/3! + … с уменьшением аргумента и восстановлением масштаба.
    fn approximate_e_to_power_by_summing_power_over_factorial_terms(value: f64) -> f64 {
        if value == f64::NEG_INFINITY || value < -745.0 {
            return 0.0;
        }
        if value == f64::INFINITY || value > 709.0 {
            return f64::INFINITY;
        }
        if value < 0.0 {
            return 1.0 / approximate_e_to_power_by_summing_power_over_factorial_terms(-value);
        }
        let mut reduced: f64 = value;
        let mut halving_count: i32 = 0;
        while reduced > 0.5 {
            reduced /= 2.0;
            halving_count += 1;
        }
        let mut term: f64 = 1.0;
        let mut exponential_approximation: f64 = 1.0;
        for term_index in 1..=30 {
            term *= reduced / term_index as f64;
            exponential_approximation += term;
        }
        for _ in 0..halving_count {
            exponential_approximation *= exponential_approximation;
        }
        exponential_approximation
    }

    /// Сглаженная вероятность следующего токена: (число пары + 1) / (число переходов из контекста + размер словаря).
    fn calc_next_token_probability_as_pair_count_plus_one_over_context_count_plus_vocabulary_size(
        counts: &std::collections::BTreeMap<(String, String), usize>,

        known_text_units: &std::collections::BTreeSet<String>,

        previous_text_unit: &str,

        next_text_unit: &str,
    ) -> f64 {
        let mut count_after_previous_text_unit: usize = 0;
        for ((previous_context, _), &text_unit_count) in counts {
            if previous_context == previous_text_unit {
                count_after_previous_text_unit += text_unit_count;
            }
        }
        (*counts
            .get(&(previous_text_unit.into(), next_text_unit.into()))
            .unwrap_or(&0) as f64
            + 1.0)
            / (count_after_previous_text_unit + known_text_units.len()) as f64
    }

    /// Перплексия: e в степени среднего отрицательного логарифма вероятности следующего слова, включая конец строки.
    fn calc_perplexity_as_e_to_average_neg_log_next_word_probability(
        sentences: &[&str],

        counts: &std::collections::BTreeMap<(String, String), usize>,

        known_text_units: &std::collections::BTreeSet<String>,
    ) -> f64 {
        let (mut text_unit_count, mut neg_log_likelihood): (i32, f64) = (0, 0.0);
        for sentence in sentences {
            let mut previous_text_unit: &str = "<s>";
            for word in sentence.split_whitespace().chain(["</s>"]) {
                neg_log_likelihood -= (|| -> f64 {
                    let value: f64 =
                        calc_next_token_probability_as_pair_count_plus_one_over_context_count_plus_vocabulary_size(

                            counts,

                            known_text_units,

                            previous_text_unit,

                            word,
                        );
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

                    approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        scaled,
                    ) + power_of_two as f64
                        * approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                            2.0,
                        )
                })();
                text_unit_count += 1;
                previous_text_unit = word;
            }
        }
        approximate_e_to_power_by_summing_power_over_factorial_terms(
            neg_log_likelihood / text_unit_count as f64,
        )
    }

    let _ = (
        &(calc_perplexity_as_e_to_average_neg_log_next_word_probability(
            &training_sentences,
            &bigram_counts,
            &known_text_units,
        )),
        &(calc_perplexity_as_e_to_average_neg_log_next_word_probability(
            &["пёс ест"],
            &bigram_counts,
            &known_text_units,
        )),
    );
    let mut previous_text_unit: &str = "<s>";
    let mut generated_text_units: Vec<&str> = vec![];
    for _ in 0..5 {
        let next_text_unit: &String = known_text_units

            .iter()

            .max_by(|candidate1, candidate2| {
                calc_next_token_probability_as_pair_count_plus_one_over_context_count_plus_vocabulary_size(

                    &bigram_counts,

                    &known_text_units,

                    previous_text_unit,

                    candidate1,
                )

                .total_cmp(
                    &calc_next_token_probability_as_pair_count_plus_one_over_context_count_plus_vocabulary_size(

                        &bigram_counts,

                        &known_text_units,

                        previous_text_unit,

                        candidate2,
                    ),
                )
            })

            .unwrap();
        if next_text_unit == "</s>" {
            break;
        }
        generated_text_units.push(next_text_unit.as_str());
        previous_text_unit = next_text_unit;
    }
    let _ = &(generated_text_units.join(" "));

    // Выполняем вычисления из примера.
    let _ = bigram_counts;
}

// Чему учит этот урок:
// Учимся строить модель соседних слов по счётчикам, сглаживать вероятности и оценивать
// последовательности.
// Генерируем продолжение выбором самого вероятного слова, учитывая отдельный маркер завершения.
