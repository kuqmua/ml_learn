// Урок 003. Считаем длину стрелки от начала координат до заданной точки.
// Каждое число умножаем само на себя, складываем результаты и берём квадратный корень.
// Для [3, 4]: 3×3 + 4×4 = 25, корень из 25 равен 5, потому что 5×5 = 25.
// Смена знака координаты меняет направление, но не длину.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l003_01_calc_vec_len_as_square_root_of_sum_of_squared_coords::calc_vec_len_as_square_root_of_sum_of_squared_coords;

fn main() {
    let cases: [(&str, [f64; 2], f64); 4] = [
        ("обычный вектор", [3.0, 4.0], 5.0),
        ("сменили знаки", [-3.0, -4.0], 5.0),
        ("вдвое длиннее", [6.0, 8.0], 10.0),
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    for (_description, vec, expected) in cases {
        assert!(check_f64_eq_1e_minus_10(
            calc_vec_len_as_square_root_of_sum_of_squared_coords(&vec),
            expected
        ));
    }
}
