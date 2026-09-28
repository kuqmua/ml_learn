//! Вычисления и примеры урока part-001-lesson-01-multiply-coordinates-and-add.

/// Умножаем соответствующие координаты и складываем результаты.
pub fn multiply_matching_coordinates_then_add(
    left: &[f64],
    right: &[f64],
) -> Result<f64, &'static str> {
    if left.len() != right.len() {
        return Err("векторы должны быть одинаковой длины");
    }
    let mut sum = 0.0;
    for index in 0..left.len() {
        sum += left[index] * right[index];
    }
    Ok(sum)
}

// Урок 01.1. Умножение координат попарно и сложение результатов.
//
// Что изучаем: для двух векторов умножаем координаты с одинаковыми индексами и складываем числа.
// Положительный ответ означает угол меньше 90° (включая 0°), отрицательный — больше 90°
// (включая 180°). Ноль означает прямой угол, если оба вектора ненулевые.
// Сам по себе знак не доказывает точного совпадения направлений.
// Нулевой вектор тоже даёт ноль, хотя направления у него нет.
// Что делает пример: показывает все варианты знака, включая граничные случаи и разную длину.

pub fn run() {
    // Для всех примеров слева используем один вектор, чтобы было проще сравнивать ответы.
    let left = [1.0, 2.0];
    let cases: [(&str, &[f64], f64); 6] = [
        ("то же направление", &[2.0, 4.0], 10.0),
        ("острый угол", &[2.0, 1.0], 4.0),
        ("перпендикулярные векторы", &[-2.0, 1.0], 0.0),
        ("тупой угол", &[-3.0, 1.0], -1.0),
        ("противоположные направления", &[-1.0, -2.0], -5.0),
        ("нулевой вектор без направления", &[0.0, 0.0], 0.0),
    ];

    for (description, right, expected) in cases {
        // Реализация находится в библиотеке этого урока; её используют и следующие уроки.
        let sum_after_multiplying_coordinates =
            crate::multiply_matching_coordinates_then_add(&left, right)
                .expect("у этой пары одинаковое число координат");
        // Например, для [1, 2] и [-2, 1] получаем 1·(-2) + 2·1 = 0.
        assert_eq!(sum_after_multiplying_coordinates, expected);
        println!("{description}: {left:?} и {right:?} → {sum_after_multiplying_coordinates}");
    }

    // Неполную пару отклоняем до вычисления: иначе лишнее значение потеряется.
    let too_short = [3.0];
    let error = crate::multiply_matching_coordinates_then_add(&left, &too_short)
        .expect_err("разная длина должна быть отклонена");
    println!("разная длина: {:?} и {too_short:?} → {error}", left);
}

#[cfg(test)]
mod tests {
    #[test]
    fn handles_perpendicular_and_mismatched_vectors() {
        assert_eq!(
            super::multiply_matching_coordinates_then_add(&[1.0, 2.0], &[-2.0, 1.0]),
            Ok(0.0)
        );
        assert!(super::multiply_matching_coordinates_then_add(&[1.0], &[1.0, 2.0]).is_err());
    }
}
