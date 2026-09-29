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

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `first`.
    let first: [f64; 2] = [3.0, 4.0];
    lesson_trace::trace_step!(first);
    // Проверяем ожидаемое свойство учебного примера.
    assert_eq!(
        l002_01_calculate_l1_vector_norm_by_summing_absolute_coordinates::calculate_l1_vector_norm_by_summing_absolute_coordinates(&first),
        7.0
    );
    // Проверяем ожидаемое свойство учебного примера.
    assert_eq!(
        l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(
            &first
        ),
        5.0
    );
    // Печатаем рассчитанные значения для проверки примера.
    println!("вектор {first:?}: L1=7, L2=5");

    // Задаём учебные значения для `cases`.
    let cases: [(&str, &[f64], f64, Option<f64>); 4] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("тот же вектор", &[3.0, 4.0], 25.0, Some(1.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("перпендикулярный", &[-4.0, 3.0], 0.0, Some(0.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("противоположный", &[-3.0, -4.0], -25.0, Some(-1.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("нулевой без направления", &[0.0, 0.0], 0.0, None),
    ];
    lesson_trace::trace_step!(cases);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, other, expected_sum, expected_cosine) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(other);
        lesson_trace::trace_step!(expected_sum);
        lesson_trace::trace_step!(expected_cosine);
        // Сохраняем результат этого шага в `sum`.
        let sum: f64 = l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&first, other)
            // Используем результат, ожидая успешного выполнения шага.
            .expect("у этих векторов одинаковое число координат");
        lesson_trace::trace_step!(sum);
        // Сохраняем результат этого шага в `distance`.
        let distance: f64 =
            l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(&first, other).unwrap();
        lesson_trace::trace_step!(distance);
        // Сохраняем результат этого шага в `cosine`.
        let cosine: Option<f64> = l006_01_calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&first, other).ok();
        lesson_trace::trace_step!(cosine);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(sum, expected_sum);
        // Выбираем дальнейший шаг по выполнению условия.
        if let (Some(actual), Some(expected)) = (cosine, expected_cosine) {
            // Проверяем ожидаемое свойство учебного примера.
            assert!((actual - expected).abs() < 1e-10);
        // Обрабатываем случай, когда предыдущее условие не выполнено.
        } else {
            // Проверяем ожидаемое свойство учебного примера.
            assert_eq!(cosine, expected_cosine);
        }
        // Сохраняем результат этого шага в `reverse_distance`.
        let reverse_distance: f64 =
            l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(other, &first).unwrap();
        lesson_trace::trace_step!(reverse_distance);
        // Проверяем ожидаемое свойство учебного примера.
        assert!((distance - reverse_distance).abs() < 1e-10);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: сумма={sum}, расстояние={distance:.3}, косинус={cosine:?}");
    }

    // Задаём учебные значения для `too_short`.
    let too_short: [f64; 1] = [1.0];
    lesson_trace::trace_step!(too_short);
    // Сохраняем результат этого шага в `error`.
    let error: &str =
        l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
            &first, &too_short,
        )
        // Настраиваем или преобразуем результат предыдущего шага.
        .expect_err("векторы разной длины нужно отклонить");
    lesson_trace::trace_step!(error);
    // Печатаем рассчитанные значения для проверки примера.
    println!("разная длина: {error}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_l1_and_euclidean_lengths_as_absolute_sum_and_square_root_of_squared_sum();
}

// Строим график по результатам урока.
fn plot_l1_and_euclidean_lengths_as_absolute_sum_and_square_root_of_squared_sum() {
    // Наглядное представление величин из этого урока.
    let sum_absolute_values_of_vector_coordinates_points: Vec<(f64, f64)> = (-50..=50)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (horizontal_value, horizontal_value.abs() + 4.0)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `euclidean_norm_points` в коллекцию.
    let euclidean_norm_points: Vec<(f64, f64)> = (-50..=50)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (
                horizontal_value,
                (horizontal_value * horizontal_value + 16.0).sqrt(),
            )
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Сравнение норм",
        // Указываем подпись горизонтальной оси.
        "первая координата",
        // Указываем подпись вертикальной оси.
        "норма",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "L1",
                // Передаём рассчитанные координаты точек.
                points: &sum_absolute_values_of_vector_coordinates_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "L2",
                // Передаём рассчитанные координаты точек.
                points: &euclidean_norm_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
