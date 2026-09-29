// Урок 36.5. Умножение двух ветвей с плавным управлением вкладом одной из них.
// Связь с принятой терминологией: Применение SwiGLU к gate и up проекциям полносвязного слоя.
// Зачем здесь эта тема: Полносвязный блок может управлять потоком признаков через отдельные ветви.
// Почему код устроен так: Считаем gate и up отдельно, затем перемножаем после нелинейности SwiGLU.
// Представь: Если gate подавлен, даже большой сигнал ветки up почти не проходит в выход.
// Одна проекция открывает gate, другая несёт значения; затем идёт выходная проекция.

fn main() {
    lesson_trace::enable();
    let input: [f64; 2] = [1.0, -2.0];
    lesson_trace::trace_step!(input);
    let gate: f64 = input[0] - input[1];
    lesson_trace::trace_step!(gate);
    let up_projection: f64 = input[0] + input[1];
    lesson_trace::trace_step!(up_projection);
    let hidden: f64 =
        part_192_lesson_36_multiply_two_branches_and_control_output_with_smooth_gate::calculate_gated_layer_output_as_gate_times_up_value_over_one_plus_e_to_negative_gate(gate, up_projection);
    lesson_trace::trace_step!(hidden);
    let down: f64 = hidden * 0.5;
    lesson_trace::trace_step!(down);
    assert!(down.is_finite());
    println!("gate={gate}, up={up_projection}, hidden={hidden:.4}, down={down:.4}");
}
