// Сначала умножь каждую строку матрицы на вектор вручную.
#[test]
#[ignore = "заполни ответы и запусти тест с --ignored"]
fn multiply_matching_coordinates_then_add_for_each_matrix_row() {
    let result: Option<[i32; 2]> = None; // [[1, 2], [3, 4]] · [5, 6]
    let result: [i32; 2] = result.expect("впиши оба числа в Some([..., ...])");
    assert_eq!(result, [17, 39]);

    // Почему умножение матрицы 2×2 на вектор из трёх чисел недопустимо?
    let compatible_with_three_values: Option<bool> = None;
    assert_eq!(compatible_with_three_values, Some(false));
}
