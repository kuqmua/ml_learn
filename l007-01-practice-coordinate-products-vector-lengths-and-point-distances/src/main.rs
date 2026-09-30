// Урок 01.6. Практика: произведения координат, длины векторов и расстояния между точками.
// Связь с принятой терминологией: Скалярное произведение, нормы, расстояние и косинус двух векторов.
// Зачем здесь эта тема: Теперь можно различать величину, расстояние и направление одних и тех же
//   векторов.
// Почему код устроен так: Считаем все четыре характеристики на одной паре, чтобы увидеть, на какой
//   вопрос отвечает каждая.
// Представь: Для одной пары векторов ответ «насколько длинные?» отличается от ответа «насколько
//   похожи направления?».
//
// Здесь соединяем вычисления из уроков 01.1–01.5. Их реализации находятся в общей
// библиотеках предыдущих уроков: позже те же функции применяются в матрицах, kNN и поиске.

use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;
use l002_01_calculate_l1_vector_norm_by_summing_absolute_coordinates::calculate_l1_vector_norm_by_summing_absolute_coordinates;
use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates;
use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences;
use l006_01_calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths;

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Задаём учебные значения для `first`.");
    let first: [f64; 2] = [3.0, 4.0];
    trace_step!(first);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert_eq!(
        calculate_l1_vector_norm_by_summing_absolute_coordinates(&first),
        7.0
    );
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert_eq!(
        calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(&first),
        5.0
    );
    trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("вектор {first:?}: L1=7, L2=5");

    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], f64, Option<f64>); 4] = [
        ("тот же вектор", &[3.0, 4.0], 25.0, Some(1.0)),
        ("перпендикулярный", &[-4.0, 3.0], 0.0, Some(0.0)),
        ("противоположный", &[-3.0, -4.0], -25.0, Some(-1.0)),
        ("нулевой без направления", &[0.0, 0.0], 0.0, None),
    ];
    trace_step!(cases);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, other, expected_sum, expected_cosine) in cases {
        trace_step!(description);
        trace_step!(other);
        trace_step!(expected_sum);
        trace_step!(expected_cosine);
        trace_note!("Сохраняем результат этого шага в `sum`.");
        trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let sum: f64 =
            calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&first, other)
                .expect("у этих векторов одинаковое число координат");
        trace_step!(sum);
        trace_note!("Сохраняем результат этого шага в `distance`.");
        let distance: f64 =
            calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
                &first, other,
            )
            .unwrap();
        trace_step!(distance);
        trace_note!("Сохраняем результат этого шага в `cosine`.");
        let cosine: Option<f64> =
            calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(
                &first, other,
            )
            .ok();
        trace_step!(cosine);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(sum, expected_sum);
        trace_note!("Выбираем дальнейший шаг по выполнению условия.");
        if let (Some(actual), Some(expected)) = (cosine, expected_cosine) {
            trace_note!("Проверяем ожидаемое свойство учебного примера.");
            assert!((actual - expected).abs() < 1e-10);
        } else {
            trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
            trace_note!("Проверяем ожидаемое свойство учебного примера.");
            assert_eq!(cosine, expected_cosine);
        }
        trace_note!("Сохраняем результат этого шага в `reverse_distance`.");
        let reverse_distance: f64 =
            calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
                other, &first,
            )
            .unwrap();
        trace_step!(reverse_distance);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((distance - reverse_distance).abs() < 1e-10);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: сумма={sum}, расстояние={distance:.3}, косинус={cosine:?}");
    }

    trace_note!("Задаём учебные значения для `too_short`.");
    let too_short: [f64; 1] = [1.0];
    trace_step!(too_short);
    trace_note!("Сохраняем результат этого шага в `error`.");
    trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    let error: &str = calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
        &first, &too_short,
    )
    .expect_err("векторы разной длины нужно отклонить");
    trace_step!(error);
    trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("разная длина: {error}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_l1_and_euclidean_lengths_as_absolute_sum_and_square_root_of_squared_sum();
}

// Строим график по результатам урока.
fn plot_l1_and_euclidean_lengths_as_absolute_sum_and_square_root_of_squared_sum() {
    trace_note!("Наглядное представление величин из этого урока.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let sum_absolute_values_of_vector_coordinates_points: Vec<(f64, f64)> = (-50..=50)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (horizontal_value, horizontal_value.abs() + 4.0)
        })
        .collect();
    trace_note!("Собираем значения для `euclidean_norm_points` в коллекцию.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let euclidean_norm_points: Vec<(f64, f64)> = (-50..=50)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (
                horizontal_value,
                (horizontal_value * horizontal_value + 16.0).sqrt(),
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
        "Сравнение норм",
        "первая координата",
        "норма",
        &[
            lesson_visualization::Series {
                name: "L1",

                points: &sum_absolute_values_of_vector_coordinates_points,
            },
            lesson_visualization::Series {
                name: "L2",

                points: &euclidean_norm_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
