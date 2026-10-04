// Урок 138. Обновлять состояние последовательности по текущему входу и предыдущему состоянию.
// На одном импульсе и следующих нулях прослеживаем, как информация о прошлом сохраняется и
// ослабевает.

use l138_25_calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state::calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state;

fn main() {
    let input: [f64; 4] = [1.0, 0.0, 0.0, 0.0];

    // Выполняем вычисления из примера.
    let _ = (&std::convert::TryInto::<[f64; 4]>::try_into(
        calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state(
            &input, 0.8, 0.7,
        ),
    )
    .expect("ожидалось по одному состоянию на каждый входной шаг"),);

    let input = [1.0, 0.0, 0.0, 0.0];
    let remembered =
        calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state(
            &input, 0.8, 0.7,
        );
    let forgotten =
        calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state(
            &input, 0.8, 0.0,
        );
    println!("Вход={input:?}; с прошлым={remembered:?}; без прошлого={forgotten:?}");
    assert!(remembered[1] > 0.0);
    assert_eq!(forgotten[1], 0.0);
}

// Чему учит этот урок:
// Учимся обновлять состояние последовательности по текущему входу и предыдущему состоянию.
// На одном импульсе и следующих нулях прослеживаем, как информация о прошлом сохраняется и
// ослабевает.
