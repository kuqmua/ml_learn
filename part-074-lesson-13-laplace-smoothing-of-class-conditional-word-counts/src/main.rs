// Урок 13.3. Сглаживание Лапласа для частот слов при известном классе.
// Зачем здесь эта тема: Нулевое число наблюдений слова не должно обнулять вероятность всего текста.
// Почему код устроен так: Добавляем псевдосчётчик к частотам и пересчитываем знаменатель для
//   каждого класса.
// Представь: Если слово ни разу не встретилось в классе, добавление единицы не даёт вероятности
//   стать нулём.
//
// Что изучаем: Сглаживание Лапласа.
// Зачем это нужно: Прибавление единицы к частотам не даёт неизвестному слову обнулить вероятность всего
// текста.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Инициализируем значение `observed_count` начальным состоянием.
    let observed_count: f64 = 0.0;
    lesson_trace::trace_step!(observed_count);
    // Сохраняем рассчитанное значение `total_words_in_class` для следующих операций.
    let total_words_in_class: f64 = 8.0;
    lesson_trace::trace_step!(total_words_in_class);
    // Сохраняем рассчитанное значение `known_text_unit_count` для следующих операций.
    // Набор известных модели текстовых единиц называют vocabulary.
    let known_text_unit_count: f64 = 4.0;
    lesson_trace::trace_step!(known_text_unit_count);
    // Нормируем или усредняем величину делением и сохраняем её в `unsmoothed`.
    let unsmoothed: f64 = observed_count / total_words_in_class;
    lesson_trace::trace_step!(unsmoothed);
    // Нормируем или усредняем величину делением и сохраняем её в `smoothed`.
    let smoothed: f64 = (observed_count + 1.0) / (total_words_in_class + known_text_unit_count);
    lesson_trace::trace_step!(smoothed);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("без сглаживания={unsmoothed}, со сглаживанием={smoothed}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_laplace_smoothing_of_class_conditional_word_counts();
}

// Строим график по результатам урока.
fn visualize_laplace_smoothing_of_class_conditional_word_counts() {
    // График величин и зависимостей, изученных в этом уроке.
    let unsmoothed_probability_points: Vec<(f64, f64)> = (0..=8)
        .map(|sample_count| (sample_count as f64, sample_count as f64 / 10.0))
        .collect();
    // Собираем значения для `smoothed_probability_points` в коллекцию.
    let smoothed_probability_points: Vec<(f64, f64)> = (0..=8)
        // Преобразуем каждый элемент в новое значение.
        .map(|sample_count| (sample_count as f64, (sample_count as f64 + 1.0) / 12.0))
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Сглаживание Лапласа",
        // Указываем подпись горизонтальной оси.
        "частота токена",
        // Указываем подпись вертикальной оси.
        "оценка вероятности",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "без сглаживания",
                // Передаём рассчитанные координаты точек.
                points: &unsmoothed_probability_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "со сглаживанием",
                // Передаём рассчитанные координаты точек.
                points: &smoothed_probability_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
