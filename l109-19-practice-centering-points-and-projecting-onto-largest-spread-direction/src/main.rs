// Урок 19.5. Практика: вычитание средних и проекция точек на направление наибольшего разброса.
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

use lesson_float_comparison::check_f64_eq_1e_minus_12;

fn main() {
    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calc_square_by_multiplying_number_by_itself(value: f64) -> f64 {
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

    let data: [[f64; 2]; 4] = [[1., 1.], [2., 2.], [3., 3.], [4., 4.]];
    let (
        mean,
        unit_direction_of_largest_spread,
        _variance_share_explained_by_first_axis_where_0_means_none_and_1_means_all_spread_preserved,
    ): ([f64; 2], [f64; 2], f64) = (|| -> ([f64; 2], [f64; 2], f64) {
        let data: &[[f64; 2]] = &data;
        let sample_count: f64 = data.len() as f64;
        let mut coord_sums: [f64; 2] = [0.0, 0.0];
        for point in data {
            coord_sums[0] += point[0];
            coord_sums[1] += point[1];
        }
        let mean: [f64; 2] = [coord_sums[0] / sample_count, coord_sums[1] / sample_count];
        let (
            mut first_variance_sum,
            mut sum_after_multiplying_diffs_from_mean,
            mut second_variance_sum,
        ): (f64, f64, f64) = (0.0, 0.0, 0.0);
        for point in data {
            let centered_first: f64 = point[0] - mean[0];
            let centered_second: f64 = point[1] - mean[1];
            first_variance_sum += calc_square_by_multiplying_number_by_itself(centered_first);
            sum_after_multiplying_diffs_from_mean += centered_first * centered_second;
            second_variance_sum += calc_square_by_multiplying_number_by_itself(centered_second);
        }
        let discriminant: f64 =
            calc_square_by_multiplying_number_by_itself(first_variance_sum - second_variance_sum)
                + 4.0
                    * calc_square_by_multiplying_number_by_itself(
                        sum_after_multiplying_diffs_from_mean,
                    );
        let largest_eigenvalue_as_squared_spread_along_principal_axis: f64 = (first_variance_sum
            + second_variance_sum
            + approximate_square_root_by_repeated_averaging(discriminant))
            / 2.0;
        let unit_direction_of_largest_spread: [f64; 2] =
            if check_f64_eq_1e_minus_12(sum_after_multiplying_diffs_from_mean, 0.0) {
                if first_variance_sum >= second_variance_sum {
                    [1.0, 0.0]
                } else {
                    [0.0, 1.0]
                }
            } else {
                let unnormalized_axis: [f64; 2] = [
                    sum_after_multiplying_diffs_from_mean,
                    largest_eigenvalue_as_squared_spread_along_principal_axis - first_variance_sum,
                ];
                let axis_len: f64 = approximate_square_root_by_repeated_averaging(
                    calc_square_by_multiplying_number_by_itself(unnormalized_axis[0])
                        + calc_square_by_multiplying_number_by_itself(unnormalized_axis[1]),
                );
                [
                    unnormalized_axis[0] / axis_len,
                    unnormalized_axis[1] / axis_len,
                ]
            };
        let fraction_of_total_squared_spread_preserved_by_projection: f64 =
            largest_eigenvalue_as_squared_spread_along_principal_axis
                / (first_variance_sum + second_variance_sum);
        (
            mean,
            unit_direction_of_largest_spread,
            fraction_of_total_squared_spread_preserved_by_projection,
        )
    })();

    plot_points_projected_onto_direction_of_largest_spread(data.map(|point| {
        (point[0] - mean[0]) * unit_direction_of_largest_spread[0]
            + (point[1] - mean[1]) * unit_direction_of_largest_spread[1]
    }));
}

// Строим график по результатам урока.
fn plot_points_projected_onto_direction_of_largest_spread(projections: [f64; 4]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "PCA: координаты вдоль главной оси",
        "номер точки",
        "проекция",
        &[lesson_visualization::Series {
            name: "проекции",

            points: &projections
                .iter()
                .enumerate()
                .map(|(item_index, &element_value)| (item_index as f64, element_value))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
