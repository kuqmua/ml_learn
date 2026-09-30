// Урок 06.6. Изменчивость оценки: повторный набор выборок с возвращением наблюдений.
// Связь с принятой терминологией: Повторные выборки с возвращением из наблюдений.
// Зачем здесь эта тема: Неопределённость можно оценить повторными выборками без новой ручной
//   формулы для каждой статистики.
// Почему код устроен так: Выбираем наблюдения с возвращением, чтобы каждая повторная выборка имела
//   исходный размер.
// Представь: Из [2, 4, 6] повторная выборка может быть [2, 2, 6]: повторение разрешено.
//
// Что изучаем: Bootstrap.
// Зачем это нужно: Повторная выборка с возвращением показывает, как меняется оценка при перестановке
// наблюдений.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `values` для следующего шага примера.");
    let values: [f64; 3] = [2.0, 4.0, 6.0];
    lesson_trace::trace_step!(values);
    lesson_trace::trace_note!("Создаём набор значений `resamples` для следующего шага примера.");
    let resamples: [[usize; 3]; 4] = [[0, 1, 2], [0, 0, 2], [1, 2, 2], [0, 1, 1]];
    lesson_trace::trace_step!(resamples);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(!values.is_empty(), "исходная выборка не должна быть пустой");
    lesson_trace::trace_note!("Повторяем следующий блок для каждой повторной выборки.");
    for indices in resamples {
        lesson_trace::trace_step!(indices);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            !indices.is_empty(),
            "повторная выборка не должна быть пустой"
        );
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            indices.iter().all(|&index| index < values.len()),
            "индекс выходит за границы исходной выборки"
        );
        lesson_trace::trace_note!(
            "Инициализируем изменяемый накопитель `sum` начальным состоянием."
        );
        let mut sum: f64 = 0.0;
        lesson_trace::trace_step!(sum);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for index in indices {
            lesson_trace::trace_step!(index);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            sum += values[index];
            lesson_trace::trace_step!(sum);
        }
        lesson_trace::trace_note!("Считаем количество элементов и сохраняем его в `mean`.");
        let mean: f64 = sum / indices.len() as f64;
        lesson_trace::trace_step!(mean);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("индексы {indices:?} -> среднее {mean:.2}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_means_of_samples_drawn_with_replacement(values, resamples);
}

// Строим график по результатам урока.
fn plot_means_of_samples_drawn_with_replacement(values: [f64; 3], resamples: [[usize; 3]; 4]) {
    lesson_trace::trace_note!("Показываем значения, рассчитанные по данным примера.");
    lesson_trace::trace_note!("Повторную выборку с возвращением называют bootstrap sample.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Добавляем порядковый номер к каждому элементу.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let resampled_mean_points: Vec<(f64, f64)> = resamples
        .iter()
        .enumerate()
        .map(|(item_index, indices)| {
            lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
            lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
            (
                (item_index + 1) as f64,
                indices
                    .iter()
                    .map(|&sample_index| values[sample_index])
                    .sum::<f64>()
                    / indices.len() as f64,
            )
        })
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Bootstrap: средние повторных выборок",
        "номер выборки",
        "среднее",
        &[lesson_visualization::Series {
            name: "среднее",

            points: &resampled_mean_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
