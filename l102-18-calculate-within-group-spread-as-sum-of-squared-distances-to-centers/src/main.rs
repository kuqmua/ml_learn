// Урок 18.3. Разброс внутри групп: сумма квадратов расстояний от точек до центров их групп.
// Связь с принятой терминологией: Сумма квадратов расстояний до назначенных центроидов.
// Зачем здесь эта тема: Чтобы улучшать кластеры, нужна величина, уменьшаемая после переназначения и
//   пересчёта центров.
// Почему код устроен так: Суммируем квадраты расстояний до назначенных центроидов без лишнего
//   извлечения корня.
// Представь: Точка на расстоянии 3 от своего центра добавляет к инерции 9.
//
// Что изучаем: Инерция кластеризации.
// Зачем это нужно: Инерция складывает квадраты расстояний точек до назначенных центров; меньшая означает
// более плотные группы.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let points: [[f64; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [5.0, 0.0], [6.0, 0.0]];
    let centers: [[f64; 2]; 2] = [[0.5, 0.0], [5.5, 0.0]];
    let assignments: [usize; 4] = [0, 0, 1, 1];
    assert_eq!(
        points.len(),
        assignments.len(),
        "каждой точке нужен номер центра"
    );
    assert!(
        assignments.iter().all(|&index| index < centers.len()),
        "номер центра выходит за границы списка"
    );
    let mut _total_squared_distance_to_cluster_centers: f64 = 0.0;
    for index in 0..points.len() {
        let delta: f64 = points[index][0] - centers[assignments[index]][0];
        _total_squared_distance_to_cluster_centers += delta * delta;
    }

    plot_squared_distance_to_nearest_fixed_center_for_changing_point();
}

// Строим график по результатам урока.
fn plot_squared_distance_to_nearest_fixed_center_for_changing_point() {
    let squared_distance_sum_points: Vec<(f64, f64)> = (0..=60)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (
                horizontal_value,
                ((horizontal_value - 0.5) * (horizontal_value - 0.5))
                    .min((horizontal_value - 5.5) * (horizontal_value - 5.5)),
            )
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Инерция для двух центров",
        "точка x",
        "квадрат расстояния",
        &[lesson_visualization::Series {
            name: "ближайший из 0.5 и 5.5",

            points: &squared_distance_sum_points,
        }],
    )
    .expect("не удалось сохранить график");
}
