// Урок 06.1. Среднее арифметическое: сложение значений и деление суммы на их количество.
// Связь с принятой терминологией: Среднее арифметическое числовых значений.
// Зачем здесь эта тема: После вероятностей нужны сводки наблюдений; среднее задаёт один показатель
//   центра данных.
// Почему код устроен так: Суммируем значения и делим на число наблюдений, отдельно отклоняя пустой
//   набор.
// Представь: Для чисел 2, 4, 6 среднее равно 4; добавление очень большого числа заметно сдвинет
//   его.
//
// Складываем значения и делим на их количество. Для одного значения среднее равно ему,
// отрицательные числа могут уменьшить среднее, а для пустого набора делить не на что.

use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], f64); 3] = [
        ("несколько положительных", &[2.0, 4.0, 6.0], 4.0),
        ("одно значение", &[7.0], 7.0),
        ("значения разных знаков", &[-2.0, 2.0], 0.0),
    ];
    trace_step!(cases);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, values, expected) in cases {
        trace_step!(description);
        trace_step!(values);
        trace_step!(expected);
        trace_note!("Общая функция среднего повторно понадобится в дисперсии и нормализации.");
        let mean: f64 = calculate_mean_by_summing_values_and_dividing_by_count(values)
            .expect("в этой строке есть значения");
        trace_step!(mean);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(mean, expected);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {values:?} → среднее {mean}");
    }
    trace_note!("Задаём учебные значения для `empty`.");
    let empty: [f64; 0] = [];
    trace_step!(empty);
    trace_note!("Сохраняем результат этого шага в `error`.");
    trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    let error: &str = calculate_mean_by_summing_values_and_dividing_by_count(&empty)
        .expect_err("среднее пустого набора должно быть отклонено");
    trace_step!(error);
    trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("пустой набор: {error}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_values_and_mean_as_their_sum_divided_by_count();
}

// Строим график по результатам урока.
fn plot_values_and_mean_as_their_sum_divided_by_count() {
    trace_note!("Наглядное представление величин из этого урока.");
    let observation_points: Vec<(f64, f64)> = [(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)].to_vec();
    trace_note!("Собираем значения для `mean_points` в коллекцию.");
    let mean_points: Vec<(f64, f64)> = [(1.0, 4.0), (3.0, 4.0)].to_vec();
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
        "Среднее и отдельные значения",
        "индекс",
        "значение",
        &[
            lesson_visualization::Series {
                name: "наблюдения",

                points: &observation_points,
            },
            lesson_visualization::Series {
                name: "среднее",

                points: &mean_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
