// Урок 41.6. Практика: обучение оценкам действий по перемещениям и наградам.
// Зачем здесь эта тема: Полный опыт агента соединяет состояние, действие, переход, награду и
//   обновление решения.
// Почему код устроен так: Запускаем короткие эпизоды в ограниченной среде, где каждое последствие
//   видно вручную.
// Представь: Агент видит клетку, выбирает шаг, получает новую клетку и награду, затем меняет оценку
//   действий.
//
// Что повторяем вместе: состояние, действие, награда, политика, exploration/exploitation.
// Зачем это нужно: Агент учится выбирать действие по награде и оценке будущего состояния, постепенно
//   улучшая таблицу Q-значений.
// Что показывает программа: Обучаем таблицу ценности действий на эпизодах GridWorld. Начинаем новый проход
//   из стартовой клетки. Выбираем лучшее известное действие и проверяем путь к цели.
// Что проверить при изменении примера: Сравни среднюю награду до и после обучения на отдельных эпизодах с
//   фиксированными seed.
// Дополнительная практика: Сделай GridWorld и tabular Q-learning с epsilon-greedy политикой.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    fn move_agent_in_bounded_world_and_calculate_reward(
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
        (next_state, if next_state == 4 { 1. } else { -0.01 })
    }

    let action_values: [[f64; 2]; 5] = (|| -> [[f64; 2]; 5] {
        let mut action_values: [[f64; 2]; 5] = [[0.; 2]; 5];
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
                    move_agent_in_bounded_world_and_calculate_reward(current_state, action);
                let future: f64 = if action_values[next_state][0] > action_values[next_state][1] {
                    action_values[next_state][0]
                } else {
                    action_values[next_state][1]
                };
                action_values[current_state][action] = (|| -> f64 {
                    let current_value: f64 = action_values[current_state][action];
                    let reward: f64 = reward;
                    let best_future_value: f64 = future;
                    let learning_rate: f64 = 0.2;
                    let discount_factor: f64 = 0.95;
                    current_value
                        + learning_rate
                            * (reward + discount_factor * best_future_value - current_value)
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
        let action: usize =
            usize::from(action_values[current_state][1] >= action_values[current_state][0]);
        let (next_state, _): (usize, f64) =
            move_agent_in_bounded_world_and_calculate_reward(current_state, action);

        if next_state == current_state {
            break;
        }
        current_state = next_state;
    }

    plot_best_estimated_action_reward_at_each_position(action_values);
}

// Строим график по результатам урока.
fn plot_best_estimated_action_reward_at_each_position(action_values: [[f64; 2]; 5]) {
    let reinforcement_learning_points: Vec<(f64, f64)> = action_values
        .iter()
        .enumerate()
        .map(|(item_index, row)| (item_index as f64, row[0].max(row[1])))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ценность состояния после обучения",
        "состояние",
        "лучшее Q",
        &[lesson_visualization::Series {
            name: "Q-таблица",

            points: &reinforcement_learning_points,
        }],
    )
    .expect("не удалось сохранить график");
}
