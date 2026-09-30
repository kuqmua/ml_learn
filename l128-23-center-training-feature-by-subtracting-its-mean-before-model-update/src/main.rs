// Урок 23.4. Центрирование входа для обучения: вычитание среднего обучающего признака.
// Связь с принятой терминологией: Нормализация входных признаков для обучения по градиенту.
// Зачем здесь эта тема: Разные масштабы признаков делают градиентные шаги несоразмерными.
// Почему код устроен так: Вычитаем среднее train из его значений, чтобы отдельно увидеть эффект
//   центрирования.
// Представь: Для train [10, 20, 30] среднее 20; после вычитания получаем [−10, 0, 10].
//
// Что изучаем: Нормализация признаков.
// Зачем это нужно: Приведение признаков к сопоставимому масштабу помогает градиентным шагам работать с
// разными координатами.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `training_data` для следующего шага примера.");
    let training_data: [f64; 3] = [10.0, 20.0, 30.0];
    trace_step!(training_data);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `mean`.");
    let mean: f64 = calculate_mean_by_summing_values_and_dividing_by_count(&training_data).unwrap();
    trace_step!(mean);
    trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `centered`.");
    let centered: Vec<f64> = training_data.iter().map(|&value| value - mean).collect();
    trace_step!(centered);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("среднее train={mean}, центрировано={centered:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_feature_before_and_after_subtracting_mean(training_data, centered);
}

// Строим график по результатам урока.
fn plot_feature_before_and_after_subtracting_mean(
    training_data: [f64; 3],
    centered: std::vec::Vec<f64>,
) {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let original_points: Vec<(f64, f64)> = training_data
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    trace_note!("Собираем значения для `normalized_points` в коллекцию.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let normalized_points: Vec<(f64, f64)> = centered
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Центрирование признака",
        "номер объекта",
        "значение",
        &[
            lesson_visualization::Series {
                name: "до",

                points: &original_points,
            },
            lesson_visualization::Series {
                name: "после",

                points: &normalized_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
