// Урок 02.1. Проверка числа элементов матрицы и границ строки и столбца.
// Зачем здесь эта тема: Матрица хранит значения по строкам и столбцам; неверная форма испортит все
//   последующие операции.
// Почему код устроен так: Сначала проверяем число элементов и индексы, затем допускаем чтение
//   конкретной ячейки.
// Представь: Таблица из двух строк по три числа имеет форму 2×3; строка с двумя числами нарушает
//   форму.
//
// Что изучаем: форма rows×columns требует ровно rows*columns элементов.
// Также индекс строки и столбца должен оставаться внутри этих границ.

fn main() {
    let elements: [i32; 6] = [1, 2, 3, 4, 5, 6];
    for (_description, rows, columns, row, column) in [
        ("допустимая ячейка", 2, 3, 1, 2),
        ("лишний столбец", 2, 4, 1, 2),
        ("строка вне матрицы", 2, 3, 2, 0),
        ("столбец вне матрицы", 2, 3, 1, 3),
    ] {
        if rows * columns != elements.len() {
            let _ = &(elements.len());
            continue;
        }
        if row >= rows || column >= columns {
            continue;
        }
        let _value: i32 = elements[row * columns + column];
    }

    plot_matrix_with_two_rows_and_three_columns();
}

// Строим график по результатам урока.
fn plot_matrix_with_two_rows_and_three_columns() {
    let _chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Форма 2 × 3",
        &[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
    )
    .expect("не удалось сохранить тепловую карту");
}
