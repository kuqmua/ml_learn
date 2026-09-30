// Урок 14.1. Неопределённость класса (энтропия): отрицательная сумма долей, умноженных на их логарифмы.
// Связь с принятой терминологией: Энтропия долей классов в узле дерева решений.
// Зачем здесь эта тема: Дерево выбирает разбиение по смешанности классов в узле; энтропия даёт одну
//   меру смешанности.
// Почему код устроен так: Считаем вклад каждой доли класса и сравниваем чистый и смешанный узлы.
// Представь: Узел только с классом A чистый; смесь A и B даёт большую энтропию.
//
// У чистого узла энтропия равна нулю, при долях 50/50 она максимальна.
// Нулевую долю пропускаем: предел p·log(p) при p→0 равен нулю.

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    trace_note!("Долю объектов одного класса среди всех объектов называют fraction.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, positive_class_share) in [
        ("только отрицательный класс", 0.0),
        ("четверть положительных", 0.25),
        ("классы поровну", 0.5),
        ("только положительный класс", 1.0),
    ] {
        trace_step!(description);
        trace_step!(positive_class_share);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((0.0..=1.0).contains(&positive_class_share));
        trace_note!("Сохраняем результат этого шага в `negative_class_share`.");
        let negative_class_share: f64 = 1.0 - positive_class_share;
        trace_step!(negative_class_share);
        trace_note!("Сохраняем результат этого шага в `uncertainty_measure`.");
        trace_note!("Меру неопределённости распределения называют entropy.");
        let mut uncertainty_measure: f64 = 0.0;
        trace_step!(uncertainty_measure);
        trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for probability in [positive_class_share, negative_class_share] {
            trace_step!(probability);
            trace_note!("Выбираем дальнейший шаг по выполнению условия.");
            if probability > 0.0 {
                trace_note!("Сохраняем результат этого шага в `ratio`.");
                let ratio: f64 = (probability - 1.0) / (probability + 1.0);
                trace_step!(ratio);
                trace_note!("Сохраняем результат этого шага в `term`.");
                let mut term: f64 = ratio;
                trace_step!(term);
                trace_note!("Сохраняем результат этого шага в `logarithm`.");
                let mut logarithm: f64 = 0.0;
                trace_step!(logarithm);
                trace_note!("Повторяем расчёт для каждого элемента последовательности.");
                for odd_divisor in (1..=99).step_by(2) {
                    trace_step!(odd_divisor);
                    trace_note!("Обновляем значение результатом текущего вычисления.");
                    logarithm += term / odd_divisor as f64;
                    trace_step!(logarithm);
                    trace_note!("Обновляем значение результатом текущего вычисления.");
                    term *= ratio * ratio;
                    trace_step!(term);
                }
                trace_note!("Переводим натуральный логарифм в логарифм по основанию 2.");
                uncertainty_measure -= probability * (2.0 * logarithm) / std::f64::consts::LN_2;
                trace_step!(uncertainty_measure);
            }
        }
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(uncertainty_measure >= -1e-10 && uncertainty_measure <= 1.0 + 1e-10);
        trace_note!("Выбираем дальнейший шаг по выполнению условия.");
        if positive_class_share == 0.0 || positive_class_share == 1.0 {
            trace_note!("Проверяем ожидаемое свойство учебного примера.");
            assert!(uncertainty_measure.abs() < 1e-10);
        }
        trace_note!("Выбираем дальнейший шаг по выполнению условия.");
        if positive_class_share == 0.5 {
            trace_note!("Проверяем ожидаемое свойство учебного примера.");
            assert!((uncertainty_measure - 1.0).abs() < 1e-10);
        }
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: доля={positive_class_share}, энтропия={uncertainty_measure:.3}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_class_entropy_as_negative_sum_of_class_shares_times_their_logarithms();
}

// Строим график по результатам урока.
fn plot_class_entropy_as_negative_sum_of_class_shares_times_their_logarithms() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let uncertainty_measure_points: Vec<(f64, f64)> = (1..100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (
                probability,
                -probability * probability.log2()
                    - (1.0 - probability) * (1.0 - probability).log2(),
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
        "Энтропия бинарного класса",
        "доля положительных",
        "энтропия",
        &[lesson_visualization::Series {
            name: "H(p)",

            points: &uncertainty_measure_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
