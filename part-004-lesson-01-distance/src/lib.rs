//! Вычисления и примеры урока part-004-lesson-01-distance.

/// Сумма квадратов покоординатных разностей.
pub fn squared_distance(left: &[f64], right: &[f64]) -> Result<f64, &'static str> {
    if left.len() != right.len() {
        return Err("точки должны иметь одинаковое число координат");
    }
    let mut squared_sum = 0.0;
    for index in 0..left.len() {
        let difference = left[index] - right[index];
        squared_sum += difference * difference;
    }
    Ok(squared_sum)
}

/// Расстояние — длина вектора разностей; используем норму из урока 01.3.
pub fn distance(left: &[f64], right: &[f64]) -> Result<f64, &'static str> {
    if left.len() != right.len() {
        return Err("точки должны иметь одинаковое число координат");
    }
    let differences: Vec<f64> = left.iter().zip(right).map(|(&a, &b)| a - b).collect();
    Ok(lesson_003::l2_norm(&differences))
}

// Урок 01.4. Евклидово расстояние между точками.
//
// Что изучаем: разности по каждой координате возводим в квадрат, складываем и извлекаем корень.
// Для совпадающих точек ответ 0. Порядок точек не влияет на расстояние.

pub fn run() {
    let cases: [(&str, &[f64], &[f64], f64); 3] = [
        ("разные точки", &[0.0, 0.0], &[3.0, 4.0], 5.0),
        ("поменяли точки местами", &[3.0, 4.0], &[0.0, 0.0], 5.0),
        ("точки совпадают", &[3.0, 4.0], &[3.0, 4.0], 0.0),
    ];
    for (description, first_point, second_point, expected) in cases {
        // Общая функция проверяет размерности и вычисляет расстояние.
        let distance = crate::distance(first_point, second_point)
            .expect("точки в этом примере имеют одинаковую размерность");
        assert!((distance - expected).abs() < 1e-10);
        println!("{description}: {first_point:?} и {second_point:?} → {distance}");
    }
    let first_point = [0.0, 0.0];
    let too_short = [3.0];
    let error = crate::distance(&first_point, &too_short)
        .expect_err("точки разной размерности нужно отклонить");
    println!("разная размерность: {error}");
}
