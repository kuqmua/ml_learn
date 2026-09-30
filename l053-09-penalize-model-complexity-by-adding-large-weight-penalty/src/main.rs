// Урок 09.4. Ограничение сложности модели: добавление штрафа за большой вес.
// Связь с принятой терминологией: Штраф за большой вес линейной модели.
// Зачем здесь эта тема: Без ограничения веса линейная модель может подгоняться под шум обучающих
//   данных.
// Почему код устроен так: Добавляем штраф к ошибке обучения и смотрим, как меняется предпочтение
//   больших коэффициентов.
// Представь: Если модель увеличивает вес до огромного числа ради пары строк, штраф делает такое
//   решение менее выгодным.
//
// Что изучаем: Регуляризация коэффициентов.
// Зачем это нужно: Штраф за большой вес добавляется к ошибке обучения и побуждает модель выбирать более
// простое решение.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let prediction_error: f64 = 1.0;
    let weight: f64 = 3.0;
    let penalty_strength: f64 = 0.2;
    let squared_weight: f64 = weight * weight;
    let _objective: f64 = prediction_error + penalty_strength * squared_weight;
    let _ = &(penalty_strength * squared_weight);

    plot_error_with_and_without_squared_weight_penalty();
}

// Строим график по результатам урока.
fn plot_error_with_and_without_squared_weight_penalty() {
    let unregularized_model_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| (plot_step_index as f64 / 10.0, 1.0))
        .collect();
    let penalty_constrained_model_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            let weight_value: f64 = plot_step_index as f64 / 10.0;
            (weight_value, 1.0 + weight_value * weight_value)
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Штраф за большой вес",
        "вес",
        "целевая функция",
        &[
            lesson_visualization::Series {
                name: "без регуляризации",

                points: &unregularized_model_points,
            },
            lesson_visualization::Series {
                name: "со штрафом",

                points: &penalty_constrained_model_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
