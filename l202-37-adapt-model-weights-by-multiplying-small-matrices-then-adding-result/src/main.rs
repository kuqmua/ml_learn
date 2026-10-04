// Урок 37.1. Адаптация весов модели: умножение двух небольших матриц и прибавление результата к исходным весам.
// Зачем здесь эта тема: Полная перенастройка большой модели требует хранить и обновлять много
//   весов.
// Почему код устроен так: Замораживаем исходную матрицу и обучаем две маленькие матрицы,
//   произведение которых задаёт поправку.
// Представь: Вместо изменения большой матрицы добавляем произведение двух маленьких матриц к её
//   выходу.
// Замороженную матрицу дополняют произведением маленьких обучаемых матриц.

fn main() {
    let adapter_input_weights: [f64; 4] = [1.0, 0.0, -1.0, 0.0];
    let input: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
    let projected_input: f64 = adapter_input_weights
        .iter()
        .zip(input)
        .map(|(adapter_component, input_component)| adapter_component * input_component)
        .sum();
    let frozen: [[f64; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let adapter_output_weights: [f64; 4] = [0.1, 0.2, 0.3, 0.4];
    assert_eq!(
        std::array::from_fn::<f64, 4, _>(|row| {
            frozen[row]
                .iter()
                .zip(input)
                .map(|(weight_value, input_component)| weight_value * input_component)
                .sum::<f64>()
                + adapter_output_weights[row] * projected_input
        })[0],
        0.8
    );

    let baseline: f64 = frozen[0].iter().zip(input).map(|(w, x)| w * x).sum();
    let correction = adapter_output_weights[0] * projected_input;
    println!(
        "Исходный выход первой строки={baseline}; поправка={correction}; итог={}",
        baseline + correction
    );
    println!(
        "Полная матрица: 16 весов; поправка через два вектора: {} весов",
        adapter_input_weights.len() + adapter_output_weights.len()
    );
    assert_eq!(baseline, 1.0);
    assert_eq!(correction, -0.2);
}

// Чему учит этот урок:
// Учимся добавлять к результату фиксированной матрицы поправку через промежуточное представление
// меньшего размера.
// Два коротких набора весов задают такую поправку; их обучение в текущем примере не выполняется.
