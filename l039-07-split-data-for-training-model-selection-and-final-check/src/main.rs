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
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `rows` для следующего шага примера.");
    let rows: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    trace_step!(rows);
    trace_note!("Сохраняем рассчитанное значение `training_data` для следующих операций.");
    let training_data: &[i32] = &rows[..6];
    trace_step!(training_data);
    trace_note!("Сохраняем рассчитанное значение `validation` для следующих операций.");
    let validation: &[i32] = &rows[6..8];
    trace_step!(validation);
    trace_note!("Сохраняем рассчитанное значение `test` для следующих операций.");
    let test: &[i32] = &rows[8..];
    trace_step!(test);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("train={training_data:?}, validation={validation:?}, test={test:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_row_counts_in_training_validation_and_test_sets(training_data, validation, test);
}

// Строим график по результатам урока.
fn plot_row_counts_in_training_validation_and_test_sets(
    training_data: &[i32],
    validation: &[i32],
    test: &[i32],
) {
    trace_note!("Сравнение величин из этого урока.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
