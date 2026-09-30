// Урок 06.4. Квантиль: значение на заданной доле упорядоченной выборки.
// Связь с принятой терминологией: Квантиль упорядоченных числовых значений.
// Зачем здесь эта тема: Один центр и один разброс не показывают хвосты; квантиль задаёт порог для
//   выбранной доли наблюдений.
// Почему код устроен так: Работаем с упорядоченными числами и явно выбираем индекс, чтобы правило
//   квантиля было проверяемым.
// Представь: Медиана — частный случай квантиля: половина упорядоченных значений находится не выше
//   неё.
//
// Используем ближайший порядковый элемент с индексом floor((n−1)·доля).
// Доля 0 даёт минимум, 1 — максимум; между ними выбирается элемент внутри ряда.

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Задаём учебные значения для `values`.");
    let mut values: [i32; 5] = [9, 1, 7, 3, 5];
    trace_step!(values);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(!values.is_empty(), "для квантиля нужна непустая выборка");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    values.sort();
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    trace_note!("Долю наблюдений от 0 до 1 называют fraction; она задаёт положение quantile.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, target_share_below_cutoff, expected) in [
        ("минимум", 0.0, 1),
        ("середина", 0.5, 5),
        ("три четверти", 0.75, 7),
        ("максимум", 1.0, 9),
    ] {
        trace_step!(description);
        trace_step!(target_share_below_cutoff);
        trace_step!(expected);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        trace_note!("Обновляем значение результатом текущего вычисления.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            (0.0..=1.0).contains(&target_share_below_cutoff),
            "доля должна быть от 0 до 1"
        );
        trace_note!("Определяем размер данных и сохраняем его в `index`.");
        let index: usize = ((values.len() - 1) as f64 * target_share_below_cutoff) as usize;
        trace_step!(index);
        trace_note!("Сохраняем результат этого шага в `distribution_cutoff_value`.");
        trace_note!("Границу, ниже которой лежит заданная доля наблюдений, называют quantile.");
        let distribution_cutoff_value: i32 = values[index];
        trace_step!(distribution_cutoff_value);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(distribution_cutoff_value, expected);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: доля {target_share_below_cutoff} → {distribution_cutoff_value}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_quantiles_as_sorted_values_at_each_fraction_of_sample();
}

// Строим график по результатам урока.
fn plot_quantiles_as_sorted_values_at_each_fraction_of_sample() {
    trace_note!("Наглядное представление величин из этого урока.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let distribution_cutoff_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (
                probability,
                [1.0, 2.0, 3.0, 4.0, 5.0][((probability * 4.0) as usize).min(4)],
            )
        })
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Квантили выборки",
        "доля",
        "квантиль",
        &[lesson_visualization::Series {
            name: "значения 1, 2, 3, 4, 5",

            points: &distribution_cutoff_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
