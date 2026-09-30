// Урок 36.5. Выход управляемого слоя: умножение двух ветвей с плавным управлением вкладом.
// Связь с принятой терминологией: Применение SwiGLU к gate и up проекциям полносвязного слоя.
// Зачем здесь эта тема: Полносвязный блок может управлять потоком признаков через отдельные ветви.
// Почему код устроен так: Считаем gate и up отдельно, затем перемножаем после нелинейности SwiGLU.
// Представь: Если gate подавлен, даже большой сигнал ветки up почти не проходит в выход.
// Одна проекция открывает gate, другая несёт значения; затем идёт выходная проекция.

use l198_36_calculate_gated_layer_output_by_multiplying_branches_with_smooth_gate::calculate_gated_layer_output_as_gate_times_up_value_over_one_plus_e_to_negative_gate;

use lesson_trace::{enable_tracing, trace_step};

fn main() {
    enable_tracing();
    let input: [f64; 2] = [1.0, -2.0];
    trace_step!(input);
    let gate: f64 = input[0] - input[1];
    trace_step!(gate);
    let up_projection: f64 = input[0] + input[1];
    trace_step!(up_projection);
    let hidden: f64 =
        calculate_gated_layer_output_as_gate_times_up_value_over_one_plus_e_to_negative_gate(
            gate,
            up_projection,
        );
    trace_step!(hidden);
    let down: f64 = hidden * 0.5;
    trace_step!(down);
    assert!(down.is_finite());
    println!("gate={gate}, up={up_projection}, hidden={hidden:.4}, down={down:.4}");
}
