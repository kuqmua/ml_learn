// Урок 109. Центрировать двумерные точки, находить направление наибольшего разброса и проецировать их
// на него.
// Считаем долю сохранённого разброса, чтобы оценить результат сокращения числа координат.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

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

    let data: [[f64; 2]; 4] = [[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0]];
    let (mean, unit_direction_of_largest_spread, _variance_share_explained_by_axis1): (
        [f64; 2],
        [f64; 2],
        f64,
    ) = (|| -> ([f64; 2], [f64; 2], f64) {
        let data: &[[f64; 2]] = &data;
        let sample_count: f64 = data.len() as f64;
        let mut coord_sums: [f64; 2] = [0.0, 0.0];
        for point in data {
            coord_sums[0] += point[0];
            coord_sums[1] += point[1];
        }
        let mean: [f64; 2] = [coord_sums[0] / sample_count, coord_sums[1] / sample_count];
        let (mut variance1_sum, mut sum_after_multiplying_diffs_from_mean, mut variance2_sum): (
            f64,
            f64,
            f64,
        ) = (0.0, 0.0, 0.0);
        for point in data {
            let centered1: f64 = point[0] - mean[0];
            let centered2: f64 = point[1] - mean[1];
            variance1_sum += calc_square_by_multiplying_number_by_itself(centered1);
            sum_after_multiplying_diffs_from_mean += centered1 * centered2;
            variance2_sum += calc_square_by_multiplying_number_by_itself(centered2);
        }
        let discriminant: f64 =
            calc_square_by_multiplying_number_by_itself(variance1_sum - variance2_sum)
                + 4.0
                    * calc_square_by_multiplying_number_by_itself(
                        sum_after_multiplying_diffs_from_mean,
                    );
        let largest_eigenvalue_as_squared_spread_along_principal_axis: f64 = (variance1_sum
            + variance2_sum
            + approximate_square_root_by_repeated_averaging(discriminant))
            / 2.0;
        let unit_direction_of_largest_spread: [f64; 2] =
            if check_f64_eq_1e_minus_12(sum_after_multiplying_diffs_from_mean, 0.0) {
                if variance1_sum >= variance2_sum {
                    [1.0, 0.0]
                } else {
                    [0.0, 1.0]
                }
            } else {
                let unnormalized_axis: [f64; 2] = [
                    sum_after_multiplying_diffs_from_mean,
                    largest_eigenvalue_as_squared_spread_along_principal_axis - variance1_sum,
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
                / (variance1_sum + variance2_sum);
        (
            mean,
            unit_direction_of_largest_spread,
            fraction_of_total_squared_spread_preserved_by_projection,
        )
    })();

    // Выполняем вычисления из примера.
    let _ = data.map(|point| {
        (point[0] - mean[0]) * unit_direction_of_largest_spread[0]
            + (point[1] - mean[1]) * unit_direction_of_largest_spread[1]
    });

    let projections = data.map(|p| {
        (p[0] - mean[0]) * unit_direction_of_largest_spread[0]
            + (p[1] - mean[1]) * unit_direction_of_largest_spread[1]
    });
    println!(
        "Среднее={mean:?}; направление={unit_direction_of_largest_spread:?}; новые координаты={projections:?}"
    );
    assert!(check_f64_eq_1e_minus_10(
        _variance_share_explained_by_axis1,
        1.0
    ));
    for (point, projection) in data.iter().zip(projections) {
        let restored = [
            mean[0] + projection * unit_direction_of_largest_spread[0],
            mean[1] + projection * unit_direction_of_largest_spread[1],
        ];
        assert!(check_f64_eq_1e_minus_10(point[0], restored[0]));
        assert!(check_f64_eq_1e_minus_10(point[1], restored[1]));
    }
    println!("Для этих точек одной координаты достаточно: восстановление почти точное.");
}

// Чему учит этот урок:
// Учимся центрировать двумерные точки, находить направление наибольшего разброса и проецировать их
// на него.
// Считаем долю сохранённого разброса, чтобы оценить результат сокращения числа координат.
