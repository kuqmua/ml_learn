// Урок 26.1. Отклик фильтра без будущих данных: сложение взвешенных текущего и прошлого значений сигнала.
// Связь с принятой терминологией: Причинная свёртка одномерного сигнала.
// Зачем здесь эта тема: Для прогноза следующего элемента нельзя читать будущие элементы
//   последовательности.
// Почему код устроен так: Свертка использует только текущий и прошлые индексы, что проверяется
//   изменением будущего входа.
// Представь: Изменение сигнала завтра не должно менять прогноз, сделанный сегодня.
// Выход в момент t зависит от текущего и прошлых элементов, но не от будущего.

use l142_26_calculate_causal_filter_output_by_summing_weighted_current_and_past_values::calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values;

fn main() {
    let signal: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
    let output: [f64; 4] =
        calc_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &signal, 1.0, 2.0, 1,
        )
        .unwrap()
        .try_into()
        .expect("ожидалось по одному выходу на каждый отсчёт сигнала");
    assert_eq!(output, [1.0, 4.0, 7.0, 10.0]);

    plot_input_signal_and_weighted_current_and_past_sums(&signal, &output);
}
fn plot_input_signal_and_weighted_current_and_past_sums(input: &[f64; 4], output: &[f64; 4]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "causal-conv",
        "Причинная свёртка",
        "t",
        "амплитуда",
        &[
            lesson_visualization::Series {
                name: "вход",
                points: &input
                    .iter()
                    .enumerate()
                    .map(|(item_index, &vertical_value)| (item_index as f64, vertical_value))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "выход",
                points: &output
                    .iter()
                    .enumerate()
                    .map(|(item_index, &vertical_value)| (item_index as f64, vertical_value))
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
