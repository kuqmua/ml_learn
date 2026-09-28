//! Вычисления и примеры урока part-005-lesson-01-cosine-similarity.

/// Сходство направлений использует вычисление 01.1 и длину 01.3.
pub fn cosine_similarity(left: &[f64], right: &[f64]) -> Result<f64, &'static str> {
    let numerator = lesson_001::multiply_matching_coordinates_then_add(left, right)?;
    let denominator = lesson_003::l2_norm(left) * lesson_003::l2_norm(right);
    if denominator == 0.0 {
        return Err("у нулевого вектора нет направления");
    }
    Ok(numerator / denominator)
}

// Урок 01.5. Косинусное сходство.
//
// Что изучаем: сравнение направлений независимо от длины векторов.
// 1 означает одинаковое направление, 0 — перпендикулярность, −1 — противоположное.
// Промежуточные значения показывают острый или тупой угол. Для нулевого вектора направления нет.

pub fn run() {
    let left = [1.0, 0.0];
    let cases: [(&str, &[f64], f64); 5] = [
        ("то же направление", &[2.0, 0.0], 1.0),
        ("острый угол", &[1.0, 1.0], 0.7071067811865475),
        ("перпендикулярные векторы", &[0.0, 2.0], 0.0),
        ("тупой угол", &[-1.0, 1.0], -0.7071067811865475),
        ("противоположные направления", &[-2.0, 0.0], -1.0),
    ];

    for (description, right, expected) in cases {
        // Числитель и длины уже изучены; общий код соединяет их в косинусное сходство.
        let similarity = crate::cosine_similarity(&left, right)
            .expect("оба вектора ненулевые и одинаковой длины");
        assert!((similarity - expected).abs() < 1e-10);
        println!("{description}: {left:?} и {right:?} → {similarity:.3}");
    }

    // Показанные ниже входы не имеют косинусного сходства.
    for (description, right) in [
        ("нулевой вектор", &[0.0, 0.0][..]),
        ("разная длина", &[1.0][..]),
    ] {
        let error =
            crate::cosine_similarity(&left, right).expect_err("этот вход должен быть отклонён");
        println!("{description}: {error}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reuses_earlier_lessons_and_rejects_zero_vector() {
        assert_eq!(super::cosine_similarity(&[1.0, 0.0], &[0.0, 1.0]), Ok(0.0));
        assert_eq!(
            super::cosine_similarity(&[1.0, 0.0], &[-1.0, 0.0]),
            Ok(-1.0)
        );
        assert!(super::cosine_similarity(&[1.0, 0.0], &[0.0, 0.0]).is_err());
    }
}
