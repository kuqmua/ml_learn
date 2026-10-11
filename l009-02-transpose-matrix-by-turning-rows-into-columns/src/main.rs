// Урок 009. Меняем местами первое и второе измерения массива, сохраняя числа.
// Массив [[1, 2, 3], [4, 5, 6]] превращается в [[1, 4], [2, 5], [3, 6]].
// Размеры [2,3] превращаются в [3,2]. Число с индексами [i][j]
// попадает на позицию [j][i]. Повторная операция возвращает исходный массив.
// Это позволяет собирать элементы с одинаковым вторым индексом в отдельный вложенный массив.
// В учебниках такую перестановку называют транспонированием.

fn main() {
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    let mut transposed: [[i32; 2]; 3] = [[0; 2]; 3];
    for first_dimension_index in 0..2 {
        for second_dimension_index in 0..3 {
            transposed[second_dimension_index][first_dimension_index] =
                matrix[first_dimension_index][second_dimension_index];
        }
    }

    let mut restored: [[i32; 3]; 2] = [[0; 3]; 2];
    for first_dimension_index in 0..transposed.len() {
        for second_dimension_index in 0..transposed[first_dimension_index].len() {
            restored[second_dimension_index][first_dimension_index] =
                transposed[first_dimension_index][second_dimension_index];
        }
    }
    assert_eq!(restored, matrix);

    // Выполняем вычисления из примера.
    let _ = transposed;
}

// Чему учит этот урок:
// Учимся менять местами измерения массива, переставляя индексы элементов.
// Так меняется порядок обхода измерений; повторная перестановка восстанавливает
// исходную матрицу.
