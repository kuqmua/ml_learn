// Урок 12.4. Подсчёт сравнений координат при поиске среди всех обучающих точек.
// Зачем здесь эта тема: kNN хранит обучающие точки и сравнивает с ними каждый новый запрос.
// Почему код устроен так: Подсчитываем число сравнений при росте train, чтобы увидеть цену простого
//   обучения.
// Представь: Для одного нового объекта kNN сравнивает его с каждой строкой train: при 1000 строках
//   это 1000 сравнений.
//
// Что изучаем: Стоимость прогноза kNN.
// Зачем это нужно: Простой kNN сравнивает запрос с каждой обучающей точкой: число вычислений расстояния
// растёт вместе с набором.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    for training_size in [10, 100, 1000] {
        let feature_count: i32 = 4;
        let _: i32 = training_size * feature_count;
    }

    plot_coord_comparison_count_for_growing_training_set();
}

// Строим график по результатам урока.
fn plot_coord_comparison_count_for_growing_training_set() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Стоимость kNN",
        "число обучающих объектов",
        "сравнения координат",
        &[lesson_visualization::Series {
            name: "4 признака",

            points: &(1..=100)
                .map(|plot_step_index| {
                    let sample_count: f64 = (plot_step_index * 10) as f64;
                    (sample_count, 2.0 * sample_count)
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
