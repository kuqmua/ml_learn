// Урок 12.3. Выбор числа соседей для классификации kNN.
// Почему этот урок сейчас: Один сосед чувствителен к шуму, а слишком большое k размывает локальную структуру.
// Почему пример устроен так: Сравниваем голосование нескольких ближайших точек при разных k.
//
// Что изучаем: Выбор числа соседей k.
// Зачем это нужно: Малое k делает решение чувствительным к одной точке, большое сглаживает локальные
// различия.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Соседи отсортированы от ближайшего к дальнему.
    let neighbor_labels: [bool; 5] = [true, false, false, true, true];
    lesson_trace::trace_step!(neighbor_labels);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for neighbor_count in [1, 3, 5] {
        lesson_trace::trace_step!(neighbor_count);
        // Преобразуем входные данные и сохраняем полученную коллекцию в `positive`.
        let positive: usize = neighbor_labels[..neighbor_count]
            .iter()
            .filter(|&&label| label)
            .count();
        lesson_trace::trace_step!(positive);
        // Умножаем значения и сохраняем результат в `prediction`.
        let prediction: bool = positive * 2 > neighbor_count;
        lesson_trace::trace_step!(prediction);
        // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
        println!("k={neighbor_count}: прогноз={prediction}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_choose_number_of_neighbors_for_knn_classification(neighbor_labels);
}

// Строим график по результатам урока.
fn visualize_choose_number_of_neighbors_for_knn_classification(neighbor_labels: [bool; 5]) {
    // Показываем значения, рассчитанные по данным примера.
    let positive_neighbor_count_points: Vec<(f64, f64)> = [1usize, 3, 5]
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Преобразуем каждый элемент в новое значение.
        .map(|&neighbor_count| {
            (
                // Используем подготовленное значение в следующем шаге примера.
                neighbor_count as f64,
                // Вычисляем значение по указанной формуле.
                neighbor_labels[..neighbor_count]
                    .iter()
                    .filter(|&&element_value| element_value)
                    .count() as f64
                    / neighbor_count as f64,
            )
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `decision_boundary_points` в коллекцию.
    let decision_boundary_points: Vec<(f64, f64)> = [(1.0, 0.5), (5.0, 0.5)].to_vec();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Соседи и доля положительных",
        // Указываем подпись горизонтальной оси.
        "k",
        // Указываем подпись вертикальной оси.
        "доля",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "положительные среди k",
                // Передаём рассчитанные координаты точек.
                points: &positive_neighbor_count_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "граница решения",
                // Передаём рассчитанные координаты точек.
                points: &decision_boundary_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
