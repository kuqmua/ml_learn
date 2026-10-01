// Урок 03.5. Приближённая производная: оценка скорости изменения по значениям слева и справа от точки.
// Связь с принятой терминологией: Приближение производной функции центральной разностью.
// Зачем здесь эта тема: Аналитическую производную легко вывести с ошибкой; центральная разность
//   даёт независимую численную проверку.
// Почему код устроен так: Считаем функцию по обе стороны точки: симметрия уменьшает ошибку по
//   сравнению с односторонней разностью.
// Представь: Чтобы проверить наклон в точке x, сравниваем значения функции чуть левее и чуть правее
//   x.
//
// Что изучаем: Численная производная.
// Зачем это нужно: Центральная разность оценивает производную по значениям функции слева и справа от
// точки. Сравнение с точной формулой показывает погрешность.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let input_value: f64 = 3.0;
    let step: f64 = 0.0001;
    let value_after_adding_step: f64 = (input_value + step) * (input_value + step);
    let value_after_subtracting_step: f64 = (input_value - step) * (input_value - step);
    let _numerical_derivative: f64 =
        (value_after_adding_step - value_after_subtracting_step) / (2.0 * step);
    let analytical_derivative: f64 = 2.0 * input_value;

    plot_slope_estimation_error_for_shrinking_step(input_value, analytical_derivative);
}

// Строим график по результатам урока.
fn plot_slope_estimation_error_for_shrinking_step(
    horizontal_value: f64,
    analytical_derivative: f64,
) {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ошибка центральной разности",
        "k для h=10⁻ᵏ",
        "абсолютная ошибка",
        &[lesson_visualization::Series {
            name: "x² в x=3",

            points: &(1..=12)
                .map(|step_exponent| {
                    let step_size: f64 = 10f64.powi(-step_exponent);
                    let numeric: f64 = ((horizontal_value + step_size)
                        * (horizontal_value + step_size)
                        - (horizontal_value - step_size) * (horizontal_value - step_size))
                        / (2.0 * step_size);
                    (
                        step_exponent as f64,
                        (numeric - analytical_derivative).abs(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
