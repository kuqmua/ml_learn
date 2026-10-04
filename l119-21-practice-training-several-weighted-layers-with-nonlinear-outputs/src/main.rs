// Урок 119. Соединять скрытый слой, нелинейные выходы, обратный расчёт производных и обновление
// весов.
// Обучаем небольшую сеть на XOR — задаче, где ответ зависит от сочетания двух входов.

fn main() {
    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    /// Экспонента eˣ: ряд Тейлора 1 + x + x²/2! + x³/3! + … с уменьшением аргумента и восстановлением масштаба.
    fn approximate_e_to_power_by_summing_power_over_factorial_terms(value: f64) -> f64 {
        if value == f64::NEG_INFINITY || value < -745.0 {
            return 0.0;
        }
        if value == f64::INFINITY || value > 709.0 {
            return f64::INFINITY;
        }
        if value < 0.0 {
            return 1.0 / approximate_e_to_power_by_summing_power_over_factorial_terms(-value);
        }
        let mut reduced: f64 = value;
        let mut halving_count: i32 = 0;
        while reduced > 0.5 {
            reduced /= 2.0;
            halving_count += 1;
        }
        let mut term: f64 = 1.0;
        let mut exponential_approximation: f64 = 1.0;
        for term_index in 1..=30 {
            term *= reduced / term_index as f64;
            exponential_approximation += term;
        }
        for _ in 0..halving_count {
            exponential_approximation *= exponential_approximation;
        }
        exponential_approximation
    }

    const XOR: [([f64; 2], f64); 4] = [
        ([0.0, 0.0], 0.0),
        ([0.0, 1.0], 1.0),
        ([1.0, 0.0], 1.0),
        ([1.0, 1.0], 0.0),
    ];

    /// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.
    fn calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(raw_model_score: f64) -> f64 {
        1.0 / (1.0 + approximate_e_to_power_by_summing_power_over_factorial_terms(-raw_model_score))
    }
    /// Производная сигмоиды по её входу: output·(1−output), если output — уже вычисленная сигмоида.
    fn calc_sigmoid_slope_by_multiplying_output_by_one_minus_output(sigmoid_output: f64) -> f64 {
        sigmoid_output * (1.0 - sigmoid_output)
    }

    #[derive(Debug)]
    struct NeuralNetwork {
        hidden_weights: [[f64; 3]; 2],

        output_weights: [f64; 3],
    }

    let network: NeuralNetwork = (|| -> NeuralNetwork {
        let mut network: NeuralNetwork = NeuralNetwork {
            hidden_weights: [[0.8, -0.5, 0.2], [-0.3, 0.9, -0.1]],

            output_weights: [0.7, -0.8, 0.1],
        };
        for epoch in 0..20_000 {
            for &(features, expected_output) in &XOR {
                let hidden_outputs: [f64; 2] = [
                    calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
                        network.hidden_weights[0][0] * features[0]
                            + network.hidden_weights[0][1] * features[1]
                            + network.hidden_weights[0][2],
                    ),
                    calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
                        network.hidden_weights[1][0] * features[0]
                            + network.hidden_weights[1][1] * features[1]
                            + network.hidden_weights[1][2],
                    ),
                ];
                let output_probability: f64 =
                    calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
                        network.output_weights[0] * hidden_outputs[0]
                            + network.output_weights[1] * hidden_outputs[1]
                            + network.output_weights[2],
                    );
                let output_loss_rate_of_change: f64 = (|| -> f64 {
                    let prediction: f64 = output_probability;
                    let target: f64 = expected_output;
                    prediction - target
                })()
                    * calc_sigmoid_slope_by_multiplying_output_by_one_minus_output(
                        output_probability,
                    );
                let hidden_layer_loss_rates_of_change: [f64; 2] = [
                    output_loss_rate_of_change
                        * network.output_weights[0]
                        * calc_sigmoid_slope_by_multiplying_output_by_one_minus_output(
                            hidden_outputs[0],
                        ),
                    output_loss_rate_of_change
                        * network.output_weights[1]
                        * calc_sigmoid_slope_by_multiplying_output_by_one_minus_output(
                            hidden_outputs[1],
                        ),
                ];
                let learning_rate: f64 = 0.5;
                for hidden_neuron_index in 0..2 {
                    network.output_weights[hidden_neuron_index] -= learning_rate
                        * output_loss_rate_of_change
                        * hidden_outputs[hidden_neuron_index];
                    for feature_index in 0..2 {
                        network.hidden_weights[hidden_neuron_index][feature_index] -= learning_rate
                            * hidden_layer_loss_rates_of_change[hidden_neuron_index]
                            * features[feature_index];
                    }
                    network.hidden_weights[hidden_neuron_index][2] -=
                        learning_rate * hidden_layer_loss_rates_of_change[hidden_neuron_index];
                }
                network.output_weights[2] -= learning_rate * output_loss_rate_of_change;
            }
            if matches!(epoch, 0 | 1 | 9 | 99 | 999 | 9_999 | 19_999) {
                let _ = &(epoch + 1);
            }
        }
        network
    })();
    for (features, _expected_output) in XOR {
        let output_probability: f64 = {
            let hidden_outputs: [f64; 2] = [
                calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
                    network.hidden_weights[0][0] * features[0]
                        + network.hidden_weights[0][1] * features[1]
                        + network.hidden_weights[0][2],
                ),
                calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
                    network.hidden_weights[1][0] * features[0]
                        + network.hidden_weights[1][1] * features[1]
                        + network.hidden_weights[1][2],
                ),
            ];
            calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
                network.output_weights[0] * hidden_outputs[0]
                    + network.output_weights[1] * hidden_outputs[1]
                    + network.output_weights[2],
            )
        };
        let _ = &(output_probability);
    }

    let mut correct = 0;
    for (input, target) in XOR {
        let hidden = network.hidden_weights.map(|weights| {
            calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
                weights[0] * input[0] + weights[1] * input[1] + weights[2],
            )
        });
        let probability = calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(
            network.output_weights[0] * hidden[0]
                + network.output_weights[1] * hidden[1]
                + network.output_weights[2],
        );
        let predicted = probability >= 0.5;
        println!(
            "XOR {input:?}: вероятность={probability:.4}, ответ={predicted}, ожидаем={target}"
        );
        correct += usize::from(predicted == (target == 1.0));
    }
    assert_eq!(correct, 4);
}

// Чему учит этот урок:
// Учимся соединять скрытый слой, нелинейные выходы, обратный расчёт производных и обновление
// весов.
// Обучаем небольшую сеть на XOR — задаче, где ответ зависит от сочетания двух входов.
