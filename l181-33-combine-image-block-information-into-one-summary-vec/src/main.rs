// Урок 181. Собирать общий вектор изображения взвешенной суммой представлений его участков.
// По координатам этого вектора принимаем простое решение о классе, связывая внимание с итоговым
// ответом.

fn calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum(
    values: &[f64; 3],
) -> [f64; 3] {
    let maximum_value: f64 = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let exponential_values: [f64; 3] =
        std::array::from_fn(|index| (values[index] - maximum_value).exp());
    let sum_of_exponential_values: f64 = exponential_values.iter().sum();
    exponential_values.map(|patch_value| patch_value / sum_of_exponential_values)
}
fn main() {
    let image_input_representations: [[f64; 2]; 3] = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let weights: [f64; 3] =
        calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum(
            &std::array::from_fn::<f64, 3, _>(|index| {
                let image_input_representation = image_input_representations[index];
                image_input_representations[0][0] * image_input_representation[0]
                    + image_input_representations[0][1] * image_input_representation[1]
            }),
        );
    let image_classification_summary: [f64; 2] =
        image_input_representations.iter().zip(&weights).fold(
            [0.0; 2],
            |mut weighted_coord_sums, (patch_value, &weight)| {
                weighted_coord_sums[0] += weight * patch_value[0];
                weighted_coord_sums[1] += weight * patch_value[1];
                weighted_coord_sums
            },
        );
    let _: u8 = u8::from(image_classification_summary[0] > image_classification_summary[1]);

    // Выполняем вычисления из примера.
    let _ = &weights;

    let class = u8::from(image_classification_summary[0] > image_classification_summary[1]);
    println!(
        "Доли участков={weights:?}; общий вектор={image_classification_summary:?}; выбранный класс={class}"
    );
    assert_eq!(class, 1);
}

// Чему учит этот урок:
// Учимся собирать общий вектор изображения взвешенной суммой представлений его участков.
// По координатам этого вектора принимаем простое решение о классе, связывая внимание с итоговым
// ответом.
