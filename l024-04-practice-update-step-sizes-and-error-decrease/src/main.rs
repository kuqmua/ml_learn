// Урок 024. Сохранять историю ошибки при повторных обновлениях параметра.
// Разные размеры шага позволяют исследовать медленное приближение, успешное уменьшение ошибки и
// расхождение.

use lesson_float_comparison::check_f64_eq_1e_minus_8;

fn main() {
    fn calc_squared_dist_of_parameter_from_three_as_loss(parameter: f64) -> f64 {
        (|| -> f64 {
            let value: f64 = parameter - 3.0;
            value * value
        })()
    }

    for learning_rate in [0.01, 0.2, 1.1] {
        let history: Vec<f64> = (|| -> Vec<f64> {
            let learning_rate: f64 = learning_rate;
            let mut parameter: f64 = 0.0;
            let mut history: Vec<f64> =
                vec![calc_squared_dist_of_parameter_from_three_as_loss(parameter)];
            let steps: usize = 30;
            for _ in 0..steps {
                let loss_slope: f64 = (|| -> f64 {
                    let parameter: f64 = parameter;
                    2.0 * (parameter - 3.0)
                })();
                if check_f64_eq_1e_minus_8(loss_slope, 0.0) {
                    break;
                }
                parameter -= learning_rate * loss_slope;
                history.push(calc_squared_dist_of_parameter_from_three_as_loss(parameter));
                if !parameter.is_finite() {
                    break;
                }
            }
            history
        })();
        let initial = history[0];
        let final_loss = *history.last().unwrap();
        println!(
            "Размер шага={learning_rate}; ошибка {initial} -> {final_loss}; обновлений={}",
            history.len() - 1
        );
        if learning_rate < 1.0 {
            assert!(final_loss < initial);
        } else {
            assert!(final_loss > initial);
        }
    }
}

// Чему учит этот урок:
// Учимся сохранять историю ошибки при повторных обновлениях параметра.
// Разные размеры шага позволяют исследовать медленное приближение, успешное уменьшение ошибки и
// расхождение.
