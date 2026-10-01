// Урок 21.3. Активация ReLU: замена отрицательных выходов слоя нулями.
// Связь с принятой терминологией: Нелинейная активация в слое нейронной сети.
// Зачем здесь эта тема: Цепочка только линейных слоёв сводится к одному линейному преобразованию.
// Почему код устроен так: Добавляем нелинейность между слоями и сравниваем выход с линейным
//   случаем.
// Представь: Без активации два последовательных линейных слоя можно заменить одним; нелинейность
//   меняет возможности сети.
//
// Что изучаем: Функция активации.
// Зачем это нужно: Нелинейная активация позволяет сети описывать зависимости, которые не выражаются одной
// прямой.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    for raw_model_score in [-2.0, 0.0, 2.0] {
        let _rectified_linear_output: f64 = if raw_model_score > 0.0 {
            raw_model_score
        } else {
            0.0
        };
    }

    plot_rectified_activation_as_input_with_negative_values_replaced_by_zero();
}

// Строим график по результатам урока.
fn plot_rectified_activation_as_input_with_negative_values_replaced_by_zero() {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "ReLU",
        "вход",
        "выход",
        &[lesson_visualization::Series {
            name: "max(0,x)",

            points: &(-50..=50)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (horizontal_value, horizontal_value.max(0.0))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
