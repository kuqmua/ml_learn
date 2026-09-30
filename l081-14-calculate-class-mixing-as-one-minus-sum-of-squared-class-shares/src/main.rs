// Урок 14.2. Смешанность классов (мера Джини): единица минус сумма квадратов долей классов.
// Связь с принятой терминологией: Нечистота Джини по долям классов в узле дерева решений.
// Зачем здесь эта тема: Нечистота Джини даёт другой простой критерий выбора разбиения дерева.
// Почему код устроен так: Сравниваем суммы квадратов долей с энтропией на одинаковых составах узла.
// Представь: Чистый узел имеет Джини 0; для равных долей двух классов нечистота выше.
//
// Что изучаем: Нечистота Gini.
// Зачем это нужно: Gini равна единице минус сумма квадратов долей классов; чистому узлу соответствует
// ноль.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    lesson_trace::trace_note!("Долю объектов одного класса среди всех объектов называют fraction.");
    for positive_class_share in [0.0, 0.5, 1.0] {
        lesson_trace::trace_step!(positive_class_share);
        lesson_trace::trace_note!(
            "Комбинируем исходные величины и сохраняем результат в `negative_class_share`."
        );
        let negative_class_share: f64 = 1.0 - positive_class_share;
        lesson_trace::trace_step!(negative_class_share);
        lesson_trace::trace_note!("Сохраняем рассчитанное значение `gini` для следующих операций.");
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        let gini: f64 = 1.0
            - positive_class_share * positive_class_share
            - negative_class_share * negative_class_share;
        lesson_trace::trace_step!(gini);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("доля положительных={positive_class_share}, Gini={gini}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_class_mixing_as_twice_positive_share_times_negative_share();
}

// Строим график по результатам урока.
fn plot_class_mixing_as_twice_positive_share_times_negative_share() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let gini_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (probability, 2.0 * probability * (1.0 - probability))
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
        "Нечистота Gini",
        "доля положительных",
        "Gini",
        &[lesson_visualization::Series {
            name: "Gini(p)",

            points: &gini_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
