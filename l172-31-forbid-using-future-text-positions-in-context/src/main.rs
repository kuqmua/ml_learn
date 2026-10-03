// Урок 31.6. Запрет использования будущих позиций текста при сборе контекста.
// Связь с принятой терминологией: Запрет внимания к будущим токенам причинной маской.
// Зачем здесь эта тема: При генерации токена модель не должна видеть продолжение, которое ещё
//   предстоит предсказать.
// Почему код устроен так: Закрываем будущие позиции до softmax, иначе их значения утекут в текущий
//   выход.
// Представь: Предсказывая третье слово, нельзя смотреть на четвёртое: иначе ответ уже подсказан.
//
// На позиции 0 виден только первый токен, на позиции 1 — первые два,
// на последней позиции — все. Вес будущих позиций всегда равен нулю.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

fn main() {
    let raw_weights: [f64; 3] = [0.2, 0.3, 0.5];
    for current_position in 0..raw_weights.len() {
        let mut future_position_filtered_weights: [f64; 3] = [0.0; 3];
        let sum_of_allowed_weights: f64 = raw_weights[..=current_position].iter().sum();
        assert!(
            sum_of_allowed_weights > 0.0,
            "доступные позиции должны иметь положительную сумму весов"
        );
        for index in 0..=current_position {
            future_position_filtered_weights[index] = raw_weights[index] / sum_of_allowed_weights;
        }
        assert!(check_f64_eq_1e_minus_10(
            future_position_filtered_weights.iter().sum::<f64>(),
            1.0
        ));
        assert!(
            future_position_filtered_weights[current_position + 1..]
                .iter()
                .all(|&weight| weight == 0.0)
        );
    }

    plot_allowed_current_and_past_position_pairs(raw_weights);
}

// Строим график по результатам урока.
fn plot_allowed_current_and_past_position_pairs(raw_weights: [f64; 3]) {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Причинная маска внимания",
        &std::array::from_fn::<[f64; 3], 3, _>(|position| {
            let sum_of_allowed_weights: f64 = raw_weights[..=position].iter().sum();
            std::array::from_fn(|key| {
                if key <= position {
                    raw_weights[key] / sum_of_allowed_weights
                } else {
                    0.0
                }
            })
        })
        .iter()
        .map(|row| row.to_vec())
        .collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить график");
}
