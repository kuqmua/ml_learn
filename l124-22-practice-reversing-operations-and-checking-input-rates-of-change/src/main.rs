// Урок 124. Считать производные суммы элементов произведения матриц по обеим входным матрицам.
// Меняем каждый входной элемент на малую величину и проверяем все полученные производные численно.

use lesson_float_comparison::check_f64_eq_1e_minus_7;

fn main() {
    fn sum_of_product(left: &[Vec<f64>], right: &[Vec<f64>]) -> f64 {
        assert!(!left.is_empty() && !right.is_empty());
        assert!(left.iter().all(|row| row.len() == right.len()));
        let columns = right[0].len();
        assert!(right.iter().all(|row| row.len() == columns));
        let mut sum = 0.0;
        for row in left {
            for column_index in 0..columns {
                for shared_coordinate_index in 0..right.len() {
                    sum +=
                        row[shared_coordinate_index] * right[shared_coordinate_index][column_index];
                }
            }
        }
        sum
    }
    let left = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let right = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    let h = 1e-5;
    println!(
        "Сумма всех элементов произведения={}",
        sum_of_product(&left, &right)
    );
    for row_index in 0..left.len() {
        for column_index in 0..left[0].len() {
            let derivative = right[column_index].iter().sum::<f64>();
            let mut plus = left.clone();
            let mut minus = left.clone();
            plus[row_index][column_index] += h;
            minus[row_index][column_index] -= h;
            let numerical =
                (sum_of_product(&plus, &right) - sum_of_product(&minus, &right)) / (2.0 * h);
            println!(
                "Левый вход[{row_index}][{column_index}]: производная={derivative}, проверка={numerical}"
            );
            assert!(check_f64_eq_1e_minus_7(derivative, numerical));
        }
    }
    for row_index in 0..right.len() {
        for column_index in 0..right[0].len() {
            let derivative = left.iter().map(|values| values[row_index]).sum::<f64>();
            let mut plus = right.clone();
            let mut minus = right.clone();
            plus[row_index][column_index] += h;
            minus[row_index][column_index] -= h;
            let numerical =
                (sum_of_product(&left, &plus) - sum_of_product(&left, &minus)) / (2.0 * h);
            println!(
                "Правый вход[{row_index}][{column_index}]: производная={derivative}, проверка={numerical}"
            );
            assert!(check_f64_eq_1e_minus_7(derivative, numerical));
        }
    }
}

// Чему учит этот урок:
// Учимся считать производные суммы элементов произведения матриц по обеим входным матрицам.
// Меняем каждый входной элемент на малую величину и проверяем все полученные производные численно.
