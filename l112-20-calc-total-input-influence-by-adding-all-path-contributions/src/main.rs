// Урок 112. Складывать вклады всех путей, по которым один вход влияет на результат.
// На умножении x на самого себя учитываем оба появления x, поэтому производная равна 2*x.

use lesson_float_comparison::check_f64_eq_1e_minus_9;

fn main() {
    let x: f64 = 3.0;
    let left_contribution = x;
    let right_contribution = x;
    let derivative = left_contribution + right_contribution;
    let h = 0.001;
    let numerical = ((x + h) * (x + h) - (x - h) * (x - h)) / (2.0 * h);
    println!(
        "В x*x вход используется дважды: вклад слева={left_contribution}, справа={right_contribution}, всего={derivative}"
    );
    assert!(check_f64_eq_1e_minus_9(derivative, numerical));
    assert!((left_contribution - numerical).abs() > 2.0);
    println!("Если учесть только один путь, половина влияния потеряется.");
}

// Чему учит этот урок:
// Учимся складывать вклады всех путей, по которым один вход влияет на результат.
// На умножении x на самого себя учитываем оба появления x, поэтому производная равна 2*x.
