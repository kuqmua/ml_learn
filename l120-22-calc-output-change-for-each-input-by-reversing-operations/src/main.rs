// Урок 120. Учитывать несколько путей влияния входа на результат x*y + x.
// По x получаем y + 1, по y — x; это простой пример обратного расчёта производных.

use lesson_float_comparison::check_f64_eq_1e_minus_9;

fn main() {
    let f = |x: f64, y: f64| x * y + x;
    let (x, y) = (2.0, 3.0);
    let dx = y + 1.0;
    let dy = x;
    let h = 0.001;
    let numeric_x = (f(x + h, y) - f(x - h, y)) / (2.0 * h);
    let numeric_y = (f(x, y + h) - f(x, y - h)) / (2.0 * h);
    println!(
        "f=x*y+x: выход={}; по x два вклада {y}+1={dx}; по y один вклад={dy}",
        f(x, y)
    );
    println!("Численная проверка: {numeric_x}, {numeric_y}");
    assert!(check_f64_eq_1e_minus_9(numeric_x, dx));
    assert!(check_f64_eq_1e_minus_9(numeric_y, dy));
}

// Чему учит этот урок:
// Учимся учитывать несколько путей влияния входа на результат x*y + x.
// По x получаем y + 1, по y — x; это простой пример обратного расчёта производных.
