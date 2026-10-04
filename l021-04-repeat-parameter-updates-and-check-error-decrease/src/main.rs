// Урок 021. Повторять обновление параметра и останавливаться, когда производная близка к нулю.
// На простой функции приближаем параметр к значению 3, где ошибка минимальна.

use lesson_float_comparison::check_f64_eq_1e_minus_6;

fn main() {
    let mut parameter: f64 = 0.0;
    let mut previous_loss: f64 = (parameter - 3.0).powi(2);
    for epoch in 0..100 {
        let slope = 2.0 * (parameter - 3.0);
        if check_f64_eq_1e_minus_6(slope, 0.0) {
            println!("Остановка на шаге {epoch}: производная={slope:e}");
            break;
        }
        parameter -= 0.2 * slope;
        let loss = (parameter - 3.0).powi(2);
        assert!(loss < previous_loss);
        if epoch < 3 || epoch % 10 == 0 {
            println!("Шаг {epoch}: параметр={parameter:.6}, ошибка={loss:.9}");
        }
        previous_loss = loss;
    }
    assert!(check_f64_eq_1e_minus_6(parameter, 3.0));
}

// Чему учит этот урок:
// Учимся повторять обновление параметра и останавливаться, когда производная близка к нулю.
// На простой функции приближаем параметр к значению 3, где ошибка минимальна.
