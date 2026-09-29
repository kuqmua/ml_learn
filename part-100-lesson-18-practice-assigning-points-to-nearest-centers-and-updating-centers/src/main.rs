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
//   выводим итоговые центры с инерцией.
// Что проверить при изменении примера: Проверь две хорошо разделённые группы; сравни инерцию для нескольких
//   k.
// Дополнительная практика: Реализуй k-means с seed, ограничением итераций и обработкой пустого кластера.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Шаг: Задаём две визуально разделимые группы точек.
    let dataset: [[f64; 2]; 4] = [[0., 0.], [0., 1.], [10., 10.], [10., 11.]];
    lesson_trace::trace_step!(dataset);

    // Учебные реализации математических операций для этого урока.

    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn multiply_number_by_itself(value: f64) -> f64 {
        // Умножаем величины согласно используемой формуле.
        value * value
    }

    // Объявляем повторно используемое вычисление `sum_squared_differences_of_point_coordinates`; параметры ниже задают его входы.
    /// Квадрат расстояния: складываем квадраты разностей соответствующих координат двух точек.
    fn sum_squared_differences_of_point_coordinates(
        // `first_point` задаёт соответствующее входное значение или поле структуры.
        first_point: [f64; 2],
        // `second_point` задаёт соответствующее входное значение или поле структуры.
        second_point: [f64; 2],
        // Указываем тип возвращаемого значения.
    ) -> f64 {
        // Складываем или вычитаем величины согласно используемой формуле.
        multiply_number_by_itself(first_point[0] - second_point[0])
            // Складываем или вычитаем величины согласно используемой формуле.
            + multiply_number_by_itself(first_point[1] - second_point[1])
    }

    // Шаг: Инициализируем центроиды и выводим итоговые центры с инерцией.
    println!(
        // Подставляем результаты в этот шаблон вывода или текстового значения.
        "centers and inertia: {:?}",
        // Составляем результат из вычисленных значений в указанном порядке.
        (|| -> (Vec<[f64; 2]>, f64) {
            // Используем подготовленное значение в следующем шаге примера.
            /* Чередуем назначение ближайшего центра и пересчёт средних по кластерам. */
            // Сохраняем результат этого шага в `data`.
            let data: &[[f64; 2]] = &dataset;
            lesson_trace::trace_step!(data);
            lesson_trace::trace_step!(data);
            // Создаём набор значений `centers` для следующего шага примера.
            let mut centers: Vec<[f64; 2]> = vec![dataset[0], dataset[2]];
            lesson_trace::trace_step!(centers);
            lesson_trace::trace_step!(centers);
            // 100 — верхняя граница перерасчётов центров k-means: цикл также может завершиться раньше.
            for _ in 0..100 {
                // Создаём набор значений `coordinate_sums` для следующего шага примера.
                let mut coordinate_sums: Vec<[f64; 2]> = vec![[0., 0.]; centers.len()];
                lesson_trace::trace_step!(coordinate_sums);
                lesson_trace::trace_step!(coordinate_sums);
                // Создаём набор значений `cluster_sizes` для следующего шага примера.
                let mut cluster_sizes: Vec<i32> = vec![0; centers.len()];
                lesson_trace::trace_step!(cluster_sizes);
                lesson_trace::trace_step!(cluster_sizes);
                // Назначаем каждую точку ближайшему центру и собираем суммы координат.
                for &point in data {
                    lesson_trace::trace_step!(point);
                    // Считаем количество элементов и сохраняем его в `center_index`.
                    let center_index: usize = (0..centers.len())
                        // Сравниваем кандидатов и оставляем наименьшее расстояние.
                        .min_by(|&first_center_index, &second_center_index| {
                            // Вызываем нужное вычисление с подготовленными аргументами.
                            sum_squared_differences_of_point_coordinates(
                                // Используем ранее рассчитанное значение `point` в текущем выражении.
                                point,
                                // Передаём ранее рассчитанное значение в текущую операцию.
                                centers[first_center_index],
                            )
                            // Сравниваем числа с полным порядком, включая специальные значения.
                            .total_cmp(
                                // Передаём данные по ссылке или разыменовываем их для следующей операции.
                                &sum_squared_differences_of_point_coordinates(
                                    // Используем ранее рассчитанное значение `point` в текущем выражении.
                                    point,
                                    // Передаём ранее рассчитанное значение в текущую операцию.
                                    centers[second_center_index],
                                ),
                            )
                        })
                        // Извлекаем значение: выше в примере обеспечено отсутствие ошибки.
                        .unwrap();
                    lesson_trace::trace_step!(center_index);
                    lesson_trace::trace_step!(center_index);
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    coordinate_sums[center_index][0] += point[0];
                    lesson_trace::trace_step!(coordinate_sums);
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    coordinate_sums[center_index][1] += point[1];
                    lesson_trace::trace_step!(coordinate_sums);
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    cluster_sizes[center_index] += 1;
                    lesson_trace::trace_step!(cluster_sizes);
                }
                // Сохраняем рассчитанное значение `previous_centers` для следующих операций.
                let previous_centers: Vec<[f64; 2]> = centers.clone();
                lesson_trace::trace_step!(previous_centers);
                lesson_trace::trace_step!(previous_centers);
                // Новый центр каждого непустого кластера — среднее его точек.
                for center_index in 0..centers.len() {
                    lesson_trace::trace_step!(center_index);
                    // Проверяем условие и выбираем соответствующую ветку алгоритма.
                    if cluster_sizes[center_index] > 0 {
                        // Обновляем `centers[center_index]` результатом текущего шага.
                        centers[center_index] = [
                            // Делим значения, получая нормированную величину или среднее.
                            coordinate_sums[center_index][0] / cluster_sizes[center_index] as f64,
                            // Делим значения, получая нормированную величину или среднее.
                            coordinate_sums[center_index][1] / cluster_sizes[center_index] as f64,
                        ];
                        lesson_trace::trace_step!(centers);
                    }
                }
                // Проверяем условие и выбираем соответствующую ветку алгоритма.
                if previous_centers == centers {
                    // Останавливаем цикл после достижения условия завершения.
                    break;
                }
            }
            // Инерция суммирует квадраты расстояний до ближайших центров.
            let mut inertia: f64 = 0.0;
            lesson_trace::trace_step!(inertia);
            lesson_trace::trace_step!(inertia);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for &point in data {
                lesson_trace::trace_step!(point);
                // Создаём изменяемое значение `nearest_squared_distance` для следующих операций.
                let mut nearest_squared_distance: f64 = f64::INFINITY;
                lesson_trace::trace_step!(nearest_squared_distance);
                lesson_trace::trace_step!(nearest_squared_distance);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for &center in &centers {
                    lesson_trace::trace_step!(center);
                    // Сохраняем рассчитанное значение `candidate_distance` для следующих операций.
                    let candidate_distance: f64 =
                        // Вызываем нужное вычисление с подготовленными аргументами.
                        sum_squared_differences_of_point_coordinates(point, center);
                    lesson_trace::trace_step!(candidate_distance);
                    lesson_trace::trace_step!(candidate_distance);
                    // Проверяем условие и выбираем соответствующую ветку алгоритма.
                    if candidate_distance < nearest_squared_distance {
                        // Обновляем `nearest_squared_distance` результатом текущего шага.
                        nearest_squared_distance = candidate_distance;
                        lesson_trace::trace_step!(nearest_squared_distance);
                    }
                }
                // Прибавляем очередной вклад к ранее накопленному результату.
                inertia += nearest_squared_distance;
                lesson_trace::trace_step!(inertia);
            }
            // Составляем результат из вычисленных значений в указанном порядке.
            (centers, inertia)
        })()
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_training_points_for_grouping_by_nearest_center(dataset);
}

// Строим график по результатам урока.
fn plot_training_points_for_grouping_by_nearest_center(dataset: [[f64; 2]; 4]) {
    // Значения из этого урока на графике.
    // Группировку похожих объектов без готовых меток называют clustering.
    let grouped_observation_points: Vec<(f64, f64)> = dataset
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Две группы точек k-means",
        // Указываем подпись горизонтальной оси.
        "x",
        // Указываем подпись вертикальной оси.
        "y",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "данные",
            // Передаём рассчитанные координаты точек.
            points: &grouped_observation_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
