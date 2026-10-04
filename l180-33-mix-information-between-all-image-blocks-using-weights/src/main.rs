// Урок 33.2. Обмен информацией между всеми участками изображения через взвешенное сложение.
// Зачем здесь эта тема: Локальные патчи должны обмениваться сведениями о далёких областях
//   изображения.
// Почему код устроен так: Считаем веса первого патча ко всем патчам и проверяем, что даже последний
//   доступен.
// Представь: Первый патч может получить вес внимания и от последнего, даже если они далеко друг от
//   друга на картинке.
// В классификации изображения патчи могут видеть друг друга без причинной маски.

// Нормируем оценки всех патчей в вероятностные веса.
// Оценка модели до преобразования в вероятность — обычное число, которое затем переводят в диапазон от 0 до 1.
/// Softmax: вычитаем максимальную оценку, вычисляем экспоненты и делим каждую на их сумму.
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
}

// Чему учит этот урок:
// Учимся считать веса внимания от одного участка изображения ко всем остальным.
// Проверяем доступность удалённого участка; взвешенная сумма представлений в текущем коде ещё не
// вычисляется.
