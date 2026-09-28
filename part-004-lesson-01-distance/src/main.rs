// Урок 01.4. Евклидово расстояние между точками.
//
// Что изучаем: разности по каждой координате возводим в квадрат, складываем и извлекаем корень.
// Для совпадающих точек ответ 0. Порядок точек не влияет на расстояние.

fn main() {
    let cases: [(&str, &[f64], &[f64], f64); 3] = [
        ("разные точки", &[0.0, 0.0], &[3.0, 4.0], 5.0),
        ("поменяли точки местами", &[3.0, 4.0], &[0.0, 0.0], 5.0),
        ("точки совпадают", &[3.0, 4.0], &[3.0, 4.0], 0.0),
    ];
    for (description, first_point, second_point, expected) in cases {
        assert_eq!(
            first_point.len(),
            second_point.len(),
            "точки должны иметь одинаковое число координат"
        );
        let mut squared_distance = 0.0;
        for index in 0..first_point.len() {
            let difference = first_point[index] - second_point[index];
            squared_distance += difference * difference;
        }
        let mut distance = squared_distance;
        if squared_distance > 0.0 {
            for _ in 0..80 {
                distance = (distance + squared_distance / distance) / 2.0;
            }
        }
        assert!((distance - expected).abs() < 1e-10);
        println!("{description}: {first_point:?} и {second_point:?} → {distance}");
    }
    let first_point = [0.0, 0.0];
    let too_short = [3.0];
    if first_point.len() != too_short.len() {
        println!("разная размерность: расстояние вычислить нельзя");
    }
}
