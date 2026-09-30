// Урок 26.4. Остаточный и отдельный выходы блока: прибавление преобразования ко входу и передача его отдельно.
// Связь с принятой терминологией: Остаточные и сквозные связи в блоке WaveNet.
// Зачем здесь эта тема: Глубокая цепочка причинных блоков должна сохранять вход и передавать
//   промежуточные признаки к выходу.
// Почему код устроен так: Разделяем прибавление входа для следующего блока и skip для итоговой суммы.
// Представь: Прибавление входа сохраняет путь для следующего блока, а skip отправляет часть сигнала сразу к
//   общему выходу.
// Прибавление входа переносит состояние через слои, skip собирает вклады для выхода.

/// Остаточная и пропускная связи: возвращаем (input + tanh(input·transform), tanh(input·transform)).

fn calculate_input_plus_transform_and_separate_transform_output(
    input: f64,
    transform: f64,
) -> (f64, f64) {
    let activation: f64 = (input * transform).tanh();
    let input_plus_transformed_value: f64 = input + activation;
    let skip: f64 = activation;
    (input_plus_transformed_value, skip)
}
fn main() {
    let mut state: f64 = 0.5;
    let mut skip_sum: f64 = 0.0;
    for transform in [0.2, -0.4, 0.8] {
        let (next, skip): (f64, f64) =
            calculate_input_plus_transform_and_separate_transform_output(state, transform);
        state = next;
        skip_sum += skip;
    }
    assert!((state - (0.5 + skip_sum)).abs() < 1e-12);
}
