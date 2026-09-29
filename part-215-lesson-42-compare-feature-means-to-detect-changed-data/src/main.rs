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

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `reference`.
    let reference: [f64; 3] = [1.0, 2.0, 3.0];
    lesson_trace::trace_step!(reference);
    // Задаём учебные значения для `cases`.
    let cases: [(&str, [f64; 3], f64); 3] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("без сдвига среднего", [3.0, 2.0, 1.0], 0.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("сдвиг к большим значениям", [5.0, 6.0, 7.0], 4.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("то же среднее, другой разброс", [0.0, 2.0, 4.0], 0.0),
    ];
    lesson_trace::trace_step!(cases);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(!reference.is_empty());
    // Сохраняем результат этого шага в `reference_mean`.
    let reference_mean: f64 =
        part_029_lesson_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count(
            &reference,
        )
        .unwrap();
    lesson_trace::trace_step!(reference_mean);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, current, expected_difference) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(current);
        lesson_trace::trace_step!(expected_difference);
        // Проверяем ожидаемое свойство учебного примера.
        assert!(!current.is_empty());
        // Сохраняем результат этого шага в `current_mean`.
        let current_mean: f64 =
            part_029_lesson_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count(
                &current,
            )
            .unwrap();
        lesson_trace::trace_step!(current_mean);
        // Сохраняем результат этого шага в `difference`.
        let difference: f64 = current_mean - reference_mean;
        lesson_trace::trace_step!(difference);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(difference, expected_difference);
        // Печатаем рассчитанные значения для проверки примера.
        println!(
            // Передаём подпись или текстовое значение для следующего шага.
            "{description}: эталон={reference_mean}, новые данные={current_mean}, разница={difference}"
        );
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_reference_shifted_and_more_spread_out_feature_values(reference, cases);
}

// Строим график по результатам урока.
fn plot_reference_shifted_and_more_spread_out_feature_values(
    reference: [f64; 3],
    cases: [(&str, [f64; 3], f64); 3],
) {
    // Показываем значения, рассчитанные по данным примера.
    let reference_distribution_points: Vec<(f64, f64)> = reference
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `shifted_distribution_points` в коллекцию.
    let shifted_distribution_points: Vec<(f64, f64)> = cases[1]
        // Настраиваем или преобразуем результат предыдущего шага.
        .1
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `spread_distribution_points` в коллекцию.
    let spread_distribution_points: Vec<(f64, f64)> = cases[2]
        // Настраиваем или преобразуем результат предыдущего шага.
        .1
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Сдвиг среднего",
        // Указываем подпись горизонтальной оси.
        "номер наблюдения",
        // Указываем подпись вертикальной оси.
        "значение",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "эталон",
                // Передаём рассчитанные координаты точек.
                points: &reference_distribution_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "сдвиг",
                // Передаём рассчитанные координаты точек.
                points: &shifted_distribution_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "изменение разброса",
                // Передаём рассчитанные координаты точек.
                points: &spread_distribution_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
