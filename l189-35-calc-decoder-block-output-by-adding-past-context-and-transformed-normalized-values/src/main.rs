// Урок 35.5. Выход блока декодера: нормализация перед сбором контекста и преобразованием координат.
// Зачем здесь эта тема: Генеративный блок должен повторяемо соединять нормализацию, причинное
//   внимание и feed-forward.
// Почему код устроен так: Показываем нормализацию перед подслоем и прибавление входа после каждого подслоя на коротком
//   префиксе.
// Представь: Каждый подслой получает нормированный вход, а прибавление входа возвращает его в итог этого
//   шага.
// Нормализация, причинное внимание, прибавление входа, FFN и второй прибавление входа образуют блок.

/// Нормализация слоя (LayerNorm): из координат вычитаем среднее и делим на sqrt(среднее квадратов отклонений + epsilon).
use l187_35_calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches::calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches;

fn normalize_coords_by_subtracting_mean_and_dividing_by_root_mean_square(
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
fn calc_decoder_block_output_by_adding_past_context_and_transformed_normalized_values(
    input: &[[f64; 2]; 3],
) -> [[f64; 2]; 3] {
    let normalized: [[f64; 2]; 3] =
        input.map(normalize_coords_by_subtracting_mean_and_dividing_by_root_mean_square);
    let attention: [[f64; 2]; 3] =
        calc_past_context_by_summing_current_and_past_values_weighted_by_query_key_matches(
            &normalized,
            &normalized,
            &normalized,
        )
        .unwrap()
        .try_into()
        .expect("ожидался один вектор внимания на каждую позицию");
    std::array::from_fn(|index| {
        let original = input[index];
        let context = attention[index];
        let input_plus_transformed_value: [f64; 2] =
            [original[0] + context[0], original[1] + context[1]];
        let scaled_values: [f64; 2] =
            normalize_coords_by_subtracting_mean_and_dividing_by_root_mean_square(
                input_plus_transformed_value,
            );
        [
            input_plus_transformed_value[0] + 0.2 * scaled_values[0].max(0.0),
            input_plus_transformed_value[1] + 0.2 * scaled_values[1].max(0.0),
        ]
    })
}
fn main() {
    let states: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];

    assert!(
        calc_decoder_block_output_by_adding_past_context_and_transformed_normalized_values(
            &states,
        )
        .iter()
        .flatten()
        .all(|value| value.is_finite())
    );
}

// Чему учит этот урок:
// Учимся собирать блок обработки последовательности из нормализации, внимания к прошлому и
// добавочных преобразований.
// Проверяем конечность выходных чисел; веса здесь заданы, обучение блока не выполняется.
