// Урок 26.2. Расширение охвата истории: увеличение промежутков между значениями сигнала для фильтра.
// Связь с принятой терминологией: Рецептивное поле причинной свёртки с дилатацией.
// Зачем здесь эта тема: Обычная короткая свёртка видит мало прошлого; дилатация расширяет историю
//   без длинного ядра.
// Почему код устроен так: Пропускаем фиксированное число позиций между весами и считаем доступное
//   рецептивное поле.
// Представь: Фильтр с промежутками между весами видит более далёкое прошлое при том же числе весов.
// Дилатации 1, 2, 4 расширяют область прошлого без длинных фильтров.

use l142_26_calculate_causal_filter_output_by_summing_weighted_current_and_past_values::calculate_causal_filter_output_by_summing_weighted_current_and_spaced_past_values;

fn main() {
    let mut signal: [f64; 9] = [0.0; 9];
    signal[0] = 1.0;
    for filter_spacing in [1, 2, 4] {
        signal = calculate_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &signal,
            1.0,
            1.0,
            filter_spacing,
        )
        .unwrap()
        .try_into()
        .expect("фильтр выдаёт по одному значению на каждый из девяти отсчётов");
    }
    assert_eq!(signal[..8], [1.0; 8]);
    assert_eq!(signal[8], 0.0);
    plot_impulse_response_with_increasing_filter_spacing(&signal);
}

fn plot_impulse_response_with_increasing_filter_spacing(signal: &[f64; 9]) {
    let points: Vec<(f64, f64)> = signal
        .iter()
        .enumerate()
        .map(|(item_index, &vertical_value)| (item_index as f64, vertical_value))
        .collect();
    let _path: std::path::PathBuf = lesson_visualization::line_chart(
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
}
