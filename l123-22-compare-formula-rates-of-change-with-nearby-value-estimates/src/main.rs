// Урок 22.4. Сравнение скоростей изменения по формуле с оценками по соседним значениям.
// Связь с принятой терминологией: Сравнение аналитического градиента с центральной разностью.
// Зачем здесь эта тема: Сложный обратный проход легко реализовать с неверным знаком или формой.
// Почему код устроен так: Сравниваем аналитический градиент с центральной разностью на тех же
//   параметрах.
// Представь: Если формула дала градиент 6, а малое изменение параметра показывает около −6, знак в
//   формуле неверен.
//
// Что изучаем: Проверка градиента.
// Зачем это нужно: Сравниваем аналитическую производную с центральной численной разностью на малом
// примере.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let input_value: f64 = 3.0;
    let step_size: f64 = 0.0001;
    let _analytical: f64 = 2.0 * input_value;
    let right: f64 = (input_value + step_size) * (input_value + step_size);
    let left: f64 = (input_value - step_size) * (input_value - step_size);
    let _numerical: f64 = (right - left) / (2.0 * step_size);

    plot_difference_between_formula_slope_and_two_point_estimate();
}

// Строим график по результатам урока.
fn plot_difference_between_formula_slope_and_two_point_estimate() {
    let derivative_comparison_points: Vec<(f64, f64)> = (1..=100)
        .map(|plot_step_index| {
            let step_size: f64 = plot_step_index as f64 / 100.0;
            (
                step_size,
                (((3.0 + step_size) * (3.0 + step_size) - 9.0) / step_size - 6.0).abs(),
            )
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Проверка градиента",
        "шаг h",
        "разность",
        &[lesson_visualization::Series {
            name: "x² в x=3",

            points: &derivative_comparison_points,
        }],
    )
    .expect("не удалось сохранить график");
}
