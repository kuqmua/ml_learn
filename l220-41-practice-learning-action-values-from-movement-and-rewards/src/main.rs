// Урок 220. Обновлять оценки действий по награде и лучшей оценке следующего состояния.
// Соединяем движение, исследование и повторные эпизоды, затем используем выученные оценки для
// движения к цели.

fn main() {
    fn move_agent_in_bounded_world_and_calc_reward(
        current_state: usize,
        action: usize,
    ) -> (usize, f64) {
        let next_state: usize = if action == 0 {
            current_state.saturating_sub(1)
        } else {
            if current_state < 4 {
                current_state + 1
            } else {
                4
            }
        };
        (next_state, if next_state == 4 { 1.0 } else { -0.01 })
    }

    let action_values: [[f64; 2]; 5] = (|| -> [[f64; 2]; 5] {
        let mut action_values: [[f64; 2]; 5] = [[0.0; 2]; 5];
        let mut generator_state: u64 = 42u64;
        for episode in 0..2000 {
            let mut current_state: usize = 0;
            for _ in 0..30 {
                generator_state = generator_state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1);
                let explore: bool = episode < 500 && generator_state % 5 == 0;
                let action: usize = if explore {
                    (generator_state >> 8) as usize % 2
                } else if action_values[current_state][1] >= action_values[current_state][0] {
                    1
                } else {
                    0
                };
                let (next_state, reward): (usize, f64) =
                    move_agent_in_bounded_world_and_calc_reward(current_state, action);
                action_values[current_state][action] = (|| -> f64 {
                    let current_value: f64 = action_values[current_state][action];
                    let reward: f64 = reward;
                    let best_future_value: f64 =
                        if action_values[next_state][0] > action_values[next_state][1] {
                            action_values[next_state][0]
                        } else {
                            action_values[next_state][1]
                        };
                    let learning_rate: f64 = 0.2;
                    let future_reward_weight: f64 = 0.95;
                    current_value
                        + learning_rate
                            * (reward + future_reward_weight * best_future_value - current_value)
                })();
                current_state = next_state;
                if current_state == 4 {
                    break;
                }
            }
        }
        action_values
    })();
    let mut current_state: usize = 0;
    while current_state < 4 {
        let (next_state, _): (usize, f64) = move_agent_in_bounded_world_and_calc_reward(
            current_state,
            usize::from(action_values[current_state][1] >= action_values[current_state][0]),
        );

        if next_state == current_state {
            break;
        }
        current_state = next_state;
    }

    // Выполняем вычисления из примера.
    let _ = action_values;

    println!("Выученные оценки действий={action_values:?}; достигнута ячейка={current_state}");
    assert_eq!(current_state, 4);
}

// Чему учит этот урок:
// Учимся обновлять оценки действий по награде и лучшей оценке следующего состояния.
// Соединяем движение, исследование и повторные эпизоды, затем используем выученные оценки для
// движения к цели.
