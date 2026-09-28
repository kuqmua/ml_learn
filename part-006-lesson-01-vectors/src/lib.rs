//! Вычисления и примеры урока part-006-lesson-01-vectors.

// Сводная практика 01. Векторы и геометрия.
//
// Здесь соединяем вычисления из уроков 01.1–01.5. Их реализации находятся в общей
// библиотеках предыдущих уроков: позже те же функции применяются в матрицах, kNN и поиске.

pub fn run() {
    let first = [3.0, 4.0];
    assert_eq!(lesson_002::l1_norm(&first), 7.0);
    assert_eq!(lesson_003::l2_norm(&first), 5.0);
    println!("вектор {first:?}: L1=7, L2=5");

    let cases: [(&str, &[f64], f64, Option<f64>); 4] = [
        ("тот же вектор", &[3.0, 4.0], 25.0, Some(1.0)),
        ("перпендикулярный", &[-4.0, 3.0], 0.0, Some(0.0)),
        ("противоположный", &[-3.0, -4.0], -25.0, Some(-1.0)),
        ("нулевой без направления", &[0.0, 0.0], 0.0, None),
    ];
    for (description, other, expected_sum, expected_cosine) in cases {
        let sum = lesson_001::multiply_matching_coordinates_then_add(&first, other)
            .expect("у этих векторов одинаковое число координат");
        let distance = lesson_004::distance(&first, other).unwrap();
        let cosine = lesson_005::cosine_similarity(&first, other).ok();
        assert_eq!(sum, expected_sum);
        if let (Some(actual), Some(expected)) = (cosine, expected_cosine) {
            assert!((actual - expected).abs() < 1e-10);
        } else {
            assert_eq!(cosine, expected_cosine);
        }
        let reverse_distance = lesson_004::distance(other, &first).unwrap();
        assert!((distance - reverse_distance).abs() < 1e-10);
        println!("{description}: сумма={sum}, расстояние={distance:.3}, косинус={cosine:?}");
    }

    let too_short = [1.0];
    let error = lesson_001::multiply_matching_coordinates_then_add(&first, &too_short)
        .expect_err("векторы разной длины нужно отклонить");
    println!("разная длина: {error}");
}
