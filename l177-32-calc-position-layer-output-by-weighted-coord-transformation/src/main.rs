// Урок 177. Преобразовывать координаты одной позиции суммами с весами и применять ReLU.
// Так меняется набор признаков внутри позиции без смешивания с другими позициями.

fn main() {
    let text_unit: [f64; 2] = [1.0, 2.0];
    let linear: [f64; 2] = [
        0.5 * text_unit[0] - 0.2 * text_unit[1],
        0.3 * text_unit[0] + 0.4 * text_unit[1],
    ];
    let relu_outputs: [f64; 2] = [
        if linear[0] > 0.0 { linear[0] } else { 0.0 },
        if linear[1] > 0.0 { linear[1] } else { 0.0 },
    ];

    // Выполняем вычисления из примера.
    let _ = relu_outputs;

    println!("Вход={text_unit:?}; линейное преобразование={linear:?}; после ReLU={relu_outputs:?}");
    let negative_input = [-1.0_f64, -2.0];
    let transformed = [
        0.5 * negative_input[0] - 0.2 * negative_input[1],
        0.3 * negative_input[0] + 0.4 * negative_input[1],
    ];
    let rectified = transformed.map(|x| x.max(0.0));
    println!("Другой вход={negative_input:?}; до ReLU={transformed:?}, после={rectified:?}");
    assert_eq!(rectified, [0.0, 0.0]);
}

// Чему учит этот урок:
// Учимся преобразовывать координаты одной позиции суммами с весами и применять ReLU.
// Так меняется набор признаков внутри позиции без смешивания с другими позициями.
