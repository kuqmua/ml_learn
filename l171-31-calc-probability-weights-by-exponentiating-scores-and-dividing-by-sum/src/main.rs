// Урок 31.5. Веса вероятностей (softmax): вычисление экспонент оценок и деление каждой на их сумму.
// Зачем здесь эта тема: Сырые оценки внимания могут быть отрицательными и не суммируются в единицу.
// Почему код устроен так: Применяем softmax, чтобы получить неотрицательные веса с суммой один.
// Представь: Для оценок [2, 1] softmax даёт два положительных веса, которые вместе составляют 1.
//
// Большая оценка получает больший вес. Равные оценки дают равные веса.
// Прибавление одной константы к обеим оценкам не меняет веса, а сумма весов равна 1.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

fn main() {
    let cases: [(&str, [f64; 2]); 4] = [
        ("равные оценки", [0.0, 0.0]),
        ("вторая оценка выше", [0.0, 1.0]),
        ("первая оценка выше", [1.0, 0.0]),
        ("к обеим прибавили 1", [1.0, 2.0]),
    ];
    let mut reference_weights: [f64; 2] = [0.0; 2];
    for (description, raw_model_scores) in cases {
        let mut exponentials: [f64; 2] = [0.0; 2];
        for index in 0..2 {
            let mut current_series_term: f64 = 1.0;
            let mut exponential_approximation: f64 = 1.0;
            for order in 1..=30 {
                current_series_term *= raw_model_scores[index] / order as f64;
                exponential_approximation += current_series_term;
            }
            exponentials[index] = exponential_approximation;
        }
        let sum_of_exponentials: f64 = exponentials[0] + exponentials[1];
        assert!(
            sum_of_exponentials > 0.0,
            "сумма экспонент должна быть положительной"
        );
        let attention_shares_summing_to_1: [f64; 2] = [
            exponentials[0] / sum_of_exponentials,
            exponentials[1] / sum_of_exponentials,
        ];
        assert!(check_f64_eq_1e_minus_10(
            attention_shares_summing_to_1[0] + attention_shares_summing_to_1[1],
            1.0
        ));
        assert!(
            attention_shares_summing_to_1
                .iter()
                .all(|&weight| (0.0..=1.0).contains(&weight))
        );
        match description {
            "равные оценки" => assert!(check_f64_eq_1e_minus_10(
                attention_shares_summing_to_1[0],
                attention_shares_summing_to_1[1]
            )),

            "вторая оценка выше" => {
                assert!(attention_shares_summing_to_1[1] > attention_shares_summing_to_1[0]);
                reference_weights = attention_shares_summing_to_1;
            }

            "первая оценка выше" => {
                assert!(attention_shares_summing_to_1[0] > attention_shares_summing_to_1[1])
            }

            "к обеим прибавили 1" => {
                assert!(check_f64_eq_1e_minus_10(
                    attention_shares_summing_to_1[0],
                    reference_weights[0]
                ));
                assert!(check_f64_eq_1e_minus_10(
                    attention_shares_summing_to_1[1],
                    reference_weights[1]
                ));
            }

            _ => unreachable!(),
        }
    }

    plot_second_probability_weight_as_its_exponential_divided_by_sum_of_two_exponentials();
}

// Строим график по результатам урока.
fn plot_second_probability_weight_as_its_exponential_divided_by_sum_of_two_exponentials() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Softmax двух логитов",
        "разность второго и первого",
        "вес второго",
        &[lesson_visualization::Series {
            name: "softmax",

            points: &(-60..=60)
                .map(|plot_step_index| {
                    let diff_between_raw_scores: f64 = plot_step_index as f64 / 10.0;
                    (
                        diff_between_raw_scores,
                        1.0 / (1.0 + (-diff_between_raw_scores).exp()),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
