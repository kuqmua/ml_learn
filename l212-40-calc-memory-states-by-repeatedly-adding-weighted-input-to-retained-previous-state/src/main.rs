// Урок 40.1. Последовательность состояний памяти: прибавление взвешенного входа к сохранённой доле прошлого состояния.
// Связь с принятой терминологией: Вычисление линейного рекуррентного состояния по последовательности входов.
// Зачем здесь эта тема: Последовательность можно хранить в компактном состоянии вместо внимания ко
//   всему прошлому.
// Почему код устроен так: Обновляем линейное состояние по одному входу и наблюдаем вклад старых
//   элементов.
// Представь: После каждого элемента последовательности храним одно число, в котором остаётся след
//   прошлого.
// Последовательность обрабатывается линейным сканированием с компактным состоянием.

use l212_40_calc_memory_states_by_repeatedly_adding_weighted_input_to_retained_previous_state::calc_memory_states_by_repeatedly_adding_weighted_input_to_retained_previous_state;

fn main() {
    let input: [f64; 4] = [1.0, 0.0, 0.0, 0.0];
    let states: [f64; 4] =
        calc_memory_states_by_repeatedly_adding_weighted_input_to_retained_previous_state(
            &input, 0.5, 1.0,
        );
    assert_eq!(states, [1.0, 0.5, 0.25, 0.125]);

    plot_stored_state_over_repeated_weighted_updates(&states);
}

fn plot_stored_state_over_repeated_weighted_updates(states: &[f64; 4]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "state-space",
        "Затухание состояния",
        "шаг",
        "h_t",
        &[lesson_visualization::Series {
            name: "состояние",
            points: &states
                .iter()
                .enumerate()
                .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
