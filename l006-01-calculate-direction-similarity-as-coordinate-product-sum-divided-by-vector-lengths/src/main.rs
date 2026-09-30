// Урок 01.5. Сходство направлений векторов: сумма произведений координат, делённая на произведение длин.
// Связь с принятой терминологией: Косинусное сходство двух векторов.
// Зачем здесь эта тема: Для сравнения направления одной длины недостаточно; нормированное скалярное
//   произведение убирает влияние масштаба.
// Почему код устроен так: Делим произведение на обе длины и отдельно рассматриваем нулевой вектор,
//   для которого деление невозможно.
// Представь: Векторы [1, 0] и [10, 0] имеют разную длину, но одинаковое направление; косинус у них
//   равен 1.
//
// Что изучаем: сравнение направлений независимо от длины векторов.
// 1 означает одинаковое направление, 0 — перпендикулярность, −1 — противоположное.
// Промежуточные значения показывают острый или тупой угол. Для нулевого вектора направления нет.

use l006_01_calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths;

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `left`.");
    let left: [f64; 2] = [1.0, 0.0];
    trace_step!(left);
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], f64); 5] = [
        ("то же направление", &[2.0, 0.0], 1.0),
        ("острый угол", &[1.0, 1.0], 0.7071067811865475),
        ("перпендикулярные векторы", &[0.0, 2.0], 0.0),
        ("тупой угол", &[-1.0, 1.0], -0.7071067811865475),
        ("противоположные направления", &[-2.0, 0.0], -1.0),
    ];
    trace_step!(cases);

    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, right, expected) in cases {
        trace_step!(description);
        trace_step!(right);
        trace_step!(expected);
        trace_note!("Числитель и длины уже изучены; общий код соединяет их в косинусное сходство.");
        trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let similarity: f64 =
            calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(
                &left, right,
            )
            .expect("оба вектора ненулевые и одинаковой длины");
        trace_step!(similarity);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((similarity - expected).abs() < 1e-10);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {left:?} и {right:?} → {similarity:.3}");
    }

    trace_note!("Показанные ниже входы не имеют косинусного сходства.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, right) in [
        ("нулевой вектор", &[0.0, 0.0][..]),
        ("разная длина", &[1.0][..]),
    ] {
        trace_step!(description);
        trace_step!(right);
        trace_note!("Сохраняем результат этого шага в `error`.");
        trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        let error: &str =
            calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(
                &left, right,
            )
            .expect_err("этот вход должен быть отклонён");
        trace_step!(error);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {error}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_direction_similarity_as_coordinate_product_sum_divided_by_lengths_for_changing_angle();
}

// Строим график по результатам урока.
fn plot_direction_similarity_as_coordinate_product_sum_divided_by_lengths_for_changing_angle() {
    trace_note!("Наглядное представление величин из этого урока.");
    trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let cosine_similarity_between_two_vectors_points: Vec<(f64, f64)> = (0..=180)
        .step_by(5)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `angle`.");
            let angle: f64 = (plot_step_index as f64).to_radians();
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Передаём ряды или значения для отрисовки графика.");
            trace_note!("Передаём ряды или значения для отрисовки графика.");
            trace_note!("Используем результат, ожидая успешного выполнения шага.");
            (
                plot_step_index as f64,
                calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(
                    &[1.0, 0.0],
                    &[angle.cos(), angle.sin()],
                )
                .unwrap(),
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
        "Косинусное сходство двух векторов",
        "угол, градусы",
        "сходство",
        &[lesson_visualization::Series {
            name: "вектор [1, 0]",

            points: &cosine_similarity_between_two_vectors_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
