// Урок 102. Измерять компактность заданных групп суммой квадратов расстояний до их центров.
// В примере все вторые координаты нулевые, поэтому достаточно разниц первых координат.

fn main() {
    let points: [[f64; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [5.0, 0.0], [6.0, 0.0]];
    let assignments: [usize; 4] = [0, 0, 1, 1];
    assert_eq!(
        points.len(),
        assignments.len(),
        "каждой точке нужен номер центра"
    );
    let centers: [[f64; 2]; 2] = [[0.5, 0.0], [5.5, 0.0]];
    assert!(
        assignments.iter().all(|&index| index < centers.len()),
        "номер центра выходит за границы списка"
    );
    let mut _total_squared_dist_to_cluster_centers: f64 = 0.0;
    for index in 0..points.len() {
        let delta: f64 = points[index][0] - centers[assignments[index]][0];
        _total_squared_dist_to_cluster_centers += delta * delta;
    }

    println!(
        "Сумма квадратов расстояний до назначенных центров={_total_squared_dist_to_cluster_centers}"
    );
    assert_eq!(_total_squared_dist_to_cluster_centers, 1.0);
    let single_center_error = points.iter().map(|p| (p[0] - 3.0).powi(2)).sum::<f64>();
    assert!(single_center_error > _total_squared_dist_to_cluster_centers);
    println!("Если объединить группы в одну: {single_center_error}");
}

// Чему учит этот урок:
// Учимся измерять компактность заданных групп суммой квадратов расстояний до их центров.
// В примере все вторые координаты нулевые, поэтому достаточно разниц первых координат.
