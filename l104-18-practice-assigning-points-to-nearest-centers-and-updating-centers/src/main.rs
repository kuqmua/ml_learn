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
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Шаг: Задаём две визуально разделимые группы точек.");
    let dataset: [[f64; 2]; 4] = [[0., 0.], [0., 1.], [10., 10.], [10., 11.]];
    trace_step!(dataset);

    trace_note!("Учебные реализации математических операций для этого урока.");

    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calculate_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        trace_note!("Умножаем величины согласно используемой формуле.");
        value * value
    }

    trace_note!(
        "Объявляем повторно используемое вычисление `calculate_squared_point_distance_by_summing_squared_coordinate_differences`; параметры ниже задают его входы."
    );
    /// Квадрат расстояния: складываем квадраты разностей соответствующих координат двух точек.
    fn calculate_squared_point_distance_by_summing_squared_coordinate_differences(
        first_point: [f64; 2],

        second_point: [f64; 2],
    ) -> f64 {
        trace_note!("`first_point` задаёт соответствующее входное значение или поле структуры.");
        trace_note!("`second_point` задаёт соответствующее входное значение или поле структуры.");
        trace_note!("Указываем тип возвращаемого значения.");
        trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
        trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
        calculate_square_by_multiplying_number_by_itself(first_point[0] - second_point[0])
            + calculate_square_by_multiplying_number_by_itself(first_point[1] - second_point[1])
    }

    trace_note!("Шаг: Инициализируем центроиды и выводим итоговые центры с инерцией.");
    trace_note!("Подставляем результаты в этот шаблон вывода или текстового значения.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Чередуем назначение ближайшего центра и пересчёт средних по кластерам.");
    trace_note!("Сохраняем результат этого шага в `data`.");
    trace_note!("Создаём набор значений `centers` для следующего шага примера.");
    trace_note!(
        "100 — верхняя граница перерасчётов центров k-means: цикл также может завершиться раньше."
    );
    trace_note!("Создаём набор значений `coordinate_sums` для следующего шага примера.");
    trace_note!("Создаём набор значений `cluster_sizes` для следующего шага примера.");
    trace_note!("Назначаем каждую точку ближайшему центру и собираем суммы координат.");
    trace_note!("Считаем количество элементов и сохраняем его в `center_index`.");
    trace_note!("Сравниваем кандидатов и оставляем наименьшее расстояние.");
    trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    trace_note!("Используем ранее рассчитанное значение `point` в текущем выражении.");
    trace_note!("Передаём ранее рассчитанное значение в текущую операцию.");
    trace_note!("Сравниваем числа с полным порядком, включая специальные значения.");
    trace_note!("Передаём данные по ссылке или разыменовываем их для следующей операции.");
    trace_note!("Используем ранее рассчитанное значение `point` в текущем выражении.");
    trace_note!("Передаём ранее рассчитанное значение в текущую операцию.");
    trace_note!("Извлекаем значение: выше в примере обеспечено отсутствие ошибки.");
    trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
    trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
    trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
    trace_note!("Сохраняем рассчитанное значение `previous_centers` для следующих операций.");
    trace_note!("Новый центр каждого непустого кластера — среднее его точек.");
    trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
    trace_note!("Обновляем `centers[center_index]` результатом текущего шага.");
    trace_note!("Делим значения, получая нормированную величину или среднее.");
    trace_note!("Делим значения, получая нормированную величину или среднее.");
    trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
    trace_note!("Останавливаем цикл после достижения условия завершения.");
    trace_note!("Инерция суммирует квадраты расстояний до ближайших центров.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Создаём изменяемое значение `nearest_squared_distance` для следующих операций.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Сохраняем рассчитанное значение `candidate_distance` для следующих операций.");
    trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
    trace_note!("Обновляем `nearest_squared_distance` результатом текущего шага.");
    trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    println!(
        "centers and inertia: {:?}",
        (|| -> (Vec<[f64; 2]>, f64) {
            let data: &[[f64; 2]] = &dataset;
            trace_step!(data);
            trace_step!(data);

            let mut centers: Vec<[f64; 2]> = vec![dataset[0], dataset[2]];
            trace_step!(centers);
            trace_step!(centers);

            for _ in 0..100 {
                let mut coordinate_sums: Vec<[f64; 2]> = vec![[0., 0.]; centers.len()];
                trace_step!(coordinate_sums);
                trace_step!(coordinate_sums);

                let mut cluster_sizes: Vec<i32> = vec![0; centers.len()];
                trace_step!(cluster_sizes);
                trace_step!(cluster_sizes);

                for &point in data {
                    trace_step!(point);

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
                    trace_step!(center_index);
                    trace_step!(center_index);

                    coordinate_sums[center_index][0] += point[0];
                    trace_step!(coordinate_sums);

                    coordinate_sums[center_index][1] += point[1];
                    trace_step!(coordinate_sums);

                    cluster_sizes[center_index] += 1;
                    trace_step!(cluster_sizes);
                }

                let previous_centers: Vec<[f64; 2]> = centers.clone();
                trace_step!(previous_centers);
                trace_step!(previous_centers);

                for center_index in 0..centers.len() {
                    trace_step!(center_index);

                    if cluster_sizes[center_index] > 0 {
                        centers[center_index] = [
                            coordinate_sums[center_index][0] / cluster_sizes[center_index] as f64,
                            coordinate_sums[center_index][1] / cluster_sizes[center_index] as f64,
                        ];
                        trace_step!(centers);
                    }
                }

                if previous_centers == centers {
                    break;
                }
            }

            let mut inertia: f64 = 0.0;
            trace_step!(inertia);
            trace_step!(inertia);

            for &point in data {
                trace_step!(point);

                let mut nearest_squared_distance: f64 = f64::INFINITY;
                trace_step!(nearest_squared_distance);
                trace_step!(nearest_squared_distance);

                for &center in &centers {
                    trace_step!(center);

                    let candidate_distance: f64 =
                        calculate_squared_point_distance_by_summing_squared_coordinate_differences(
                            point, center,
                        );
                    trace_step!(candidate_distance);
                    trace_step!(candidate_distance);

                    if candidate_distance < nearest_squared_distance {
                        nearest_squared_distance = candidate_distance;
                        trace_step!(nearest_squared_distance);
                    }
                }

                inertia += nearest_squared_distance;
                trace_step!(inertia);
            }

            (centers, inertia)
        })()
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_training_points_for_grouping_by_nearest_center(dataset);
}

// Строим график по результатам урока.
fn plot_training_points_for_grouping_by_nearest_center(dataset: [[f64; 2]; 4]) {
    trace_note!("Значения из этого урока на графике.");
    trace_note!("Группировку похожих объектов без готовых меток называют clustering.");
    let grouped_observation_points: Vec<(f64, f64)> = dataset
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Две группы точек k-means",
        "x",
        "y",
        &[lesson_visualization::Series {
            name: "данные",

            points: &grouped_observation_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
