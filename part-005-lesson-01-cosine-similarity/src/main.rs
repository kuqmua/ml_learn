// Урок 01.5. Косинусное сходство.
//
// Что изучаем: сравнение направлений независимо от длины векторов.
// 1 означает одинаковое направление, 0 — перпендикулярность, −1 — противоположное.
// Промежуточные значения показывают острый или тупой угол. Для нулевого вектора направления нет.

fn main() {
    let left = [1.0, 0.0];
    let cases: [(&str, &[f64], f64); 5] = [
        ("то же направление", &[2.0, 0.0], 1.0),
        ("острый угол", &[1.0, 1.0], 0.7071067811865475),
        ("перпендикулярные векторы", &[0.0, 2.0], 0.0),
        ("тупой угол", &[-1.0, 1.0], -0.7071067811865475),
        ("противоположные направления", &[-2.0, 0.0], -1.0),
    ];

    for (description, right, expected) in cases {
        assert_eq!(
            left.len(),
            right.len(),
            "векторы должны быть одинаковой длины"
        );
        let mut sum_after_multiplying_coordinates = 0.0;
        let mut left_squared_length = 0.0;
        let mut right_squared_length = 0.0;
        for index in 0..left.len() {
            sum_after_multiplying_coordinates += left[index] * right[index];
            left_squared_length += left[index] * left[index];
            right_squared_length += right[index] * right[index];
        }
        assert!(
            left_squared_length > 0.0 && right_squared_length > 0.0,
            "у нулевого вектора нет направления"
        );
        // Извлекаем корень по методу Ньютона, чтобы получить длины векторов.
        let squared_denominator = left_squared_length * right_squared_length;
        let mut denominator = squared_denominator;
        for _ in 0..80 {
            denominator = (denominator + squared_denominator / denominator) / 2.0;
        }
        let similarity = sum_after_multiplying_coordinates / denominator;
        assert!((similarity - expected).abs() < 1e-10);
        println!("{description}: {left:?} и {right:?} → {similarity:.3}");
    }

    // Показанные ниже входы не имеют косинусного сходства.
    for (description, right) in [
        ("нулевой вектор", &[0.0, 0.0][..]),
        ("разная длина", &[1.0][..]),
    ] {
        if left.len() != right.len() || right.iter().all(|&value| value == 0.0) {
            println!("{description}: вычисление невозможно");
        }
    }
}
