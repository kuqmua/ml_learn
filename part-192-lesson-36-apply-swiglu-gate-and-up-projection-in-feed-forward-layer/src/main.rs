// Урок 36.5. Применение SwiGLU к gate и up проекциям полносвязного слоя.
// Одна проекция открывает gate, другая несёт значения; затем идёт выходная проекция.

fn main() {
    let input = [1.0, -2.0];
    let gate = input[0] - input[1];
    let up_projection = input[0] + input[1];
    let hidden =
        part_192_lesson_36_apply_swiglu_gate_and_up_projection_in_feed_forward_layer::swish_gated_linear_unit_of_gate_and_up_projection(gate, up_projection);
    let down = hidden * 0.5;
    assert!(down.is_finite());
    println!("gate={gate}, up={up_projection}, hidden={hidden:.4}, down={down:.4}");
}
