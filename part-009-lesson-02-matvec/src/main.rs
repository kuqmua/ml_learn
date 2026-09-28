// Урок 02.3. Умножение матрицы на вектор.
//
// Для каждой строки умножаем её значения на координаты вектора и складываем.
// Нулевой вектор даёт нулевой ответ; число столбцов должно совпадать с длиной вектора.

fn main() {
    let matrix = [[1.0, 2.0], [3.0, 4.0]];
    for (description, vector, expected) in [
        ("обычный вектор", [5.0, 6.0], [17.0, 39.0]),
        ("нулевой вектор", [0.0, 0.0], [0.0, 0.0]),
    ] {
        assert_eq!(
            matrix[0].len(),
            vector.len(),
            "число столбцов должно совпадать с длиной вектора"
        );
        let mut result = [0.0; 2];
        for row in 0..matrix.len() {
            for column in 0..vector.len() {
                result[row] += matrix[row][column] * vector[column];
            }
        }
        assert_eq!(result, expected);
        println!("{description}: {vector:?} → {result:?}");
    }
    let too_short = [5.0];
    if matrix[0].len() != too_short.len() {
        println!("разная длина строки и вектора: вычисление невозможно");
    }
}
