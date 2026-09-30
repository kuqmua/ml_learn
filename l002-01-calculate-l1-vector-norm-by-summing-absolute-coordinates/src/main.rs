// Урок 01.2. Длина пути вдоль осей (норма L1): сложение модулей координат вектора.
// Связь с принятой терминологией: Сумма модулей координат одного вектора (норма L1).
// Зачем здесь эта тема: После произведения векторов нужна величина одного вектора; L1 измеряет
//   общий модуль координат без взаимного сокращения знаков.
// Почему код устроен так: Складываем модули явно, чтобы отрицательные координаты не уменьшали
//   норму.
// Представь: Если координаты равны −3 и 4, общий размер по L1 равен 3+4=7: минус не уменьшает
//   ответ.
//
// Что изучаем: складываем модули всех координат. Отрицательное число даёт положительный вклад,
// поэтому смена знаков не меняет ответ. У нулевого вектора результат равен нулю.

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 2], f64); 4] = [
        ("положительные координаты", [3.0, 4.0], 7.0),
        ("смешанные знаки", [3.0, -4.0], 7.0),
        ("сменили оба знака", [-3.0, 4.0], 7.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    trace_step!(cases);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, vector, expected) in cases {
        trace_step!(description);
        trace_step!(vector);
        trace_step!(expected);
        trace_note!("Формула из общей библиотеки пригодится и в сводной практике.");
        let calculate_l1_vector_norm_by_summing_absolute_coordinates: f64 =
            l002_01_calculate_l1_vector_norm_by_summing_absolute_coordinates::calculate_l1_vector_norm_by_summing_absolute_coordinates(
                &vector,
            );
        trace_step!(calculate_l1_vector_norm_by_summing_absolute_coordinates);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(
            calculate_l1_vector_norm_by_summing_absolute_coordinates,
            expected
        );
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!(
            "{description}: {vector:?} → L1 = {calculate_l1_vector_norm_by_summing_absolute_coordinates}"
        );
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_l1_vector_norm_as_absolute_coordinate_sum_for_changing_first_coordinate();
}

// Строим график по результатам урока.
fn plot_l1_vector_norm_as_absolute_coordinate_sum_for_changing_first_coordinate() {
    trace_note!("Наглядное представление величин из этого урока.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let sum_absolute_values_of_vector_coordinates_points: Vec<(f64, f64)> = (-50..=50)

        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (
                horizontal_value,
                l002_01_calculate_l1_vector_norm_by_summing_absolute_coordinates::calculate_l1_vector_norm_by_summing_absolute_coordinates(
                    &[horizontal_value, 4.0],
                ),
            )
        })

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
        "Сумма модулей координат одного вектора (норма L1)",
        "первая координата",
        "L1",
        &[lesson_visualization::Series {
            name: "вектор [x, 4]",

            points: &sum_absolute_values_of_vector_coordinates_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
