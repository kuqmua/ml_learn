// Урок 104. По очереди назначать точки ближайшим центрам и пересчитывать центры как средние.
// Останавливаемся, когда центры перестают меняться, и считаем итоговую компактность групп.

fn main() {
    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calc_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        value * value
    }

    /// Квадрат расстояния: складываем квадраты разностей соответствующих координат двух точек.
    fn calc_squared_point_dist_by_summing_squared_coord_diffs(
        point1: [f64; 2],

        point2: [f64; 2],
    ) -> f64 {
        calc_square_by_multiplying_number_by_itself(point1[0] - point2[0])
            + calc_square_by_multiplying_number_by_itself(point1[1] - point2[1])
    }

    let dataset: [[f64; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [10.0, 10.0], [10.0, 11.0]];
    let (centers, error) = (|| -> ([[f64; 2]; 2], f64) {
        let data: &[[f64; 2]] = &dataset;

        let mut centers: [[f64; 2]; 2] = [dataset[0], dataset[2]];

        for _ in 0..100 {
            let mut coord_sums: [[f64; 2]; 2] = [[0.0, 0.0]; 2];

            let mut cluster_sizes: [usize; 2] = [0; 2];

            for &point in data {
                let center_index: usize = (0..centers.len())
                    .min_by(|&center1_index, &center2_index| {
                        calc_squared_point_dist_by_summing_squared_coord_diffs(
                            point,
                            centers[center1_index],
                        )
                        .total_cmp(
                            &calc_squared_point_dist_by_summing_squared_coord_diffs(
                                point,
                                centers[center2_index],
                            ),
                        )
                    })
                    .unwrap();

                coord_sums[center_index][0] += point[0];

                coord_sums[center_index][1] += point[1];

                cluster_sizes[center_index] += 1;
            }

            let previous_centers: [[f64; 2]; 2] = centers;

            for center_index in 0..centers.len() {
                if cluster_sizes[center_index] > 0 {
                    centers[center_index] = [
                        coord_sums[center_index][0] / cluster_sizes[center_index] as f64,
                        coord_sums[center_index][1] / cluster_sizes[center_index] as f64,
                    ];
                }
            }

            if previous_centers == centers {
                break;
            }
        }

        let mut sum_of_squared_dists_to_cluster_centers: f64 = 0.0;

        for &point in data {
            let mut nearest_squared_dist: f64 = f64::INFINITY;

            for &center in &centers {
                let candidate_dist: f64 =
                    calc_squared_point_dist_by_summing_squared_coord_diffs(point, center);

                if candidate_dist < nearest_squared_dist {
                    nearest_squared_dist = candidate_dist;
                }
            }

            sum_of_squared_dists_to_cluster_centers += nearest_squared_dist;
        }

        (centers, sum_of_squared_dists_to_cluster_centers)
    })();
    println!("Найденные центры={centers:?}; сумма квадратов расстояний={error}");
    assert_eq!(centers, [[0.0, 0.5], [10.0, 10.5]]);
    assert_eq!(error, 1.0);

    println!("Исходные точки: {:?}", dataset);
}

// Чему учит этот урок:
// Учимся по очереди назначать точки ближайшим центрам и пересчитывать центры как средние.
// Останавливаемся, когда центры перестают меняться, и считаем итоговую компактность групп.
