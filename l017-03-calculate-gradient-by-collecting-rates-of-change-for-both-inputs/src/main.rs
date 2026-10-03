// Урок 03.4. Градиент: сбор скоростей изменения функции по двум входам в один вектор.
// Связь с принятой терминологией: Градиент функции двух переменных.
// Зачем здесь эта тема: Частные производные вместе дают направление изменения функции в
//   пространстве параметров.
// Почему код устроен так: Собираем их в вектор, чтобы следующий блок мог сделать шаг против
//   градиента.
// Представь: Когда ошибка зависит от двух весов, градиент говорит, как меняется она при изменении
//   каждого веса.
//
// Что изучаем: Градиент.
// Зачем это нужно: Градиент собирает все частные производные в вектор. Он указывает направление самого
// быстрого роста функции.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let (input_value, second_input_value): (f64, f64) = (0.0, 0.0);
    let _gradient_pointing_toward_fastest_local_increase_with_0_components_meaning_no_first_order_change: [f64; 2] = [2.0 * (input_value - 2.0), 6.0 * (second_input_value + 1.0)];

    plot_rate_of_change_along_first_coordinate();
}

// Строим график по результатам урока.
fn plot_rate_of_change_along_first_coordinate() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Градиент квадратичной функции",
        "x",
        "производная",
        &[lesson_visualization::Series {
            name: "∂f/∂x при y=0",

            points: &(-40..=40)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (horizontal_value, 2.0 * (horizontal_value - 3.0))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
