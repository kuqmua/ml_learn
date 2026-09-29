// Урок 19.5. Практика: вычитание средних и проекция точек на направление наибольшего разброса.
// Связь с принятой терминологией: PCA с центрированием, ковариацией и объяснённой дисперсией.
// Зачем здесь эта тема: Центрирование, ковариация, главная ось и доля сохранённой дисперсии
//   образуют один конвейер PCA.
// Почему код устроен так: На двумерных точках показываем проекцию и восстановление, чтобы геометрию
//   можно было нарисовать.
// Представь: Точки вдоль одной линии можно приблизительно описать одной координатой вместо двух.
//
// Что повторяем вместе: центрирование, ковариация, собственные направления, объяснённая дисперсия.
// Зачем это нужно: PCA находит направление наибольшей изменчивости данных и показывает, сколько информации
//   сохраняет проекция.
// Что показывает программа: Создаём точки, лежащие на одной прямой. Находим среднее, главную ось и долю
//   объяснённой дисперсии. Проецируем исходные точки на найденную ось.
// Что проверить при изменении примера: Проверь восстановление точек на прямой и долю объяснённой дисперсии.
// Дополнительная практика: Реализуй PCA для 2D через ковариационную матрицу и проекцию на главную ось.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Шаг: Создаём точки, лежащие на одной прямой.
    let data: [[f64; 2]; 4] = [[1., 1.], [2., 2.], [3., 3.], [4., 4.]];
    lesson_trace::trace_step!(data);

    // Учебные реализации математических операций для этого урока.

    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn multiply_number_by_itself(value: f64) -> f64 {
        // Умножаем величины согласно используемой формуле.
        value * value
    }

    /// Корень через итерацию Ньютона: x_(n+1) = (x_n + value / x_n) / 2.
    /// Учебный аналог `f64::sqrt`; показывает алгоритм и может работать медленнее.
    /// Здесь отрицательный вход вызывает panic, а `sqrt` возвращает NaN.
    /// Метод Ньютона для корня: повторяем estimate = (estimate + value / estimate) / 2.
    fn approximate_square_root_by_repeated_averaging(value: f64) -> f64 {
        // Проверяем обязательное условие до дальнейшего вычисления.
        assert!(value >= 0.0, "корень из отрицательного числа");
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value == 0.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return 0.0;
        }
        // Создаём изменяемое значение `estimate` для следующих операций.
        let mut estimate: f64 = if value > 1.0 { value } else { 1.0 };
        lesson_trace::trace_step!(estimate);
        // 80 шагов Ньютона — запас точности для учебного вычисления корня в f64.
        for _ in 0..80 {
            // Из x² = value получаем новую оценку √value как среднее x и value/x.
            estimate = (estimate + value / estimate) / 2.0;
            lesson_trace::trace_step!(estimate);
        }
        // Используем ранее рассчитанное значение `estimate` в текущем выражении.
        estimate
    }

    // Шаг: Находим среднее, главную ось и долю объяснённой дисперсии.
    // Долю общей дисперсии, объяснённую осью, называют explained variance fraction.
    let (mean, axis, variance_share_explained_by_first_axis): ([f64; 2], [f64; 2], f64) =
        (|| -> ([f64; 2], [f64; 2], f64) {
            // Используем подготовленное значение в следующем шаге примера.
            /* Находим главную ось двумерной ковариационной матрицы. */
            // Сохраняем результат этого шага в `data`.
            let data: &[[f64; 2]] = &data;
            lesson_trace::trace_step!(data);
            // Считаем количество элементов и сохраняем его в `sample_count`.
            let sample_count: f64 = data.len() as f64;
            lesson_trace::trace_step!(sample_count);
            // Создаём набор значений `coordinate_sums` для следующего шага примера.
            let mut coordinate_sums: [f64; 2] = [0.0, 0.0];
            lesson_trace::trace_step!(coordinate_sums);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for point in data {
                lesson_trace::trace_step!(point);
                // Прибавляем очередной вклад к ранее накопленному результату.
                coordinate_sums[0] += point[0];
                lesson_trace::trace_step!(coordinate_sums);
                // Прибавляем очередной вклад к ранее накопленному результату.
                coordinate_sums[1] += point[1];
                lesson_trace::trace_step!(coordinate_sums);
            }
            // Создаём набор значений `mean` для следующего шага примера.
            let mean: [f64; 2] = [
                // Делим значения, получая нормированную величину или среднее.
                coordinate_sums[0] / sample_count,
                // Делим значения, получая нормированную величину или среднее.
                coordinate_sums[1] / sample_count,
            ];
            lesson_trace::trace_step!(mean);
            // Сохраняем рассчитанное значение `(mut first_variance_sum, mut cross_covariance_sum, mut second_variance_sum)` для следующих операций.
            // Совместное изменение двух величин описывают через covariance.
            let (mut first_variance_sum, mut cross_deviation_product_sum, mut second_variance_sum): (f64, f64, f64) =
            // Составляем результат из вычисленных значений в указанном порядке.
            (0.0, 0.0, 0.0);
            lesson_trace::trace_step!(first_variance_sum);
            lesson_trace::trace_step!(cross_deviation_product_sum);
            lesson_trace::trace_step!(second_variance_sum);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for point in data {
                lesson_trace::trace_step!(point);
                // Комбинируем исходные величины и сохраняем результат в `centered_first`.
                let centered_first: f64 = point[0] - mean[0];
                lesson_trace::trace_step!(centered_first);
                // Комбинируем исходные величины и сохраняем результат в `centered_second`.
                let centered_second: f64 = point[1] - mean[1];
                lesson_trace::trace_step!(centered_second);
                // Прибавляем очередной вклад к ранее накопленному результату.
                first_variance_sum += multiply_number_by_itself(centered_first);
                lesson_trace::trace_step!(first_variance_sum);
                // Прибавляем очередной вклад к ранее накопленному результату.
                cross_deviation_product_sum += centered_first * centered_second;
                lesson_trace::trace_step!(cross_deviation_product_sum);
                // Прибавляем очередной вклад к ранее накопленному результату.
                second_variance_sum += multiply_number_by_itself(centered_second);
                lesson_trace::trace_step!(second_variance_sum);
            }
            // Для симметричной матрицы [[a,b],[b,c]] большее собственное значение
            // равно (a+c+sqrt((a-c)^2+4b^2))/2. Его собственный вектор — [b, lambda-a].
            let discriminant: f64 = multiply_number_by_itself(first_variance_sum - second_variance_sum)
            // Умножаем величины согласно используемой формуле.
            + 4.0 * multiply_number_by_itself(cross_deviation_product_sum);
            lesson_trace::trace_step!(discriminant);
            // Сохраняем рассчитанное значение `largest_eigenvalue` для следующих операций.
            let largest_eigenvalue: f64 = (first_variance_sum
            // Складываем или вычитаем величины согласно используемой формуле.
            + second_variance_sum
            // Складываем или вычитаем величины согласно используемой формуле.
            + approximate_square_root_by_repeated_averaging(discriminant))
            // Делим значения, получая нормированную величину или среднее.
            / 2.0;
            lesson_trace::trace_step!(largest_eigenvalue);
            // Комбинируем исходные величины и сохраняем результат в `axis`.
            // 10⁻¹² считаем численным нулём ковариации: тогда ось можно выбрать без поворота.
            let axis: [f64; 2] = if (|| -> f64 {
                // Используем подготовленное значение в следующем шаге примера.
                /* Модуль числа по определению: меняем знак только у отрицательного числа. */
                // Сохраняем результат этого шага в `value`.
                let value: f64 = cross_deviation_product_sum;
                lesson_trace::trace_step!(value);
                // Проверяем условие и выбираем соответствующую ветку алгоритма.
                if value < 0.0 { -value } else { value }
                // Используем подготовленное значение в следующем шаге примера.
            })() < 1e-12
            {
                // Проверяем условие и выбираем соответствующую ветку алгоритма.
                if first_variance_sum >= second_variance_sum {
                    // Составляем результат из вычисленных значений в указанном порядке.
                    [1.0, 0.0]
                // Обрабатываем случай, когда предыдущее условие не выполнено.
                } else {
                    // Составляем результат из вычисленных значений в указанном порядке.
                    [0.0, 1.0]
                }
            // Обрабатываем случай, когда предыдущее условие не выполнено.
            } else {
                // Создаём набор значений `unnormalized_axis` для следующего шага примера.
                let unnormalized_axis: [f64; 2] = [
                    // Используем ранее рассчитанное значение `cross_deviation_product_sum` в текущем выражении.
                    cross_deviation_product_sum,
                    // Складываем или вычитаем величины согласно используемой формуле.
                    largest_eigenvalue - first_variance_sum,
                ];
                lesson_trace::trace_step!(unnormalized_axis);
                // Сохраняем рассчитанное значение `axis_length` для следующих операций.
                let axis_length: f64 = approximate_square_root_by_repeated_averaging(
                    // Вызываем нужное вычисление с подготовленными аргументами.
                    multiply_number_by_itself(unnormalized_axis[0])
                    // Складываем или вычитаем величины согласно используемой формуле.
                    + multiply_number_by_itself(unnormalized_axis[1]),
                );
                lesson_trace::trace_step!(axis_length);
                // Составляем результат из вычисленных значений в указанном порядке.
                [
                    // Делим значения, получая нормированную величину или среднее.
                    unnormalized_axis[0] / axis_length,
                    // Делим значения, получая нормированную величину или среднее.
                    unnormalized_axis[1] / axis_length,
                ]
            };
            lesson_trace::trace_step!(axis);
            // Нормируем или усредняем величину делением и сохраняем её в `variance`.
            let variance: f64 = largest_eigenvalue / (first_variance_sum + second_variance_sum);
            lesson_trace::trace_step!(variance);
            // Составляем результат из вычисленных значений в указанном порядке.
            (mean, axis, variance)
        })();
    lesson_trace::trace_step!(mean);
    lesson_trace::trace_step!(axis);
    lesson_trace::trace_step!(variance_share_explained_by_first_axis);
    // Шаг: Проецируем исходные точки на найденную ось.
    let projections: Vec<f64> = data
        // Перебираем элементы по ссылке, не копируя исходную коллекцию.
        .iter()
        // Преобразуем каждый элемент последовательности.
        .map(|point| (point[0] - mean[0]) * axis[0] + (point[1] - mean[1]) * axis[1])
        // Собираем элементы итератора в итоговую коллекцию.
        .collect();
    lesson_trace::trace_step!(projections);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        // Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.
        "mean={mean:?}, axis={axis:?}, explained={variance_share_explained_by_first_axis:.3}, projected={projections:?}"
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_points_projected_onto_direction_of_largest_spread(projections);
}

// Строим график по результатам урока.
fn plot_points_projected_onto_direction_of_largest_spread(projections: std::vec::Vec<f64>) {
    // Значения из этого урока на графике.
    let principal_component_analysis_points: Vec<(f64, f64)> = projections
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "PCA: координаты вдоль главной оси",
        // Указываем подпись горизонтальной оси.
        "номер точки",
        // Указываем подпись вертикальной оси.
        "проекция",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "проекции",
            // Передаём рассчитанные координаты точек.
            points: &principal_component_analysis_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
