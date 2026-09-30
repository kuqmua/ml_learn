// Урок 17.3. Выбор настроек на внутренних группах данных и проверка на внешних.
// Связь с принятой терминологией: Вложенная кросс-валидация для выбора параметров и оценки качества.
// Зачем здесь эта тема: Выбор параметров по тем же проверочным данным делает оценку оптимистичной.
// Почему код устроен так: Внутренние блоки выбирают параметры, а внешние оценивают уже выбранный
//   алгоритм.
// Представь: Если выбрали лучший параметр по validation, тот же validation уже не даёт независимой
//   оценки качества.
//
// Что изучаем: Вложенная оценка.
// Зачем это нужно: Внутреннее разбиение выбирает гиперпараметр, а внешнее измеряет качество выбранной
// процедуры на новых данных.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `inner_scores` для следующего шага примера.");
    let inner_scores: [(usize, f64); 3] = [(1, 0.70), (3, 0.85), (5, 0.80)];
    trace_step!(inner_scores);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !inner_scores.is_empty(),
        "для выбора k нужна хотя бы одна оценка"
    );
    trace_note!("Сохраняем рассчитанное значение `best` для следующих операций.");
    trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    trace_note!("Выбираем k с максимальной оценкой на внутренней проверке.");
    trace_note!("Извлекаем значение: выше в примере обеспечено отсутствие ошибки.");
    let best: &(usize, f64) = inner_scores
        .iter()
        .max_by(|first_candidate, second_candidate| {
            first_candidate.1.total_cmp(&second_candidate.1)
        })
        .unwrap();
    trace_step!(best);
    trace_note!("Внешние метки не участвовали в выборе k: они нужны только для итоговой оценки.");
    let outer_truth: [bool; 4] = [true, false, true, false];
    trace_step!(outer_truth);
    trace_note!("Создаём набор значений `outer_predictions` для следующего шага примера.");
    let outer_predictions: [bool; 4] = [true, false, false, false];
    trace_step!(outer_predictions);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert_eq!(
        outer_truth.len(),
        outer_predictions.len(),
        "число прогнозов должно совпадать с числом ответов"
    );
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !outer_truth.is_empty(),
        "для внешней оценки нужен хотя бы один пример"
    );
    trace_note!("Считаем количество элементов и сохраняем его в `outer_correct`.");
    trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
    trace_note!("Подсчитываем число элементов после отбора.");
    let outer_correct: usize = (0..outer_truth.len())
        .filter(|&index| outer_truth[index] == outer_predictions[index])
        .count();
    trace_step!(outer_correct);
    trace_note!("Считаем количество элементов и сохраняем его в `outer_test_accuracy`.");
    let outer_test_accuracy: f64 = outer_correct as f64 / outer_truth.len() as f64;
    trace_step!(outer_test_accuracy);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    trace_note!("Присваиваем вычисленное значение соответствующей переменной или полю.");
    trace_note!("Печатаем выбранное число соседей рядом с внешней оценкой.");
    println!(
        "внутри выбрано k={}, внешняя оценка={outer_test_accuracy}",
        best.0
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_validation_score_for_different_neighbor_counts(inner_scores);
}

// Строим график по результатам урока.
fn plot_validation_score_for_different_neighbor_counts(inner_scores: [(usize, f64); 3]) {
    trace_note!("Значения из этого урока на графике.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let nested_evaluation_points: Vec<(f64, f64)> = inner_scores
        .iter()
        .map(|&(neighbor_count, score)| (neighbor_count as f64, score))
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
        "Выбор k внутри вложенной оценки",
        "k",
        "внутренняя оценка",
        &[lesson_visualization::Series {
            name: "validation",

            points: &nested_evaluation_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
