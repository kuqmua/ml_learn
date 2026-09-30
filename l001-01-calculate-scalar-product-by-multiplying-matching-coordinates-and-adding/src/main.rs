// Урок 01.1. Скалярное произведение: умножение соответствующих координат двух векторов и сложение произведений.
// Зачем здесь эта тема: Скалярное произведение связывает координаты с направлением; оно понадобится
//   для длины, матриц и сходства.
// Почему код устроен так: Берём две координаты и разные углы, чтобы знак суммы можно было проверить
//   вручную.
// Представь: Два стрелочных направления могут смотреть в одну сторону, под прямым углом или в
//   противоположные стороны; знак суммы помогает это различить.
//
// Что изучаем: для двух векторов умножаем координаты с одинаковыми индексами и складываем числа.
// Положительный ответ означает угол меньше 90° (включая 0°), отрицательный — больше 90°
// (включая 180°). Ноль означает прямой угол, если оба вектора ненулевые.
// Сам по себе знак не доказывает точного совпадения направлений.
// Нулевой вектор тоже даёт ноль, хотя направления у него нет.
// Что делает пример: показывает все варианты знака, включая граничные случаи и разную длину.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Для всех примеров слева используем один вектор, чтобы было проще сравнивать ответы."
    );
    let left: [f64; 2] = [1.0, 2.0];
    lesson_trace::trace_step!(left);
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], f64); 6] = [
        ("то же направление", &[2.0, 4.0], 10.0),
        ("острый угол", &[2.0, 1.0], 4.0),
        ("перпендикулярные векторы", &[-2.0, 1.0], 0.0),
        ("тупой угол", &[-3.0, 1.0], -1.0),
        ("противоположные направления", &[-1.0, -2.0], -5.0),
        ("нулевой вектор без направления", &[0.0, 0.0], 0.0),
    ];
    lesson_trace::trace_step!(cases);

    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, right, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(right);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!(
            "Реализация находится в библиотеке этого урока; её используют и следующие уроки."
        );
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let sum_after_multiplying_coordinates: f64 =

            l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&left, right)

                .expect("у этой пары одинаковое число координат");
        lesson_trace::trace_step!(sum_after_multiplying_coordinates);
        lesson_trace::trace_note!("Например, для [1, 2] и [-2, 1] получаем 1·(-2) + 2·1 = 0.");
        assert_eq!(sum_after_multiplying_coordinates, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {left:?} и {right:?} → {sum_after_multiplying_coordinates}");
    }

    lesson_trace::trace_note!(
        "Неполную пару отклоняем до вычисления: иначе лишнее значение потеряется."
    );
    let too_short: [f64; 1] = [3.0];
    lesson_trace::trace_step!(too_short);
    lesson_trace::trace_note!("Сохраняем результат этого шага в `error`.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    let error: &str =

        l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(

            &left, &too_short,
        )

        .expect_err("разная длина должна быть отклонена");
    lesson_trace::trace_step!(error);
    lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("разная длина: {:?} и {too_short:?} → {error}", left);
    lesson_trace::trace_note!(
        "Показываем зависимость скалярного произведения от второй координаты."
    );
    lesson_trace::disable();
    plot_scalar_product_as_coordinate_product_sum_for_changing_second_coordinate(&left);
}

// Визуализация вынесена из основного сценария урока.
fn plot_scalar_product_as_coordinate_product_sum_for_changing_second_coordinate(left: &[f64; 2]) {
    lesson_trace::trace_note!(
        "График показывает, как меняется скалярное произведение [1, 2] · [1, x] = 1 + 2x."
    );
    lesson_trace::trace_note!(
        "По горизонтали откладываем вторую координату правого вектора от −4 до 4 с шагом 0,1;"
    );
    lesson_trace::trace_note!(
        "по вертикали — результат умножения координат и сложения. При x = −0,5 результат равен нулю."
    );
    lesson_trace::trace_note!("Превращаем целые числа в значения x с шагом 0,1.");
    let chart_points: Vec<(f64, f64)> = (-40..=40)

        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!("Вычисляем скалярное произведение той же функцией, что использовали выше.");
            let product: f64 = l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
                left,
                &[1.0, horizontal_value],
            )
            .expect("оба вектора имеют две координаты");
            (horizontal_value, product)
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
        "Скалярное произведение [1, 2] · [1, x]",
        "вторая координата правого вектора, x",
        "скалярное произведение",
        &[lesson_visualization::Series {
            name: "вектор [1, 2]",

            points: &chart_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
