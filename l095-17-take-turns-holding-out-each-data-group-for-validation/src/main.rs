// Урок 17.1. Поочерёдное выделение каждой группы данных для проверки модели.
// Зачем здесь эта тема: Один train/validation split может дать случайно удачную оценку.
// Почему код устроен так: По очереди делаем каждый блок проверочным и усредняем результаты.
// Представь: При трёх блоках каждый один раз становится проверочным, а два других служат обучением.
//
// Что изучаем: K-fold кросс-валидация.
// Зачем это нужно: Каждый блок данных один раз становится проверочным, пока остальные используются для
// обучения.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let rows: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let folds: i32 = 3;
    for fold in 0..folds {
        let _: Vec<i32> = rows
            .iter()
            .copied()
            .filter(|&row| row % folds == fold)
            .collect();
        let _: Vec<i32> = rows
            .iter()
            .copied()
            .filter(|&row| row % folds != fold)
            .collect();
    }

    plot_validation_group_assigned_to_each_row();
}

// Строим график по результатам урока.
fn plot_validation_group_assigned_to_each_row() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "K-fold: номер fold для строки",
        "номер строки",
        "fold",
        &[lesson_visualization::Series {
            name: "3 части",

            points: &(0..9)
                .map(|plot_step_index| (plot_step_index as f64, (plot_step_index % 3) as f64))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
