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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Шаг: Создаём точки, лежащие на одной прямой.");
    let data: [[f64; 2]; 4] = [[1., 1.], [2., 2.], [3., 3.], [4., 4.]];
    trace_step!(data);

    trace_note!("Учебные реализации математических операций для этого урока.");

    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calculate_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        trace_note!("Умножаем величины согласно используемой формуле.");
        value * value
    }

    /// Корень через итерацию Ньютона: x_(n+1) = (x_n + value / x_n) / 2.
    /// Учебный аналог `f64::sqrt`; показывает алгоритм и может работать медленнее.
    /// Здесь отрицательный вход вызывает panic, а `sqrt` возвращает NaN.
    /// Метод Ньютона для корня: повторяем estimate = (estimate + value / estimate) / 2.
    fn approximate_square_root_by_repeated_averaging(value: f64) -> f64 {
        trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
        assert!(value >= 0.0, "корень из отрицательного числа");
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == 0.0 {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 0.0;
        }
        trace_note!("Создаём изменяемое значение `estimate` для следующих операций.");
        let mut estimate: f64 = if value > 1.0 { value } else { 1.0 };
        trace_step!(estimate);
        trace_note!("80 шагов Ньютона — запас точности для учебного вычисления корня в f64.");
        for _ in 0..80 {
            trace_note!("Из x² = value получаем новую оценку √value как среднее x и value/x.");
            estimate = (estimate + value / estimate) / 2.0;
            trace_step!(estimate);
        }
        trace_note!("Используем ранее рассчитанное значение `estimate` в текущем выражении.");
        estimate
    }

    trace_note!("Шаг: Находим среднее, главную ось и долю объяснённой дисперсии.");
    trace_note!("Долю общей дисперсии, объяснённую осью, называют explained variance fraction.");
    let (mean, axis, variance_share_explained_by_first_axis): ([f64; 2], [f64; 2], f64) =
        (|| -> ([f64; 2], [f64; 2], f64) {
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Находим главную ось двумерной ковариационной матрицы.");
            trace_note!("Сохраняем результат этого шага в `data`.");
            let data: &[[f64; 2]] = &data;
            trace_step!(data);
            trace_note!("Считаем количество элементов и сохраняем его в `sample_count`.");
            let sample_count: f64 = data.len() as f64;
            trace_step!(sample_count);
            trace_note!("Создаём набор значений `coordinate_sums` для следующего шага примера.");
            let mut coordinate_sums: [f64; 2] = [0.0, 0.0];
            trace_step!(coordinate_sums);
            trace_note!(
                "Повторяем следующий блок для каждого элемента указанной последовательности."
            );
            for point in data {
                trace_step!(point);
                trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                coordinate_sums[0] += point[0];
                trace_step!(coordinate_sums);
                trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                coordinate_sums[1] += point[1];
                trace_step!(coordinate_sums);
            }
            trace_note!("Создаём набор значений `mean` для следующего шага примера.");
            trace_note!("Делим значения, получая нормированную величину или среднее.");
            trace_note!("Делим значения, получая нормированную величину или среднее.");
            let mean: [f64; 2] = [
                coordinate_sums[0] / sample_count,
                coordinate_sums[1] / sample_count,
            ];
            trace_step!(mean);
            trace_note!(
                "Сохраняем рассчитанное значение `(mut first_variance_sum, mut cross_covariance_sum, mut second_variance_sum)` для следующих операций."
            );
            trace_note!("Совместное изменение двух величин описывают через covariance.");
            trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
            let (mut first_variance_sum, mut cross_deviation_product_sum, mut second_variance_sum): (f64, f64, f64) =

            (0.0, 0.0, 0.0);
            trace_step!(first_variance_sum);
            trace_step!(cross_deviation_product_sum);
            trace_step!(second_variance_sum);
            trace_note!(
                "Повторяем следующий блок для каждого элемента указанной последовательности."
            );
            for point in data {
                trace_step!(point);
                trace_note!(
                    "Комбинируем исходные величины и сохраняем результат в `centered_first`."
                );
                let centered_first: f64 = point[0] - mean[0];
                trace_step!(centered_first);
                trace_note!(
                    "Комбинируем исходные величины и сохраняем результат в `centered_second`."
                );
                let centered_second: f64 = point[1] - mean[1];
                trace_step!(centered_second);
                trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                first_variance_sum +=
                    calculate_square_by_multiplying_number_by_itself(centered_first);
                trace_step!(first_variance_sum);
                trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                cross_deviation_product_sum += centered_first * centered_second;
                trace_step!(cross_deviation_product_sum);
                trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                second_variance_sum +=
                    calculate_square_by_multiplying_number_by_itself(centered_second);
                trace_step!(second_variance_sum);
            }
            trace_note!("Для симметричной матрицы [[a,b],[b,c]] большее собственное значение");
            trace_note!(
                "равно (a+c+sqrt((a-c)^2+4b^2))/2. Его собственный вектор — [b, lambda-a]."
            );
            trace_note!("Умножаем величины согласно используемой формуле.");
            let discriminant: f64 = calculate_square_by_multiplying_number_by_itself(
                first_variance_sum - second_variance_sum,
            ) + 4.0
                * calculate_square_by_multiplying_number_by_itself(cross_deviation_product_sum);
            trace_step!(discriminant);
            trace_note!(
                "Сохраняем рассчитанное значение `largest_eigenvalue` для следующих операций."
            );
            trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
            trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
            trace_note!("Делим значения, получая нормированную величину или среднее.");
            let largest_eigenvalue: f64 = (first_variance_sum
                + second_variance_sum
                + approximate_square_root_by_repeated_averaging(discriminant))
                / 2.0;
            trace_step!(largest_eigenvalue);
            trace_note!("Комбинируем исходные величины и сохраняем результат в `axis`.");
            trace_note!(
                "10⁻¹² считаем численным нулём ковариации: тогда ось можно выбрать без поворота."
            );
            let axis: [f64; 2] = if (|| -> f64 {
                trace_note!("Используем подготовленное значение в следующем шаге примера.");
                trace_note!(
                    "Модуль числа по определению: меняем знак только у отрицательного числа."
                );
                trace_note!("Сохраняем результат этого шага в `value`.");
                let value: f64 = cross_deviation_product_sum;
                trace_step!(value);
                trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
                trace_note!("Используем подготовленное значение в следующем шаге примера.");
                if value < 0.0 { -value } else { value }
            })() < 1e-12
            {
                trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
                if first_variance_sum >= second_variance_sum {
                    trace_note!(
                        "Составляем результат из вычисленных значений в указанном порядке."
                    );
                    [1.0, 0.0]
                } else {
                    trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                    trace_note!(
                        "Составляем результат из вычисленных значений в указанном порядке."
                    );
                    [0.0, 1.0]
                }
            } else {
                trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                trace_note!(
                    "Создаём набор значений `unnormalized_axis` для следующего шага примера."
                );
                trace_note!(
                    "Используем ранее рассчитанное значение `cross_deviation_product_sum` в текущем выражении."
                );
                trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
                let unnormalized_axis: [f64; 2] = [
                    cross_deviation_product_sum,
                    largest_eigenvalue - first_variance_sum,
                ];
                trace_step!(unnormalized_axis);
                trace_note!(
                    "Сохраняем рассчитанное значение `axis_length` для следующих операций."
                );
                trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
                trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
                let axis_length: f64 = approximate_square_root_by_repeated_averaging(
                    calculate_square_by_multiplying_number_by_itself(unnormalized_axis[0])
                        + calculate_square_by_multiplying_number_by_itself(unnormalized_axis[1]),
                );
                trace_step!(axis_length);
                trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                trace_note!("Делим значения, получая нормированную величину или среднее.");
                trace_note!("Делим значения, получая нормированную величину или среднее.");
                [
                    unnormalized_axis[0] / axis_length,
                    unnormalized_axis[1] / axis_length,
                ]
            };
            trace_step!(axis);
            trace_note!("Нормируем или усредняем величину делением и сохраняем её в `variance`.");
            let variance: f64 = largest_eigenvalue / (first_variance_sum + second_variance_sum);
            trace_step!(variance);
            trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
            (mean, axis, variance)
        })();
    trace_step!(mean);
    trace_step!(axis);
    trace_step!(variance_share_explained_by_first_axis);
    trace_note!("Шаг: Проецируем исходные точки на найденную ось.");
    trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    trace_note!("Преобразуем каждый элемент последовательности.");
    trace_note!("Собираем элементы итератора в итоговую коллекцию.");
    let projections: Vec<f64> = data
        .iter()
        .map(|point| (point[0] - mean[0]) * axis[0] + (point[1] - mean[1]) * axis[1])
        .collect();
    trace_step!(projections);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    trace_note!("Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.");
    println!(
        "mean={mean:?}, axis={axis:?}, explained={variance_share_explained_by_first_axis:.3}, projected={projections:?}"
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_points_projected_onto_direction_of_largest_spread(projections);
}

// Строим график по результатам урока.
fn plot_points_projected_onto_direction_of_largest_spread(projections: std::vec::Vec<f64>) {
    trace_note!("Значения из этого урока на графике.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let principal_component_analysis_points: Vec<(f64, f64)> = projections
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
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
        "PCA: координаты вдоль главной оси",
        "номер точки",
        "проекция",
        &[lesson_visualization::Series {
            name: "проекции",

            points: &principal_component_analysis_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
