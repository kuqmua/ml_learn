//! Вычисления и примеры урока part-009-lesson-02-matvec.

// Урок 02.3. Умножение матрицы на вектор.
//
// Для каждой строки умножаем её значения на координаты вектора и складываем.
// Нулевой вектор даёт нулевой ответ; число столбцов должно совпадать с длиной вектора.

pub fn run() {
    let matrix = [[1.0, 2.0], [3.0, 4.0]];
    for (description, vector, expected) in [
        ("обычный вектор", [5.0, 6.0], [17.0, 39.0]),
        ("нулевой вектор", [0.0, 0.0], [0.0, 0.0]),
    ] {
        let mut result = [0.0; 2];
        for row in 0..matrix.len() {
            // Урок 01.1 теперь работает и для каждой строки матрицы.
            result[row] = lesson_001::multiply_matching_coordinates_then_add(&matrix[row], &vector)
                .expect("число столбцов совпадает с длиной вектора");
        }
        assert_eq!(result, expected);
        println!("{description}: {vector:?} → {result:?}");
    }
    let too_short = [5.0];
    let error = lesson_001::multiply_matching_coordinates_then_add(&matrix[0], &too_short)
        .expect_err("разные длины нужно отклонить");
    println!("разная длина строки и вектора: {error}");
}
