// Урок 09.3. Линейный прогноз: умножение входного значения на вес и прибавление смещения.
// Связь с принятой терминологией: Вес и смещение линейной регрессионной модели.
// Зачем здесь эта тема: После метрики нужна модель, параметры которой можно подбирать для
//   уменьшения ошибки.
// Почему код устроен так: Считаем линейный прогноз как сумму признаков с весами и отдельным
//   смещением.
// Представь: При формуле y=2x+1 число 2 — вес признака, а 1 — смещение прогноза.
//
// Что изучаем: Коэффициенты линейной модели.
// Зачем это нужно: Вес задаёт изменение прогноза при увеличении признака на единицу, а смещение задаёт
// прогноз при нулевом признаке.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let weight: f64 = 2.0;
    let constant_input_weight: f64 = 1.0;
    for feature in [0.0, 1.0, 3.0] {
        let _: f64 = weight * feature + constant_input_weight;
    }

    plot_linear_prediction_as_weighted_input_plus_constant_input_weight();
}

// Строим график по результатам урока.
fn plot_linear_prediction_as_weighted_input_plus_constant_input_weight() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Линейная модель",
        "признак x",
        "предсказание",
        &[lesson_visualization::Series {
            name: "y=2x+1",

            points: &(0..=50)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (horizontal_value, 2.0 * horizontal_value + 1.0)
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
