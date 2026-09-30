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
    let data: [[f64; 2]; 4] = [[1., 1.], [2., 2.], [3., 3.], [4., 4.]];

    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calculate_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        value * value
    }

    /// Корень через итерацию Ньютона: x_(n+1) = (x_n + value / x_n) / 2.
    /// Учебный аналог `f64::sqrt`; показывает алгоритм и может работать медленнее.
    /// Здесь отрицательный вход вызывает panic, а `sqrt` возвращает NaN.
    /// Метод Ньютона для корня: повторяем estimate = (estimate + value / estimate) / 2.
    fn approximate_square_root_by_repeated_averaging(value: f64) -> f64 {
        assert!(value >= 0.0, "корень из отрицательного числа");
        if value == 0.0 {
            return 0.0;
        }
        let mut estimate: f64 = if value > 1.0 { value } else { 1.0 };
        for _ in 0..80 {
            estimate = (estimate + value / estimate) / 2.0;
        }
        estimate
    }

    let (mean, axis, _variance_share_explained_by_first_axis): ([f64; 2], [f64; 2], f64) =
        (|| -> ([f64; 2], [f64; 2], f64) {
            let data: &[[f64; 2]] = &data;
            let sample_count: f64 = data.len() as f64;
            let mut coordinate_sums: [f64; 2] = [0.0, 0.0];
            for point in data {
                coordinate_sums[0] += point[0];
                coordinate_sums[1] += point[1];
            }
            let mean: [f64; 2] = [
                coordinate_sums[0] / sample_count,
                coordinate_sums[1] / sample_count,
            ];
            let (
                mut first_variance_sum,
                mut sum_after_multiplying_differences_from_mean,
                mut second_variance_sum,
            ): (f64, f64, f64) = (0.0, 0.0, 0.0);
            for point in data {
                let centered_first: f64 = point[0] - mean[0];
                let centered_second: f64 = point[1] - mean[1];
                first_variance_sum +=
                    calculate_square_by_multiplying_number_by_itself(centered_first);
                sum_after_multiplying_differences_from_mean += centered_first * centered_second;
                second_variance_sum +=
                    calculate_square_by_multiplying_number_by_itself(centered_second);
            }
            let discriminant: f64 = calculate_square_by_multiplying_number_by_itself(
                first_variance_sum - second_variance_sum,
            ) + 4.0
                * calculate_square_by_multiplying_number_by_itself(
                    sum_after_multiplying_differences_from_mean,
                );
            let largest_eigenvalue: f64 = (first_variance_sum
                + second_variance_sum
                + approximate_square_root_by_repeated_averaging(discriminant))
                / 2.0;
            let axis: [f64; 2] = if (|| -> f64 {
                let value: f64 = sum_after_multiplying_differences_from_mean;
                if value < 0.0 { -value } else { value }
            })() < 1e-12
            {
                if first_variance_sum >= second_variance_sum {
                    [1.0, 0.0]
                } else {
                    [0.0, 1.0]
                }
            } else {
                let unnormalized_axis: [f64; 2] = [
                    sum_after_multiplying_differences_from_mean,
                    largest_eigenvalue - first_variance_sum,
                ];
                let axis_length: f64 = approximate_square_root_by_repeated_averaging(
                    calculate_square_by_multiplying_number_by_itself(unnormalized_axis[0])
                        + calculate_square_by_multiplying_number_by_itself(unnormalized_axis[1]),
                );
                [
                    unnormalized_axis[0] / axis_length,
                    unnormalized_axis[1] / axis_length,
                ]
            };
            let variance: f64 = largest_eigenvalue / (first_variance_sum + second_variance_sum);
            (mean, axis, variance)
        })();
    let projections: [f64; 4] =
        data.map(|point| (point[0] - mean[0]) * axis[0] + (point[1] - mean[1]) * axis[1]);

    plot_points_projected_onto_direction_of_largest_spread(projections);
}

// Строим график по результатам урока.
fn plot_points_projected_onto_direction_of_largest_spread(projections: [f64; 4]) {
    let principal_component_analysis_points: Vec<(f64, f64)> = projections
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
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
}
