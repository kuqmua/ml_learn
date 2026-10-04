// Урок 014. Узнаём, насколько меняется ответ при очень маленьком изменении входа.
// Для формулы x*x скорость изменения в точке x равна 2*x.
// При x=3 это 6: если увеличить вход на 0.001, ответ вырастет примерно на 0.006.
// Это местная оценка для маленького изменения, а не обещание для любого шага.
// Такую скорость изменения называют производной. Она пригодится для уменьшения ошибки.

fn main() {
    // Сначала смотрим на саму функцию, а не подставляем число в готовую производную.
    let x: f64 = 3.0;
    let value: f64 = x * x;
    let derivative: f64 = 2.0 * x;
    println!("Функция x*x: при x={x} результат={value}");
    for step in [1.0, 0.1, 0.001] {
        let next_x: f64 = x + step;
        let next_value: f64 = next_x * next_x;
        let actual_change: f64 = next_value - value;
        let change_per_input_unit: f64 = actual_change / step;
        let predicted_change: f64 = derivative * step;
        // (x+h)*(x+h) - x*x = 2*x*h + h*h.
        // После деления на h остаётся 2*x+h: при уменьшении h приближаемся к 2*x.
        println!("Шаг={step}: x={next_x}, x*x={next_value:.6}, изменение={actual_change:.6}");
        println!("Изменение / шаг={change_per_input_unit:.6}; производная={derivative}");
        println!(
            "Ожидали изменение около {predicted_change:.6}, отличие={:.6}",
            actual_change - predicted_change
        );
        assert!((change_per_input_unit - (derivative + step)).abs() < 1e-9);
        assert!((actual_change - predicted_change - step * step).abs() < 1e-9);
    }
    // Знак производной указывает сторону уменьшения функции.
    for x in [-3.0_f64, 3.0] {
        let derivative = 2.0 * x;
        let updated_x = x - 0.1 * derivative;
        println!(
            "Уменьшаем x*x: x={x} -> {updated_x}, результат={} -> {}",
            x * x,
            updated_x * updated_x
        );
        assert!(updated_x * updated_x < x * x);
    }
}

// Чему учит этот урок:
// Учимся понимать производную через изменение входа и результата, а не только через готовую
// формулу.
// Сравниваем точное изменение x*x с оценкой 2*x*шаг при разных шагах и проверяем направление
// уменьшения функции.
