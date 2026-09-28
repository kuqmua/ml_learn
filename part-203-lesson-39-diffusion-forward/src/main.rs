// Урок 39.1. Прямой процесс диффузии.
// При уменьшении доли исходного сигнала смесь становится ближе к шуму.

use part_203_lesson_39_diffusion_forward::add_noise;
fn main() {
    let clean = 2.0;
    let noise = -1.0;
    for alpha in [1.0, 0.75, 0.25, 0.0] {
        let noisy = add_noise(clean, noise, alpha).unwrap();
        println!("alpha_bar={alpha:.2}; x_t={noisy:.3}");
    }
    assert_eq!(add_noise(clean, noise, 1.0).unwrap(), clean);
    assert_eq!(add_noise(clean, noise, 0.0).unwrap(), noise);
    visualize(clean, noise);
}

fn visualize(clean: f64, noise: f64) {
    let points: Vec<_> = (0..=100)
        .map(|i| {
            let alpha = i as f64 / 100.0;
            (alpha, add_noise(clean, noise, alpha).unwrap())
        })
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "diffusion-forward",
        "Смесь сигнала и шума",
        "alpha_bar",
        "x_t",
        &[lesson_visualization::Series {
            name: "x_t",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
