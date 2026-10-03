// Урок 32.4. Выход слоя для каждой позиции: взвешенное преобразование координат.
// Связь с принятой терминологией: Преобразование каждого токена полносвязным слоем.
// Зачем здесь эта тема: Внимание смешивает сведения между позициями, но каждой позиции нужно и
//   собственное нелинейное преобразование.
// Почему код устроен так: Применяем одинаковый полносвязный блок к каждому токену независимо.
// Представь: После обмена сведениями между словами каждый вектор отдельно проходит одинаковую
//   нелинейную функцию.
//
// Что изучаем: Feed-forward слой.
// Зачем это нужно: После внимания каждый токен отдельно проходит через линейное преобразование и
// нелинейную активацию.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let text_unit: [f64; 2] = [1.0, 2.0];
    let linear: [f64; 2] = [
        0.5 * text_unit[0] - 0.2 * text_unit[1],
        0.3 * text_unit[0] + 0.4 * text_unit[1],
    ];
    let relu_outputs_where_negative_inputs_become_0_and_positive_inputs_pass_unchanged: [f64; 2] = [
        if linear[0] > 0.0 { linear[0] } else { 0.0 },
        if linear[1] > 0.0 { linear[1] } else { 0.0 },
    ];

    plot_activations_after_transforming_each_position(
        relu_outputs_where_negative_inputs_become_0_and_positive_inputs_pass_unchanged,
    );
}

// Строим график по результатам урока.
fn plot_activations_after_transforming_each_position(
    relu_outputs_where_negative_inputs_become_0_and_positive_inputs_pass_unchanged: [f64; 2],
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Feed-forward",
        "активация",
        &[
            (
                "нейрон 0",
                relu_outputs_where_negative_inputs_become_0_and_positive_inputs_pass_unchanged[0],
            ),
            (
                "нейрон 1",
                relu_outputs_where_negative_inputs_become_0_and_positive_inputs_pass_unchanged[1],
            ),
        ],
    )
    .expect("не удалось сохранить график");
}
