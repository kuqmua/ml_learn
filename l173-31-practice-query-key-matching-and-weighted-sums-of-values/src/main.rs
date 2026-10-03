// Урок 31.7. Практика: сравнение запросов с ключами и взвешенное сложение значений.
// Зачем здесь эта тема: Полное внимание связывает Q, K, V, масштабирование, маску и взвешенную
//   сумму.
// Почему код устроен так: На нескольких коротких векторах проверяем отдельно оценки, веса и выход.
// Представь: Запрос сравнивается с ключами, веса выбирают важные позиции, а их V складываются с
//   этими весами.
//
// Что повторяем вместе: Q, K, V, масштабированная сумма после попарного умножения координат, softmax, causal mask.
// Зачем это нужно: Внимание взвешивает информацию от разных позиций; causal mask запрещает токену видеть
//   будущее.
// Что показывает программа: Задаём короткую последовательность двумерных векторов. Считаем маскированное
//   внимание и получаем новый вектор для каждой позиции. Печатаем веса, чтобы увидеть запрет доступа к
//   будущим токенам.
// Что проверить при изменении примера: Проверь суммы весов, форму выхода и отсутствие доступа к будущим
//   позициям при causal mask.
// Дополнительная практика: Реализуй single-head attention для короткой последовательности без готового слоя.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;

fn main() {
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

    let (_attended_output, attention_weights): ([[f64; 2]; 3], [[f64; 3]; 3]) =
        (|| -> ([[f64; 2]; 3], [[f64; 3]; 3]) {
            let sequence: [[f64; 2]; 3] = [[1., 0.], [0., 1.], [1., 1.]];
            let queries: &[[f64; 2]; 3] = &sequence;
            let keys: &[[f64; 2]; 3] = &sequence;
            let values: &[[f64; 2]; 3] = &sequence;
            let past_only_attention: bool = true;
            let mut outputs: [[f64; 2]; 3] = [[0.0; 2]; 3];
            let mut weights: [[f64; 3]; 3] = [[0.0; 3]; 3];
            for (query_index, query) in queries.iter().enumerate() {
                let attention_weights: Vec<f64> = {
                    let raw_model_scores: Vec<f64> = keys
                        .iter()
                        .enumerate()
                        .map(|(key_index, key)| {
                            if past_only_attention && key_index > query_index {
                                f64::NEG_INFINITY
                            } else {
                                multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(query, key).unwrap()
                                    / (|| -> f64 {
                                        let value: f64 = 2.0;
                                        assert!(value >= 0.0, "корень из отрицательного числа");
                                        if value == 0.0 {
                                            return 0.0;
                                        }
                                        let mut estimate: f64 =
                                            if value > 1.0 { value } else { 1.0 };
                                        for _ in 0..80 {
                                            estimate = (estimate + value / estimate) / 2.0;
                                        }
                                        estimate
                                    })()
                            }
                        })
                        .collect();
                    (|| -> Vec<f64> {
                        let raw_model_scores: &[f64] = &raw_model_scores;
                        let mut maximum_raw_model_score: f64 = f64::NEG_INFINITY;
                        for &raw_model_score in raw_model_scores {
                            if raw_model_score > maximum_raw_model_score {
                                maximum_raw_model_score = raw_model_score;
                            }
                        }
                        let mut exponentials: Vec<f64> = Vec::with_capacity(raw_model_scores.len());
                        let mut normalizer: f64 = 0.0;
                        for &raw_model_score in raw_model_scores {
                            let exponential_value: f64 =
                                approximate_e_to_power_by_summing_power_over_factorial_terms(
                                    raw_model_score - maximum_raw_model_score,
                                );
                            exponentials.push(exponential_value);
                            normalizer += exponential_value;
                        }
                        for exponential_value in &mut exponentials {
                            *exponential_value /= normalizer;
                        }
                        exponentials
                    })()
                };
                let mut attended_vec: [f64; 2] = [0.0, 0.0];
                for value_index in 0..values.len() {
                    attended_vec[0] += attention_weights[value_index] * values[value_index][0];
                    attended_vec[1] += attention_weights[value_index] * values[value_index][1];
                }
                outputs[query_index] = attended_vec;
                weights[query_index] = attention_weights
                    .try_into()
                    .expect("ожидалось по одному весу на каждый из трёх ключей");
            }
            (outputs, weights)
        })();

    plot_weights_assigned_to_current_and_past_positions(attention_weights);
}

// Строим график по результатам урока.
fn plot_weights_assigned_to_current_and_past_positions(attention_weights: [[f64; 3]; 3]) {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Причинные веса внимания",
        &attention_weights
            .iter()
            .map(|row| row.to_vec())
            .collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
}
