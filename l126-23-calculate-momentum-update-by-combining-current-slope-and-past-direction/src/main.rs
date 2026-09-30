// Урок 23.2. Обновление с накоплением направления: объединение текущего и прошлых изменений.
// Связь с принятой терминологией: Обновление параметра с учётом текущего и прошлых градиентов.
// Зачем здесь эта тема: Шумный SGD может колебаться; momentum накапливает направление прошлых
//   шагов.
// Почему код устроен так: Храним скорость и обновляем её из текущего градиента и предыдущего
//   состояния.
// Представь: Если несколько градиентов подряд указывают вправо, momentum накапливает это
//   направление.
//
// Что изучаем: Импульс momentum.
// Зачем это нужно: Скорость накапливает прежние градиенты и сглаживает последовательность обновлений.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Создаём набор значений `rates_of_change` для следующего шага примера."
    );
    lesson_trace::trace_note!(
        "Производную функции по параметру или вектор таких производных называют gradient."
    );
    let rates_of_change: [f64; 3] = [2.0, 1.0, -0.5];
    lesson_trace::trace_step!(rates_of_change);
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `velocity` начальным состоянием."
    );
    let mut velocity: f64 = 0.0;
    lesson_trace::trace_step!(velocity);
    lesson_trace::trace_note!("Создаём изменяемое значение `weight` для следующих операций.");
    let mut weight: f64 = 1.0;
    lesson_trace::trace_step!(weight);
    lesson_trace::trace_note!("Собираем значения для `weight_history` в коллекцию.");
    let mut weight_history: Vec<(f64, f64)> = vec![(0.0, weight)];
    lesson_trace::trace_step!(weight_history);
    lesson_trace::trace_note!("Собираем значения для `velocity_history` в коллекцию.");
    let mut velocity_history: Vec<(f64, f64)> = vec![(0.0, velocity)];
    lesson_trace::trace_step!(velocity_history);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for (step, rate_of_change) in rates_of_change.into_iter().enumerate() {
        lesson_trace::trace_step!(step);
        lesson_trace::trace_step!(rate_of_change);
        lesson_trace::trace_note!(
            "Присваиваем вычисленное значение соответствующей переменной или полю."
        );
        velocity = 0.8 * velocity + rate_of_change;
        lesson_trace::trace_step!(velocity);
        lesson_trace::trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
        weight -= 0.1 * velocity;
        lesson_trace::trace_step!(weight);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("градиент={rate_of_change}, скорость={velocity}, вес={weight}");
        lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
        weight_history.push(((step + 1) as f64, weight));
        lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
        velocity_history.push(((step + 1) as f64, velocity));
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_weight_and_accumulated_update_direction(weight_history, velocity_history);
}

// Строим график по результатам урока.
fn plot_weight_and_accumulated_update_direction(
    weight_history: std::vec::Vec<(f64, f64)>,
    velocity_history: std::vec::Vec<(f64, f64)>,
) {
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Momentum: накопление скорости",
        "шаг",
        "значение",
        &[
            lesson_visualization::Series {
                name: "вес",

                points: &weight_history,
            },
            lesson_visualization::Series {
                name: "скорость",

                points: &velocity_history,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
