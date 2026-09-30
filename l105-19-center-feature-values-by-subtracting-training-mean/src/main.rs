// Урок 19.1. Центрирование признака: вычитание среднего по обучающим данным.
// Связь с принятой терминологией: Центрирование признаков вычитанием среднего по обучающим данным.
// Зачем здесь эта тема: PCA ищет разброс относительно центра; без вычитания среднего направление
//   может отражать смещение данных.
// Почему код устроен так: Вычисляем среднее на train и вычитаем его из каждой координаты.
// Представь: Если все x увеличить на 100, форма облака не меняется; вычитание среднего убирает это
//   смещение.
//
// Что изучаем: Центрирование признаков.
// Зачем это нужно: Вычитаем среднее из каждой координаты, чтобы PCA искал направления изменчивости
// относительно центра данных.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `values` для следующего шага примера.");
    let values: [f64; 3] = [1.0, 2.0, 3.0];
    trace_step!(values);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !values.is_empty(),
        "для центрирования нужно хотя бы одно значение"
    );
    trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `mean`.");
    let mean: f64 = calculate_mean_by_summing_values_and_dividing_by_count(&values).unwrap();
    trace_step!(mean);
    trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `centered`.");
    let centered: Vec<f64> = values.iter().map(|&value| value - mean).collect();
    trace_step!(centered);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("среднее={mean}, центрированные значения={centered:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_feature_values_after_subtracting_mean(values, centered);
}

// Строим график по результатам урока.
fn plot_feature_values_after_subtracting_mean(values: [f64; 3], centered: std::vec::Vec<f64>) {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let original_points: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    trace_note!("Собираем значения для `centered_points` в коллекцию.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let centered_points: Vec<(f64, f64)> = centered
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
        "номер наблюдения",
        "значение",
        &[
            lesson_visualization::Series {
                name: "исходные",

                points: &original_points,
            },
            lesson_visualization::Series {
                name: "центрированные",

                points: &centered_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
