// Урок 111. Передавать влияние результата назад через промежуточные вычисления.
// Для удвоенного квадрата перемножаем производные шагов, получая производную по исходному входу.

use lesson_float_comparison::check_f64_eq_1e_minus_9;

fn main() {
    let x: f64 = 2.0;
    let square = x * x;
    let output = 2.0 * square;
    let output_by_square = 2.0;
    let square_by_x = 2.0 * x;
    let output_by_x = output_by_square * square_by_x;
    let step = 0.001;
    let numerical = (2.0 * (x + step).powi(2) - 2.0 * (x - step).powi(2)) / (2.0 * step);
    println!(
        "Прямо: {x} -> {square} -> {output}; назад: {output_by_square} * {square_by_x} = {output_by_x}"
    );
    println!("Проверка малыми изменениями входа={numerical}");
    assert!(check_f64_eq_1e_minus_9(numerical, output_by_x));
}

// Чему учит этот урок:
// Учимся передавать влияние результата назад через промежуточные вычисления.
// Для удвоенного квадрата перемножаем производные шагов, получая производную по исходному входу.
