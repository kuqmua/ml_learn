// Урок 12.2. Приведение признаков к сопоставимым масштабам перед измерением расстояния.
// Связь с принятой терминологией: Масштабирование признаков перед вычислением расстояния до соседей.
// Зачем здесь эта тема: Признак с большим числовым диапазоном может захватить расстояние независимо
//   от полезности.
// Почему код устроен так: Масштабируем координаты по train и сравниваем порядок соседей до и после.
// Представь: Разность возраста на 30 лет численно перекроет разность доли 0,2, если признаки не
//   привести к сравнимому масштабу.
//
// Что изучаем: Масштабирование признаков.
// Зачем это нужно: Признак с большими единицами измерения может захватить расстояние. Масштабируем
// координаты перед сравнением соседей.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences;

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `first` для следующего шага примера.");
    let first: [f64; 2] = [1.0, 1000.0];
    trace_step!(first);
    trace_note!("Создаём набор значений `second` для следующего шага примера.");
    let second: [f64; 2] = [2.0, 1010.0];
    trace_step!(second);
    trace_note!("Создаём набор значений `scale` для следующего шага примера.");
    let scale: [f64; 2] = [1.0, 1000.0];
    trace_step!(scale);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        scale.iter().all(|&value| value > 0.0),
        "масштабы должны быть положительными"
    );
    trace_note!("Сохраняем результат этого шага в `raw_squared`.");
    let raw_squared: f64 =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(&first, &second)
            .unwrap();
    trace_step!(raw_squared);
    trace_note!("Задаём учебные значения для `scaled_first`.");
    let scaled_first: [f64; 2] = [first[0] / scale[0], first[1] / scale[1]];
    trace_step!(scaled_first);
    trace_note!("Задаём учебные значения для `scaled_second`.");
    let scaled_second: [f64; 2] = [second[0] / scale[0], second[1] / scale[1]];
    trace_step!(scaled_second);
    trace_note!("Сохраняем результат этого шага в `scaled_squared`.");
    let scaled_squared: f64 =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(
            &scaled_first,
            &scaled_second,
        )
        .unwrap();
    trace_step!(scaled_squared);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("до={raw_squared}, после масштабирования={scaled_squared}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_squared_distances_before_and_after_feature_scaling(raw_squared, scaled_squared);
}

// Строим график по результатам урока.
fn plot_squared_distances_before_and_after_feature_scaling(raw_squared: f64, scaled_squared: f64) {
    trace_note!("Сравнение величин из этого урока.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Эффект масштабирования",
        "квадрат расстояния",
        &[("до", raw_squared), ("после", scaled_squared)],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
