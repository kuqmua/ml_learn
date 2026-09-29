// Урок 26.2. Рецептивное поле причинной свёртки с дилатацией.
// Почему этот урок сейчас: Обычная короткая свёртка видит мало прошлого; дилатация расширяет историю без длинного ядра.
// Почему пример устроен так: Пропускаем фиксированное число позиций между весами и считаем доступное рецептивное поле.
// Дилатации 1, 2, 4 расширяют область прошлого без длинных фильтров.

fn main() {
    lesson_trace::enable();
    let mut signal: Vec<f64> = vec![0.0; 9];
    lesson_trace::trace_step!(signal);
    signal[0] = 1.0;
    lesson_trace::trace_step!(signal);
    // Промежуток между используемыми точками фильтра называют dilation.
    for filter_spacing in [1, 2, 4] {
        lesson_trace::trace_step!(filter_spacing);
        signal =
            part_138_lesson_26_causal_convolution_over_one_dimensional_signal::causal_convolution_of_one_dimensional_signal(
                &signal,
                1.0,
                1.0,
                filter_spacing,
            )
            .unwrap();
        lesson_trace::trace_step!(signal);
        println!("после dilation={filter_spacing}: {signal:?}");
    }
    // Три двухточечных слоя видят 1 + 1 + 2 + 4 = 8 временных шагов.
    assert_eq!(signal[..8], [1.0; 8]);
    assert_eq!(signal[8], 0.0);
    lesson_trace::disable();
    visualize_dilated_causal_convolution_receptive_field(&signal);
}

fn visualize_dilated_causal_convolution_receptive_field(signal: &[f64]) {
    let points: Vec<(f64, f64)> = signal
        .iter()
        .enumerate()
        .map(|(item_index, &vertical_value)| (item_index as f64, vertical_value))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
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
