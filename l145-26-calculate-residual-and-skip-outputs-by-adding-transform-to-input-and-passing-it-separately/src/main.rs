// Урок 26.4. Остаточный и отдельный выходы блока: прибавление преобразования ко входу и передача его отдельно.
// Связь с принятой терминологией: Остаточные и сквозные связи в блоке WaveNet.
// Зачем здесь эта тема: Глубокая цепочка причинных блоков должна сохранять вход и передавать
//   промежуточные признаки к выходу.
// Почему код устроен так: Разделяем residual для следующего блока и skip для итоговой суммы.
// Представь: Residual сохраняет путь для следующего блока, а skip отправляет часть сигнала сразу к
//   общему выходу.
// Residual переносит состояние через слои, skip собирает вклады для выхода.

/// Остаточная и пропускная связи: возвращаем (input + tanh(input·transform), tanh(input·transform)).

fn calculate_residual_and_skip_outputs_as_input_plus_tanh_transform_and_transform_separately(
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
            calculate_residual_and_skip_outputs_as_input_plus_tanh_transform_and_transform_separately(state, transform);
        state = next;
        skip_sum += skip;
    }
    assert!((state - (0.5 + skip_sum)).abs() < 1e-12);
}
