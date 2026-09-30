// Урок 42.1. Сравнение средних значений признаков для обнаружения изменений данных.
// Связь с принятой терминологией: Сравнение средних признаков для обнаружения сдвига распределения.
// Зачем здесь эта тема: После выпуска модели входные данные могут измениться без изменения кода.
// Почему код устроен так: Сравниваем статистики признаков старых и новых строк при одинаковой
//   схеме.
// Представь: Если после выпуска средний возраст входных клиентов изменился, это повод проверить
//   модель.
//
// Сравнение средних — первый сигнал: при похожих данных разница мала, при сдвиге растёт.
// Совпадение средних само по себе не доказывает совпадения распределений.

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `reference`.");
    let reference: [f64; 3] = [1.0, 2.0, 3.0];
    trace_step!(reference);
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 3], f64); 3] = [
        ("без сдвига среднего", [3.0, 2.0, 1.0], 0.0),
        ("сдвиг к большим значениям", [5.0, 6.0, 7.0], 4.0),
        ("то же среднее, другой разброс", [0.0, 2.0, 4.0], 0.0),
    ];
    trace_step!(cases);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(!reference.is_empty());
    trace_note!("Сохраняем результат этого шага в `reference_mean`.");
    let reference_mean: f64 =
        l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count(
            &reference,
        )
        .unwrap();
    trace_step!(reference_mean);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, current, expected_difference) in cases {
        trace_step!(description);
        trace_step!(current);
        trace_step!(expected_difference);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(!current.is_empty());
        trace_note!("Сохраняем результат этого шага в `current_mean`.");
        let current_mean: f64 =
            l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count(
                &current,
            )
            .unwrap();
        trace_step!(current_mean);
        trace_note!("Сохраняем результат этого шага в `difference`.");
        let difference: f64 = current_mean - reference_mean;
        trace_step!(difference);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(difference, expected_difference);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        println!(
            "{description}: эталон={reference_mean}, новые данные={current_mean}, разница={difference}"
        );
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_reference_shifted_and_more_spread_out_feature_values(reference, cases);
}

// Строим график по результатам урока.
fn plot_reference_shifted_and_more_spread_out_feature_values(
    reference: [f64; 3],
    cases: [(&str, [f64; 3], f64); 3],
) {
    trace_note!("Показываем значения, рассчитанные по данным примера.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let reference_distribution_points: Vec<(f64, f64)> = reference
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    trace_note!("Собираем значения для `shifted_distribution_points` в коллекцию.");
    trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let shifted_distribution_points: Vec<(f64, f64)> = cases[1]
        .1
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    trace_note!("Собираем значения для `spread_distribution_points` в коллекцию.");
    trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let spread_distribution_points: Vec<(f64, f64)> = cases[2]
        .1
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
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сдвиг среднего",
        "номер наблюдения",
        "значение",
        &[
            lesson_visualization::Series {
                name: "эталон",

                points: &reference_distribution_points,
            },
            lesson_visualization::Series {
                name: "сдвиг",

                points: &shifted_distribution_points,
            },
            lesson_visualization::Series {
                name: "изменение разброса",

                points: &spread_distribution_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
