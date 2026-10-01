// Урок 03.1. Производная функции: скорость изменения результата при изменении входа.
// Связь с принятой терминологией: Производная функции одной переменной.
// Зачем здесь эта тема: Обучение меняет параметр так, чтобы снизить ошибку; производная показывает
//   местный наклон этой ошибки.
// Почему код устроен так: Начинаем с одной переменной, где знак и величину наклона легко сверить по
//   формуле.
// Представь: Если f(x)=x², при x=3 небольшой рост x увеличивает f; производная 6 показывает местную
//   скорость роста.
//
// Что изучаем: Производная одной переменной.
// Зачем это нужно: Производная показывает мгновенную скорость изменения функции. Для f(x)=x² аналитическая
// производная равна 2x.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let input_value: f64 = 3.0;
    let _derivative: f64 = 2.0 * input_value;

    plot_squared_input_and_tangent_line();
}

// Строим график по результатам урока.
fn plot_squared_input_and_tangent_line() {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Функция и касательная в x=3",
        "x",
        "значение",
        &[
            lesson_visualization::Series {
                name: "x²",

                points: &(0..=60)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, horizontal_value * horizontal_value)
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "касательная",

                points: &(0..=60)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, 9.0 + 6.0 * (horizontal_value - 3.0))
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
