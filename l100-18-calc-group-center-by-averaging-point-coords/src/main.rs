// Урок 100. Находить центр группы точек, усредняя каждую координату отдельно.
// Так можно представить несколько близких объектов одной центральной точкой.

fn main() {
    let cluster: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    assert!(!cluster.is_empty(), "для центра нужна хотя бы одна точка");
    let mut cluster_center: [f64; 2] = [0.0, 0.0];
    for point in cluster {
        cluster_center[0] += point[0];
        cluster_center[1] += point[1];
    }
    cluster_center[0] /= cluster.len() as f64;
    cluster_center[1] /= cluster.len() as f64;

    // Выполняем вычисления из примера.
    let _ = (&cluster, &cluster_center);

    println!("Точки={cluster:?}; центр={cluster_center:?}");
    assert_eq!(cluster_center, [2.0, 3.0]);
    let sum_squared = |center: [f64; 2]| {
        cluster
            .iter()
            .map(|p| (p[0] - center[0]).powi(2) + (p[1] - center[1]).powi(2))
            .sum::<f64>()
    };
    assert!(sum_squared(cluster_center) < sum_squared(cluster[0]));
    println!(
        "Сумма квадратов расстояний до среднего={}, до первой точки={}",
        sum_squared(cluster_center),
        sum_squared(cluster[0])
    );
}

// Чему учит этот урок:
// Учимся находить центр группы точек, усредняя каждую координату отдельно.
// Так можно представить несколько близких объектов одной центральной точкой.
