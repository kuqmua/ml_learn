// Урок 14.4. Сравнение ошибок на обучающих и новых данных при увеличении глубины дерева.
// Связь с принятой терминологией: Переобучение из-за чрезмерной глубины дерева решений.
// Зачем здесь эта тема: Дерево может довести train до идеальной чистоты, но запомнить шум вместо
//   закономерности.
// Почему код устроен так: Сравниваем разную глубину на обучающих и проверочных строках.
// Представь: Глубокое дерево может выделить отдельный лист на каждую шумную строку и провалиться на
//   новых данных.
//
// Что изучаем: Переобучение дерева.
// Зачем это нужно: Чем глубже дерево, тем легче запомнить отдельные обучающие точки. Сравниваем ошибку
// train и новых данных.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let training_error: [f64; 3] = [0.25, 0.05, 0.0];
    let validation_error: [f64; 3] = [0.30, 0.15, 0.35];
    for depth in 1..=3 {
        let _ = (&(training_error[depth - 1]), &(validation_error[depth - 1]));
    }

    plot_training_and_validation_errors_for_growing_tree_depth(training_error, validation_error);
}

// Строим график по результатам урока.
fn plot_training_and_validation_errors_for_growing_tree_depth(
    training_error: [f64; 3],
    validation_error: [f64; 3],
) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Переобучение дерева",
        "глубина",
        "ошибка",
        &[
            lesson_visualization::Series {
                name: "train",

                points: &training_error
                    .iter()
                    .enumerate()
                    .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "validation",

                points: &validation_error
                    .iter()
                    .enumerate()
                    .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
