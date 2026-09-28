// Урок 26.2. Обратное распространение через время.
// Градиент рекуррентного веса учитывает все предыдущие шаги.

use part_137_lesson_26_rnn_state::states;
fn loss(input: &[f64], wx: f64, wh: f64, target: f64) -> f64 {
    let last = *states(input, wx, wh).last().unwrap();
    0.5 * (last - target).powi(2)
}
fn main() {
    let input = [1.0, 0.5, -0.2];
    let wx = 0.3;
    let wh = 0.4;
    let target = 0.7;
    let history = states(&input, wx, wh);
    let mut dh = history.last().unwrap() - target;
    let mut gradient_wh = 0.0;
    for t in (0..input.len()).rev() {
        let h = history[t];
        let dz = dh * (1.0 - h * h);
        let previous = if t == 0 { 0.0 } else { history[t - 1] };
        gradient_wh += dz * previous;
        dh = dz * wh;
    }
    let epsilon = 1e-5;
    let numeric = (loss(&input, wx, wh + epsilon, target) - loss(&input, wx, wh - epsilon, target))
        / (2.0 * epsilon);
    assert!((gradient_wh - numeric).abs() < 1e-8);
    println!("BPTT gradient={gradient_wh:.6}; численная проверка={numeric:.6}");
}
