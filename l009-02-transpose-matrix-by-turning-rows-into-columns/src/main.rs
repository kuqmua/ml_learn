// Урок 02.2. Транспонирование матрицы: перестановка строк в столбцы.
// Зачем здесь эта тема: Транспонирование меняет роль строк и столбцов; это подготовка к матричному
//   произведению.
// Почему код устроен так: На малой матрице явно переставляем индексы, чтобы проверить новую форму и
//   адрес каждого элемента.
// Представь: Число в строке 1 и столбце 2 после поворота таблицы окажется в строке 2 и столбце 1.
//
// Что изучаем: Транспонирование матрицы.
// Зачем это нужно: Строки исходной матрицы становятся столбцами результата. Значение не меняется, меняются
// только его координаты.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    let mut transposed: [[i32; 2]; 3] = [[0; 2]; 3];
    for row in 0..2 {
        for column in 0..3 {
            transposed[column][row] = matrix[row][column];
        }
    }

    let mut restored: [[i32; 3]; 2] = [[0; 3]; 2];
    for row in 0..transposed.len() {
        for column in 0..transposed[row].len() {
            restored[column][row] = transposed[row][column];
        }
    }
    assert_eq!(restored, matrix);

    plot_matrix_after_turning_rows_into_columns(transposed);
}

// Строим график по результатам урока.
fn plot_matrix_after_turning_rows_into_columns(transposed: [[i32; 2]; 3]) {
    let _chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Транспонированная матрица",
        &transposed
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&element_value| element_value as f64)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
}
