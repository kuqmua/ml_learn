// Урок 10.3. Преобразование взвешенного входа со смещением в вероятность положительного класса.
// Связь с принятой терминологией: Вероятность положительного класса из логистической модели.
// Зачем здесь эта тема: Логистическая модель связывает признаки с вероятностью через линейный логит
//   и сигмоиду.
// Почему код устроен так: Показываем оба шага отдельно, чтобы вес признака не смешивался с порогом
//   решения.
// Представь: Вес превращает признак в логит, сигмоида — логит в вероятность; это два разных шага.
//
// Что изучаем: Вероятность класса.
// Зачем это нужно: Вероятность класса описывает уверенность модели и позволяет менять решение без
// повторного обучения.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Инициализируем значение `probability_positive` начальным состоянием.");
    let probability_positive: f64 = 0.7;
    trace_step!(probability_positive);
    trace_note!("Комбинируем исходные величины и сохраняем результат в `probability_negative`.");
    let probability_negative: f64 = 1.0 - probability_positive;
    trace_step!(probability_negative);
    trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
    assert!(probability_positive >= 0.0 && probability_positive <= 1.0);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("P(y=1)={probability_positive}, P(y=0)={probability_negative}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_positive_and_negative_class_probabilities();
}

// Строим график по результатам урока.
fn plot_positive_and_negative_class_probabilities() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let positive_class_probability_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (probability, probability)
        })
        .collect();
    trace_note!("Собираем значения для `negative_class_probability_points` в коллекцию.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let negative_class_probability_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (probability, 1.0 - probability)
        })
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
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Вероятности двух классов",
        "P(y=1)",
        "вероятность",
        &[
            lesson_visualization::Series {
                name: "положительный",

                points: &positive_class_probability_points,
            },
            lesson_visualization::Series {
                name: "отрицательный",

                points: &negative_class_probability_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
