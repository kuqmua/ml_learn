// Урок 26.4. Остаточный и отдельный выходы блока: прибавление преобразования ко входу и передача его отдельно.
// Зачем здесь эта тема: Глубокая цепочка причинных блоков должна сохранять вход и передавать
//   промежуточные признаки к выходу.
// Почему код устроен так: Разделяем прибавление входа для следующего блока и skip для итоговой суммы.
// Представь: Прибавление входа сохраняет путь для следующего блока, а skip отправляет часть сигнала сразу к
//   общему выходу.
// Прибавление входа переносит состояние через слои, skip собирает вклады для выхода.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

/// Остаточная и пропускная связи: возвращаем (input + tanh(input·transform), tanh(input·transform)).

fn calc_input_plus_transform_and_separate_transform_output(
    input: f64,
    transform: f64,
) -> (f64, f64) {
    let tanh_bounded_transform_between_minus_1_and_1: f64 = calc_tanh(input * transform);
    let input_plus_transformed_value: f64 = input + tanh_bounded_transform_between_minus_1_and_1;
    let transformed_signal_passed_to_output_without_adding_input: f64 =
        tanh_bounded_transform_between_minus_1_and_1;
    (
        input_plus_transformed_value,
        transformed_signal_passed_to_output_without_adding_input,
    )
}
fn main() {
    let mut state: f64 = 0.5;
    let mut sum_of_transformed_signals_from_all_layers: f64 = 0.0;
    for transform in [0.2, -0.4, 0.8] {
        let (next, transformed_signal_passed_to_output_without_adding_input): (f64, f64) =
            calc_input_plus_transform_and_separate_transform_output(state, transform);
        state = next;
        sum_of_transformed_signals_from_all_layers +=
            transformed_signal_passed_to_output_without_adding_input;
    }
    assert!(check_f64_eq_1e_minus_12(
        state,
        0.5 + sum_of_transformed_signals_from_all_layers
    ));
}

/// tanh сохраняет знак, равен 0 при нулевом входе и насыщается к −1 или 1.
fn calc_tanh(input: f64) -> f64 {
    input.tanh()
}

// Чему учит этот урок:
// Учимся передавать дальше вход с добавленной поправкой и отдельно накапливать сами поправки.
// Проверяем связь конечного состояния с исходным значением и суммой преобразований всех шагов.
