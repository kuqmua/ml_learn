// Урок 04.5. Практика: выбор размера шага, способа обновления и проверка уменьшения ошибки.
// Зачем здесь эта тема: Оптимизация требует одновременно выбрать начальную точку, шаг и способ
//   расчёта градиента.
// Почему код устроен так: На одной функции соединяем режимы обновления и наблюдаем всю траекторию,
//   а не только итог.
// Представь: Две траектории могут начать одинаково, но из-за разного шага или режима обновления
//   прийти к разным точкам.
//
// Что повторяем вместе: скорость обучения, сходимость, локальные минимумы, batch и stochastic updates.
// Зачем это нужно: Скорость обучения определяет, приблизится ли последовательность обновлений к минимуму
//   или начнёт расходиться.
// Что показывает программа: Запускаем спуск с тремя скоростями обучения. Сохраняем loss после каждого
//   обновления параметра. По первой и последней ошибке видим сходимость или расходимость.
// Что проверить при изменении примера: Покажи успешный, медленный и расходящийся запуск; критерий остановки
//   не должен зависеть только от числа шагов.
// Дополнительная практика: Минимизируй квадратичную функцию и запиши историю loss для нескольких learning
//   rate.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

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
        let _ = (
            &(history[0]),
            &(history[history.len() - 1]),
            &(history.len() - 1),
        );
    }
}
