// Урок 35.5. Выход блока декодера: нормализация перед сбором контекста и преобразованием координат.
// Связь с принятой терминологией: Блок Transformer decoder с нормализацией перед подслоями.
// Зачем здесь эта тема: Генеративный блок должен повторяемо соединять нормализацию, причинное
//   внимание и feed-forward.
// Почему код устроен так: Показываем pre-norm и residual после каждого подслоя на коротком
//   префиксе.
// Представь: Каждый подслой получает нормированный вход, а residual возвращает его в итог этого
//   шага.
// Нормализация, причинное внимание, residual, FFN и второй residual образуют блок.

/// Нормализация слоя (LayerNorm): из координат вычитаем среднее и делим на sqrt(среднее квадратов отклонений + epsilon).
use l187_35_calculate_past_context_by_summing_current_and_past_values_with_match_weights::calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches;

fn normalize_coordinates_by_subtracting_mean_and_dividing_by_root_mean_squared_deviation(
    input_value: [f64; 2],
) -> [f64; 2] {
    let mean: f64 = (input_value[0] + input_value[1]) / 2.0;
    let variance: f64 = ((input_value[0] - mean).powi(2) + (input_value[1] - mean).powi(2)) / 2.0;
    [
        (input_value[0] - mean) / (variance + 1e-5).sqrt(),
        (input_value[1] - mean) / (variance + 1e-5).sqrt(),
    ]
}
/// Учебный блок декодера: нормализуем вход, прибавляем причинный контекст, снова нормализуем и прибавляем 0.2·max(0, x).
fn calculate_decoder_block_output_by_adding_past_context_and_transformed_normalized_values<
    const N: usize,
>(
    input: &[[f64; 2]; N],
) -> [[f64; 2]; N] {
    let normalized: [[f64; 2]; N] = input
        .map(normalize_coordinates_by_subtracting_mean_and_dividing_by_root_mean_squared_deviation);
    let attention: [[f64; 2]; N] =
        calculate_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &normalized,
            &normalized,
            &normalized,
        )
        .unwrap()
        .try_into()
        .expect("внимание возвращает по одному вектору на позицию");
    std::array::from_fn(|index| {
        let original = input[index];
        let context = attention[index];
        let input_plus_transformed_value: [f64; 2] =
            [original[0] + context[0], original[1] + context[1]];
        let norm: [f64; 2] =
            normalize_coordinates_by_subtracting_mean_and_dividing_by_root_mean_squared_deviation(
                input_plus_transformed_value,
            );
        [
            input_plus_transformed_value[0] + 0.2 * norm[0].max(0.0),
            input_plus_transformed_value[1] + 0.2 * norm[1].max(0.0),
        ]
    })
}
fn main() {
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let output: [[f64; 2]; 3] =
        calculate_decoder_block_output_by_adding_past_context_and_transformed_normalized_values(
            &states,
        );
    assert!(output.iter().flatten().all(|value| value.is_finite()));
}
