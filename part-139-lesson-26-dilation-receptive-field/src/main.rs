// Урок 26.2. Дилатация и рецептивное поле.
// Дилатации 1, 2, 4 расширяют область прошлого без длинных фильтров.

use part_138_lesson_26_causal_convolution::causal_convolution;
fn main() {
    let mut signal = vec![0.0; 9];
    signal[0] = 1.0;
    // Промежуток между используемыми точками фильтра называют dilation.
    for filter_spacing in [1, 2, 4] {
        signal = causal_convolution(&signal, 1.0, 1.0, filter_spacing).unwrap();
        println!("после dilation={filter_spacing}: {signal:?}");
    }
    // Три двухточечных слоя видят 1 + 1 + 2 + 4 = 8 временных шагов.
    assert_eq!(signal[..8], [1.0; 8]);
    assert_eq!(signal[8], 0.0);
    visualize(&signal);
}

fn visualize(signal: &[f64]) {
    let points: Vec<_> = signal
        .iter()
        .enumerate()
        .map(|(item_index, &vertical_value)| (item_index as f64, vertical_value))
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "receptive-field",
        "Отклик на импульс",
        "t",
        "отклик",
        &[lesson_visualization::Series {
            name: "dilation 1,2,4",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
