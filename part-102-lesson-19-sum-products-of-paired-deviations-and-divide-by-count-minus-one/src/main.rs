// Урок 19.2. Сумма произведений парных отклонений от средних, делённая на число наблюдений минус один.
// Связь с принятой терминологией: Выборочная ковариация двух признаков.
// Зачем здесь эта тема: После центрирования нужна мера совместного изменения двух признаков.
// Почему код устроен так: Усредняем произведения отклонений с выборочным знаменателем n−1.
// Представь: Если два признака растут вместе, произведения их отклонений чаще положительны.
//
// Что изучаем: Ковариация.
// Зачем это нужно: Ковариация показывает, меняются ли два признака вместе. Для выборки делим сумму
// результатов умножения отклонений на n−1.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `first_feature_values` для следующего шага примера.
    let first_feature_values: [f64; 3] = [1.0, 2.0, 3.0];
    lesson_trace::trace_step!(first_feature_values);
    // Создаём набор значений `second_feature_values` для следующего шага примера.
    let second_feature_values: [f64; 3] = [2.0, 4.0, 6.0];
    lesson_trace::trace_step!(second_feature_values);
    // Преобразуем входные данные и сохраняем полученную коллекцию в `mean_horizontal_coordinate`.
    // Для каждой пары наблюдений нужны обе координаты; выборочная оценка требует хотя бы две пары.
    assert_eq!(
        first_feature_values.len(),
        second_feature_values.len(),
        "оба ряда должны иметь одинаковую длину"
    );
    // Проверяем ожидаемое свойство учебного примера.
    assert!(
        // Обновляем значение результатом текущего вычисления.
        first_feature_values.len() >= 2,
        // Передаём подпись или текстовое значение для следующего шага.
        "для выборочной ковариации нужны хотя бы две пары"
    );
    // Вычисляем `mean_horizontal_coordinate` по элементам исходной коллекции.
    let mean_horizontal_coordinate: f64 =
        first_feature_values.iter().sum::<f64>() / first_feature_values.len() as f64;
    lesson_trace::trace_step!(mean_horizontal_coordinate);
    // Преобразуем входные данные и сохраняем полученную коллекцию в `mean_vertical_coordinate`.
    let mean_vertical_coordinate: f64 =
        second_feature_values.iter().sum::<f64>() / second_feature_values.len() as f64;
    lesson_trace::trace_step!(mean_vertical_coordinate);
    // Инициализируем изменяемый накопитель `sum` начальным состоянием.
    let mut sum: f64 = 0.0;
    lesson_trace::trace_step!(sum);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for index in 0..first_feature_values.len() {
        lesson_trace::trace_step!(index);
        // Прибавляем очередной вклад к ранее накопленному результату.
        sum += (first_feature_values[index] - mean_horizontal_coordinate)
            * (second_feature_values[index] - mean_vertical_coordinate);
        lesson_trace::trace_step!(sum);
    }
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        "выборочная ковариация={}",
        sum / (first_feature_values.len() - 1) as f64
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_paired_feature_values_to_show_joint_variation(first_feature_values, second_feature_values);
}

// Строим график по результатам урока.
fn plot_paired_feature_values_to_show_joint_variation(
    horizontal_value: [f64; 3],
    vertical_value: [f64; 3],
) {
    // Значения из этого урока на графике.
    // Совместное изменение двух величин описывают через covariance.
    let joint_variation_points: Vec<(f64, f64)> = horizontal_value
        .iter()
        .zip(vertical_value.iter())
        .map(|(&first_feature_value, &second_feature_value)| {
            (first_feature_value, second_feature_value)
        })
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Ковариация: совместное изменение",
        // Указываем подпись горизонтальной оси.
        "x",
        // Указываем подпись вертикальной оси.
        "y",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "наблюдения",
            // Передаём рассчитанные координаты точек.
            points: &joint_variation_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
