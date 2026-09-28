// Урок 41.5. SwiGLU в feed-forward слое.
// Одна проекция открывает gate, другая несёт значения; затем идёт выходная проекция.

use part_212_lesson_41_swiglu::swiglu;
fn main() {
    let input = [1.0, -2.0];
    let gate = input[0] - input[1];
    let up = input[0] + input[1];
    let hidden = swiglu(gate, up);
    let down = hidden * 0.5;
    assert!(down.is_finite());
    println!("gate={gate}, up={up}, hidden={hidden:.4}, down={down:.4}");
}
