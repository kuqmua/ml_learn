// Урок 32.5. Практика: сбор контекста, прибавление входа и преобразование координат.
// Связь с принятой терминологией: Блок Transformer с вниманием, остаточными связями и полносвязным слоем.
// Зачем здесь эта тема: Transformer объединяет обмен между токенами и преобразование каждого токена
//   при стабильном масштабе.
// Почему код устроен так: Собираем внимание, прибавление входа, LayerNorm и feed-forward по порядку и
//   проверяем форму после каждого шага.
// Представь: Токен сначала получает контекст через attention, затем сохраняет вход через прибавление входа и
//   преобразуется дальше.
//
// Что повторяем вместе: self-attention, прибавление входа, нормализацию слоя, feed-forward.
// Зачем это нужно: Блок Transformer сочетает внимание, остаточные связи, нормализацию и преобразование
//   каждого токена.
// Что показывает программа: Создаём вход из двух токенов с двумерными признаками. Пропускаем его через
//   attention, прибавление входа, нормализацию и feed-forward.
// Что проверить при изменении примера: Проверь сохранение формы, отсутствие NaN и детерминированный forward
//   при фиксированных весах.
// Дополнительная практика: Собери один блок на малых тензорах и опиши порядок операций.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results;

fn main() {
    let input_values: [[f64; 2]; 2] = [[1., 0.], [0., 1.]];

    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calculate_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        value * value
    }

    /// Корень через итерацию Ньютона: x_(n+1) = (x_n + value / x_n) / 2.
    /// Учебный аналог `f64::sqrt`; показывает алгоритм и может работать медленнее.
    /// Здесь отрицательный вход вызывает panic, а `sqrt` возвращает NaN.
    /// Метод Ньютона для корня: повторяем estimate = (estimate + value / estimate) / 2.
    fn approximate_square_root_by_repeated_averaging(value: f64) -> f64 {
        assert!(value >= 0.0, "корень из отрицательного числа");
        if value == 0.0 {
            return 0.0;
        }
        let mut estimate: f64 = if value > 1.0 { value } else { 1.0 };
        for _ in 0..80 {
            estimate = (estimate + value / estimate) / 2.0;
        }
        estimate
    }

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

    /// Выбираем большее из двух чисел для формул softmax, log-loss и Q-learning.
    /// Аналог `first_number.max(second_number)` для обычных чисел; при NaN результат может отличаться.
    fn choose_larger_number(first_number: f64, second_number: f64) -> f64 {
        if first_number > second_number {
            first_number
        } else {
            second_number
        }
    }

    /// Нормализация слоя (LayerNorm): из координат вычитаем среднее и делим на sqrt(среднее квадратов отклонений + epsilon).
    fn normalize_coordinates_by_subtracting_mean_and_dividing_by_root_mean_square(
        input_values: [f64; 2],
    ) -> [f64; 2] {
        let mean: f64 = (input_values[0] + input_values[1]) / 2.;
        let variance: f64 =
            (calculate_square_by_multiplying_number_by_itself(input_values[0] - mean)
                + calculate_square_by_multiplying_number_by_itself(input_values[1] - mean))
                / 2.;
        [
            (input_values[0] - mean)
                / approximate_square_root_by_repeated_averaging(variance + 1e-5),
            (input_values[1] - mean)
                / approximate_square_root_by_repeated_averaging(variance + 1e-5),
        ]
    }

    let transformer_output: [[f64; 2]; 2] = std::array::from_fn(|text_unit_index| {
        let query = input_values[text_unit_index];
        let raw_model_scores: Vec<f64> = input_values
            .iter()
            .take(text_unit_index + 1)
            .map(|key| {
                multiply_matching_coordinates_then_add_results(&query, key).unwrap()
                    / approximate_square_root_by_repeated_averaging(2.0)
            })
            .collect();
        let attention_weights: Vec<f64> = (|| -> Vec<f64> {
            let input_values: &[f64] = &raw_model_scores;
            let mut maximum_value: f64 = f64::NEG_INFINITY;
            for &value in input_values {
                if value > maximum_value {
                    maximum_value = value;
                }
            }
            let mut exponentials: Vec<f64> = Vec::with_capacity(input_values.len());
            let mut normalizer: f64 = 0.0;
            for &value in input_values {
                let exponential_value: f64 =
                    approximate_e_to_power_by_summing_power_over_factorial_terms(
                        value - maximum_value,
                    );
                exponentials.push(exponential_value);
                normalizer += exponential_value;
            }
            for exponential_value in &mut exponentials {
                *exponential_value /= normalizer;
            }
            exponentials
        })();
        let mut attended: [f64; 2] = [0.0, 0.0];
        for key_index in 0..attention_weights.len() {
            attended[0] += attention_weights[key_index] * input_values[key_index][0];
            attended[1] += attention_weights[key_index] * input_values[key_index][1];
        }
        let normalized_values: [f64; 2] =
            normalize_coordinates_by_subtracting_mean_and_dividing_by_root_mean_square([
                query[0] + attended[0],
                query[1] + attended[1],
            ]);
        let feed_forward_values: [f64; 2] = [
            choose_larger_number(normalized_values[0], 0.),
            choose_larger_number(normalized_values[1], 0.),
        ];
        normalize_coordinates_by_subtracting_mean_and_dividing_by_root_mean_square([
            normalized_values[0] + feed_forward_values[0],
            normalized_values[1] + feed_forward_values[1],
        ])
    });

    plot_outputs_after_context_mixing_and_coordinate_transformation(
        input_values,
        transformer_output,
    );
}

// Строим график по результатам урока.
fn plot_outputs_after_context_mixing_and_coordinate_transformation(
    input_values: [[f64; 2]; 2],
    transformer_output: [[f64; 2]; 2],
) {
    let input_matrix: Vec<Vec<f64>> = input_values.iter().map(|row| row.to_vec()).collect();
    let output_matrix: Vec<Vec<f64>> = transformer_output.iter().map(|row| row.to_vec()).collect();
    for (name, title, values) in [
        ("input", "Вход блока Transformer", &input_matrix),
        ("output", "Выход блока Transformer", &output_matrix),
    ] {
        let _chart: std::path::PathBuf =
            lesson_visualization::heatmap(env!("CARGO_MANIFEST_DIR"), name, title, values)
                .expect("не удалось сохранить график");
    }
}
