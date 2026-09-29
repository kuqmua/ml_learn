// Урок 36.5. Применение SwiGLU к gate и up проекциям полносвязного слоя.
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
        part_192_lesson_36_apply_swiglu_gate_and_up_projection_in_feed_forward_layer::swish_gated_linear_unit_of_gate_and_up_projection(gate, up_projection);
    lesson_trace::trace_step!(hidden);
    let down: f64 = hidden * 0.5;
    lesson_trace::trace_step!(down);
    assert!(down.is_finite());
    println!("gate={gate}, up={up_projection}, hidden={hidden:.4}, down={down:.4}");
}
