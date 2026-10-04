// Урок 198. Перемножать преобразование входа с управляющим значением после SiLU.
// Сравниваем нулевое, положительное и сильно отрицательное управление, чтобы увидеть изменение
// выходного сигнала.

use l198_36_calc_gated_layer_output_as_silu_gate_times_up_value::calc_gated_layer_output_as_silu_gate_times_up_value;

fn main() {
    let input: [f64; 2] = [1.0, -2.0];
    let gate: f64 = input[0] - input[1];
    let up_projection: f64 = input[0] + input[1];

    let down: f64 = calc_gated_layer_output_as_silu_gate_times_up_value(gate, up_projection) * 0.5;
    assert!(down.is_finite());

    let open = calc_gated_layer_output_as_silu_gate_times_up_value(3.0, 2.0);
    let closed = calc_gated_layer_output_as_silu_gate_times_up_value(-10.0, 2.0);
    println!(
        "Исходный выход={down}; положительное управление={open}, сильно отрицательное={closed}"
    );
    assert!(open > 5.0);
    assert!(closed.abs() < 0.001);
    assert_eq!(
        calc_gated_layer_output_as_silu_gate_times_up_value(0.0, 2.0),
        0.0
    );
}

// Чему учит этот урок:
// Учимся перемножать преобразование входа с управляющим значением после SiLU.
// Сравниваем нулевое, положительное и сильно отрицательное управление, чтобы увидеть изменение
// выходного сигнала.
