// Урок 18.5. Практика: распределение точек по ближайшим центрам и пересчёт центров.
// Связь с принятой терминологией: Кластеризация k-means с центроидами и инерцией.
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
    fn calculate_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        value * value
    }

    /// Квадрат расстояния: складываем квадраты разностей соответствующих координат двух точек.
    fn calculate_squared_point_distance_by_summing_squared_coordinate_differences(
        first_point: [f64; 2],

        second_point: [f64; 2],
    ) -> f64 {
        calculate_square_by_multiplying_number_by_itself(first_point[0] - second_point[0])
            + calculate_square_by_multiplying_number_by_itself(first_point[1] - second_point[1])
    }

    let dataset: [[f64; 2]; 4] = [[0., 0.], [0., 1.], [10., 10.], [10., 11.]];
    let _ = &((|| -> ([[f64; 2]; 2], f64) {
        let data: &[[f64; 2]] = &dataset;

        let mut centers: [[f64; 2]; 2] = [dataset[0], dataset[2]];

        for _ in 0..100 {
            let mut coordinate_sums: [[f64; 2]; 2] = [[0., 0.]; 2];

            let mut cluster_sizes: [usize; 2] = [0; 2];

            for &point in data {
                let center_index: usize = (0..centers.len())

                        .min_by(|&first_center_index, &second_center_index| {

                            calculate_squared_point_distance_by_summing_squared_coordinate_differences(

                                point,

                                centers[first_center_index],
                            )

                            .total_cmp(

                                &calculate_squared_point_distance_by_summing_squared_coordinate_differences(

                                    point,

                                    centers[second_center_index],
                                ),
                            )
                        })

                        .unwrap();

                coordinate_sums[center_index][0] += point[0];

                coordinate_sums[center_index][1] += point[1];

                cluster_sizes[center_index] += 1;
            }

            let previous_centers: [[f64; 2]; 2] = centers;

            for center_index in 0..centers.len() {
                if cluster_sizes[center_index] > 0 {
                    centers[center_index] = [
                        coordinate_sums[center_index][0] / cluster_sizes[center_index] as f64,
                        coordinate_sums[center_index][1] / cluster_sizes[center_index] as f64,
                    ];
                }
            }

            if previous_centers == centers {
                break;
            }
        }

        let mut inertia: f64 = 0.0;

        for &point in data {
            let mut nearest_squared_distance: f64 = f64::INFINITY;

            for &center in &centers {
                let candidate_distance: f64 =
                    calculate_squared_point_distance_by_summing_squared_coordinate_differences(
                        point, center,
                    );

                if candidate_distance < nearest_squared_distance {
                    nearest_squared_distance = candidate_distance;
                }
            }

            inertia += nearest_squared_distance;
        }

        (centers, inertia)
    })());

    plot_training_points_for_grouping_by_nearest_center(dataset);
}

// Строим график по результатам урока.
fn plot_training_points_for_grouping_by_nearest_center(dataset: [[f64; 2]; 4]) {
    lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Две группы точек k-means",
        "x",
        "y",
        &[lesson_visualization::Series {
            name: "данные",

            points: &dataset
                .iter()
                .map(|data_point| (data_point[0], data_point[1]))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
