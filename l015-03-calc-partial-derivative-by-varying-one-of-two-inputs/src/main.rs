// Урок 015. Когда ответ зависит от двух чисел, рассматриваем влияние каждого отдельно.
// Для x*x + 3*y*y при изменении x удерживаем y на месте: скорость изменения равна 2*x.
// При изменении y удерживаем x на месте: скорость изменения равна 6*y.
// Так можно узнать, какой вход и насколько влияет на ответ рядом с выбранной точкой.

fn main() {
    let x: f64 = 2.0;
    let y: f64 = -1.0;
    let value = x * x + 3.0 * y * y;
    let step = 0.001;
    let x_derivative = 2.0 * x;
    let y_derivative = 6.0 * y;
    let change_when_only_x_moves = (x + step) * (x + step) + 3.0 * y * y - value;
    let change_when_only_y_moves = x * x + 3.0 * (y + step) * (y + step) - value;
    println!("f(x,y)=x*x+3*y*y; x={x}, y={y}, результат={value}");
    println!(
        "Меняем только x на {step}: изменение={change_when_only_x_moves:.6}, оценка={:.6}",
        x_derivative * step
    );
    println!(
        "Меняем только y на {step}: изменение={change_when_only_y_moves:.6}, оценка={:.6}",
        y_derivative * step
    );
    assert!(change_when_only_x_moves > 0.0);
    assert!(change_when_only_y_moves < 0.0);
    assert!((change_when_only_x_moves - x_derivative * step).abs() < 2e-6);
    assert!((change_when_only_y_moves - y_derivative * step).abs() < 4e-6);
}

// Чему учит этот урок:
// Учимся менять один вход при неизменном втором и сравнивать реальное изменение с оценкой по
// частной производной.
// На числах видим, почему увеличение одного входа повышает ответ, а другого — понижает.
