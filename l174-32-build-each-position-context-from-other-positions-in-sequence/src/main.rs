// Урок 174. Получать разные контексты для разных позиций из одной последовательности.
// У каждой позиции своя строка весов, поэтому одни и те же входы смешиваются по-разному.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

fn main() {
    let text_units: [f64; 2] = [1.0, 3.0];
    let weights: [[f64; 2]; 2] = [[0.8, 0.2], [0.4, 0.6]];
    let context: [f64; 2] = [
        weights[0][0] * text_units[0] + weights[0][1] * text_units[1],
        weights[1][0] * text_units[0] + weights[1][1] * text_units[1],
    ];

    // Выполняем вычисления из примера.
    let _ = context;

    println!("Входы={text_units:?}; строки весов={weights:?}; контексты={context:?}");
    assert!(check_f64_eq_1e_minus_12(context[0], 1.4_f64));
    assert!(check_f64_eq_1e_minus_12(context[1], 2.2_f64));
    assert_ne!(context[0], context[1]);
}

// Чему учит этот урок:
// Учимся получать разные контексты для разных позиций из одной последовательности.
// У каждой позиции своя строка весов, поэтому одни и те же входы смешиваются по-разному.
