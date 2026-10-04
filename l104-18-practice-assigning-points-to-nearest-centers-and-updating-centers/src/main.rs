// Урок 18.5. Практика: распределение точек по ближайшим центрам и пересчёт центров.
// Зачем здесь эта тема: k-means чередует назначение точек и пересчёт центроидов до остановки.
// Почему код устроен так: Показываем оба шага и итоговую инерцию на наборе, который можно проверить
//   вручную.
// Представь: Назначили точки ближайшим центрам → передвинули центры в средние точки групп →
//   повторили.
//
// Что повторяем вместе: центроиды, инициализация, инерция, выбор k.
// Зачем это нужно: K-means попеременно назначает точки ближайшим центрам и пересчитывает центры; инерция
//   измеряет компактность групп.
// Что показывает программа: Задаём две визуально разделимые группы точек. Инициализируем центроиды и
//   вычисляем итоговые центры с инерцией.
// Что проверить при изменении примера: Проверь две хорошо разделённые группы; сравни инерцию для нескольких
//   k.
// Дополнительная практика: Реализуй k-means с seed, ограничением итераций и обработкой пустого кластера.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

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
    let _ = &((|| -> ([[f64; 2]; 2], f64) {
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
    })());

    // Выполняем вычисления из примера.
    let _ = dataset;
}

// Чему учит этот урок:
// Учимся по очереди назначать точки ближайшим центрам и пересчитывать центры как средние.
// Останавливаемся, когда центры перестают меняться, и считаем итоговую компактность групп.
