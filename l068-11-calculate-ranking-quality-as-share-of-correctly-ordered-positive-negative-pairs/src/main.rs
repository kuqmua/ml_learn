// Урок 11.5. Качество ранжирования (ROC-AUC): доля правильно упорядоченных положительных и отрицательных пар.
// Связь с принятой терминологией: Площадь под ROC кривой по парам положительных и отрицательных оценок.
// Зачем здесь эта тема: Метрика при одном пороге не показывает качество ранжирования всех
//   положительных относительно отрицательных.
// Почему код устроен так: Сравниваем пары оценок разных классов: верный порядок увеличивает ROC
//   AUC.
// Представь: Если положительные примеры обычно получают оценку выше отрицательных, ранжирование
//   работает даже до выбора порога.
//
// Значение 1 означает, что каждая положительная оценка выше отрицательной; 0 — наоборот.
// При равных оценках даём паре половину балла, как в стандартном определении ROC-AUC.

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], &[f64], f64); 4] = [
        ("идеальный порядок", &[0.9, 0.7], &[0.6, 0.2], 1.0),
        ("обратный порядок", &[0.1, 0.2], &[0.8, 0.9], 0.0),
        ("одинаковые оценки", &[0.5, 0.5], &[0.5, 0.5], 0.5),
        ("смешанный порядок", &[0.8, 0.2], &[0.6, 0.4], 0.5),
    ];
    trace_step!(cases);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, positive_scores, negative_scores, expected) in cases {
        trace_step!(description);
        trace_step!(positive_scores);
        trace_step!(negative_scores);
        trace_step!(expected);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            !positive_scores.is_empty() && !negative_scores.is_empty(),
            "для ROC-AUC нужны оба класса"
        );
        trace_note!("Сохраняем результат этого шага в `ordered_pairs`.");
        let mut ordered_pairs: f64 = 0.0;
        trace_step!(ordered_pairs);
        trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for &positive in positive_scores {
            trace_step!(positive);
            trace_note!("Повторяем расчёт для каждого элемента последовательности.");
            for &negative in negative_scores {
                trace_step!(negative);
                trace_note!("Обновляем значение результатом текущего вычисления.");
                ordered_pairs += if positive > negative {
                    trace_note!("Используем подготовленное значение в следующем шаге примера.");
                    trace_note!("Обновляем значение результатом текущего вычисления.");
                    1.0
                } else if positive == negative {
                    trace_note!("Используем подготовленное значение в следующем шаге примера.");
                    0.5
                } else {
                    trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                    trace_note!("Используем подготовленное значение в следующем шаге примера.");
                    0.0
                };
                trace_step!(ordered_pairs);
            }
        }
        trace_note!("Определяем размер данных и сохраняем его в `pair_count`.");
        let pair_count: f64 = (positive_scores.len() * negative_scores.len()) as f64;
        trace_step!(pair_count);
        trace_note!("Сохраняем результат этого шага в `area_under_curve`.");
        let area_under_curve: f64 = ordered_pairs / pair_count;
        trace_step!(area_under_curve);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(area_under_curve, expected);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: ROC-AUC = {area_under_curve}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_detected_positive_share_against_false_positive_share();
}

// Строим график по результатам урока.
fn plot_detected_positive_share_against_false_positive_share() {
    trace_note!("Значения из этого урока на графике.");
    let ideal_receiver_operating_characteristic_points: Vec<(f64, f64)> =
        [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)].to_vec();
    trace_note!(
        "Собираем значения для `random_receiver_operating_characteristic_points` в коллекцию."
    );
    let random_receiver_operating_characteristic_points: Vec<(f64, f64)> =
        [(0.0, 0.0), (1.0, 1.0)].to_vec();
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
        "ROC-кривая: идеальное ранжирование",
        "доля ложных срабатываний",
        "полнота",
        &[
            lesson_visualization::Series {
                name: "идеал",

                points: &ideal_receiver_operating_characteristic_points,
            },
            lesson_visualization::Series {
                name: "случайный порядок",

                points: &random_receiver_operating_characteristic_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
