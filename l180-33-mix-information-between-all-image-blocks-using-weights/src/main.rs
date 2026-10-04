// Урок 180. Вычислять веса внимания между участками изображения и собирать из них контекст первого
// участка.
// Проверяем, что в общий вектор действительно входят значения разных участков.

use l179_33_extract_nonoverlapping_square_patches_from_square_image::extract_nonoverlapping_square_patches_from_square_image;

fn calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum(
    raw_model_scores: &[f64; 4],
) -> [f64; 4] {
    let maximum: f64 = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let exponential_values: [f64; 4] =
        std::array::from_fn(|index| (raw_model_scores[index] - maximum).exp());
    let sum_of_exponential_values: f64 = exponential_values.iter().sum();
    exponential_values.map(|patch_value| patch_value / sum_of_exponential_values)
}
fn main() {
    let image: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];

    let image_patch_representations: [[f64; 2]; 4] =
        extract_nonoverlapping_square_patches_from_square_image(&image, 1)
            .unwrap()
            .iter()
            .map(|patch| [patch[0], 1.0 - patch[0]])
            .collect::<Vec<_>>()
            .try_into()
            .expect("ожидалось четыре патча 1×1 из изображения 2×2");
    let patch1_representation: [f64; 2] = image_patch_representations[0];
    let weights: [f64; 4] =
        calc_softmax_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_sum(
            &std::array::from_fn::<f64, 4, _>(|index| {
                let key = image_patch_representations[index];
                patch1_representation[0] * key[0] + patch1_representation[1] * key[1]
            }),
        );
    assert!(weights[3] > 0.0);

    // Выполняем вычисления из примера.
    let _ = &weights;

    let mut context = [0.0; 2];
    for (patch, weight) in image_patch_representations.iter().zip(weights) {
        context[0] += weight * patch[0];
        context[1] += weight * patch[1];
    }
    println!(
        "Представления участков={image_patch_representations:?}; веса={weights:?}; контекст первого={context:?}"
    );
    assert!(context[0] > 0.0 && context[1] > 0.0);
    assert!((context.iter().sum::<f64>() - 1.0).abs() < 1e-12);
}

// Чему учит этот урок:
// Учимся вычислять веса внимания между участками изображения и собирать из них контекст первого
// участка.
// Проверяем, что в общий вектор действительно входят значения разных участков.
