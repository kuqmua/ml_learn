// Урок 19.2. Совместное изменение признаков (ковариация): сумма произведений отклонений, делённая на число наблюдений минус один.
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
    lesson_trace::trace_note!(
        "Создаём набор значений `first_feature_values` для следующего шага примера."
    );
    let first_feature_values: [f64; 3] = [1.0, 2.0, 3.0];
    lesson_trace::trace_step!(first_feature_values);
    lesson_trace::trace_note!(
        "Создаём набор значений `second_feature_values` для следующего шага примера."
    );
    let second_feature_values: [f64; 3] = [2.0, 4.0, 6.0];
    lesson_trace::trace_step!(second_feature_values);
    lesson_trace::trace_note!(
        "Преобразуем входные данные и сохраняем полученную коллекцию в `mean_horizontal_coordinate`."
    );
    lesson_trace::trace_note!(
        "Для каждой пары наблюдений нужны обе координаты; выборочная оценка требует хотя бы две пары."
    );
    assert_eq!(
        first_feature_values.len(),
        second_feature_values.len(),
        "оба ряда должны иметь одинаковую длину"
    );
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        first_feature_values.len() >= 2,
        "для выборочной ковариации нужны хотя бы две пары"
    );
    lesson_trace::trace_note!(
        "Вычисляем `mean_horizontal_coordinate` по элементам исходной коллекции."
    );
    let mean_horizontal_coordinate: f64 =
        first_feature_values.iter().sum::<f64>() / first_feature_values.len() as f64;
    lesson_trace::trace_step!(mean_horizontal_coordinate);
    lesson_trace::trace_note!(
        "Преобразуем входные данные и сохраняем полученную коллекцию в `mean_vertical_coordinate`."
    );
    let mean_vertical_coordinate: f64 =
        second_feature_values.iter().sum::<f64>() / second_feature_values.len() as f64;
    lesson_trace::trace_step!(mean_vertical_coordinate);
    lesson_trace::trace_note!("Инициализируем изменяемый накопитель `sum` начальным состоянием.");
    let mut sum: f64 = 0.0;
    lesson_trace::trace_step!(sum);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for index in 0..first_feature_values.len() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        sum += (first_feature_values[index] - mean_horizontal_coordinate)
            * (second_feature_values[index] - mean_vertical_coordinate);
        lesson_trace::trace_step!(sum);
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!(
        "выборочная ковариация={}",
        sum / (first_feature_values.len() - 1) as f64
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_paired_feature_values_to_show_joint_variation(first_feature_values, second_feature_values);
}

// Строим график по результатам урока.
fn plot_paired_feature_values_to_show_joint_variation(
    horizontal_value: [f64; 3],
    vertical_value: [f64; 3],
) {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    lesson_trace::trace_note!("Совместное изменение двух величин описывают через covariance.");
    let joint_variation_points: Vec<(f64, f64)> = horizontal_value
        .iter()
        .zip(vertical_value.iter())
        .map(|(&first_feature_value, &second_feature_value)| {
            (first_feature_value, second_feature_value)
        })
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ковариация: совместное изменение",
        "x",
        "y",
        &[lesson_visualization::Series {
            name: "наблюдения",

            points: &joint_variation_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
