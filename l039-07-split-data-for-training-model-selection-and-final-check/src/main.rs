// Урок 07.3. Разделение данных для обучения, выбора модели и итоговой проверки.
// Связь с принятой терминологией: Разделение набора данных на обучение, валидацию и тест.
// Зачем здесь эта тема: Качество на обучающих строках не показывает работу на новых; данные нужно
//   разделить заранее.
// Почему код устроен так: Выделяем train, validation и test до любых операций, способных запомнить
//   статистики набора.
// Представь: Если модель увидела строку при обучении, проверка на той же строке не показывает
//   прогноз на новые данные.
//
// Что изучаем: Разделение train/validation/test.
// Зачем это нужно: Обучение подбирает параметры по train, validation выбирает настройку, test остаётся для
// итоговой проверки.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let rows: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let training_data: &[i32] = &rows[..6];
    let validation: &[i32] = &rows[6..8];
    let test: &[i32] = &rows[8..];

    plot_row_counts_in_training_validation_and_test_sets(training_data, validation, test);
}

// Строим график по результатам урока.
fn plot_row_counts_in_training_validation_and_test_sets(
    training_data: &[i32],
    validation: &[i32],
    test: &[i32],
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Разделение набора",
        "число строк",
        &[
            ("train", training_data.len() as f64),
            ("validation", validation.len() as f64),
            ("test", test.len() as f64),
        ],
    )
    .expect("не удалось сохранить график");
}
