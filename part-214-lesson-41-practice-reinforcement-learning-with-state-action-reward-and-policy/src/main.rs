// Сводная практика 41. Сводная практика.
// Почему этот урок сейчас: Полный опыт агента соединяет состояние, действие, переход, награду и обновление решения.
// Почему пример устроен так: Запускаем короткие эпизоды в ограниченной среде, где каждое последствие видно вручную.
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
    lesson_trace::enable();
    // Мир из пяти клеток: старт 0, цель 4, действия — влево и вправо.
    fn move_agent_in_bounded_world_and_calculate_reward(
        current_state: usize,
        action: usize,
    ) -> (usize, f64) {
        // Сохраняем рассчитанное значение `next_state` для следующих операций.
        let next_state: usize = if action == 0 {
            // Сдвигаемся влево, не уходя за нулевую клетку.
            current_state.saturating_sub(1)
        // Обрабатываем случай, когда предыдущее условие не выполнено.
        } else {
            // Проверяем условие и выбираем соответствующую ветку алгоритма.
            if current_state < 4 {
                // Складываем или вычитаем величины согласно используемой формуле.
                current_state + 1
            // Обрабатываем случай, когда предыдущее условие не выполнено.
            } else {
                // Используем фиксированное значение для этого варианта примера.
                4
            }
        };
        lesson_trace::trace_step!(next_state);
        // Составляем результат из вычисленных значений в указанном порядке.
        (next_state, if next_state == 4 { 1. } else { -0.01 })
    }

    // Шаг: Обучаем таблицу ценности действий на эпизодах GridWorld.
    let action_values: [[f64; 2]; 5] = (|| -> [[f64; 2]; 5] {
        // Используем подготовленное значение в следующем шаге примера.
        /* Q-learning обновляет ценность действия через награду и лучшую оценку следующего состояния. */
        // Задаём учебные значения для `action_values`.
        let mut action_values: [[f64; 2]; 5] = [[0.; 2]; 5];
        lesson_trace::trace_step!(action_values);
        // Создаём изменяемое значение `generator_state` для следующих операций.
        let mut generator_state: u64 = 42u64;
        lesson_trace::trace_step!(generator_state);
        // 2000 учебных эпизодов дают значениям действий время распространить награду от клетки 4 к старту.
        // Каждый эпизод начинается в нулевой клетке; в первых эпизодах исследуем случайные действия.
        for episode in 0..2000 {
            lesson_trace::trace_step!(episode);
            // Инициализируем изменяемый накопитель `current_state` начальным состоянием.
            let mut current_state: usize = 0;
            lesson_trace::trace_step!(current_state);
            // Ограничиваем эпизод 30 переходами, чтобы блуждание не длилось бесконечно.
            // При достижении целевой клетки 4 цикл завершается раньше.
            for _ in 0..30 {
                // Линейный конгруэнтный генератор: фиксированный множитель и +1 по mod 2⁶⁴.
                generator_state = generator_state
                    // Используем арифметику с переполнением для воспроизводимого генератора.
                    .wrapping_mul(6364136223846793005)
                    // Используем арифметику с переполнением для воспроизводимого генератора.
                    .wrapping_add(1);
                lesson_trace::trace_step!(generator_state);
                // Первые 500 эпизодов исследуем: остаток 0 при делении на 5 даёт примерно 20% случайных ходов.
                let explore: bool = episode < 500 && generator_state % 5 == 0;
                lesson_trace::trace_step!(explore);
                // Сохраняем рассчитанное значение `action` для следующих операций.
                let action: usize = if explore {
                    // Составляем результат из вычисленных значений в указанном порядке.
                    (generator_state >> 8) as usize % 2
                // Обновляем значение результатом текущего вычисления.
                } else if action_values[current_state][1] >= action_values[current_state][0] {
                    // Используем фиксированное значение для этого варианта примера.
                    1
                // Обрабатываем случай, когда предыдущее условие не выполнено.
                } else {
                    // Используем фиксированное значение для этого варианта примера.
                    0
                };
                lesson_trace::trace_step!(action);
                // Сохраняем рассчитанное значение `(next_state, reward)` для следующих операций.
                let (next_state, reward): (usize, f64) =
                    move_agent_in_bounded_world_and_calculate_reward(current_state, action);
                lesson_trace::trace_step!(next_state);
                lesson_trace::trace_step!(reward);
                // Цель Беллмана = награда сейчас + лучшая оценка следующего состояния.
                let future: f64 = if action_values[next_state][0] > action_values[next_state][1] {
                    // Берём ценность действия из следующего состояния для обновления Q-оценки.
                    action_values[next_state][0]
                // Обрабатываем случай, когда предыдущее условие не выполнено.
                } else {
                    // Берём ценность действия из следующего состояния для обновления Q-оценки.
                    action_values[next_state][1]
                };
                lesson_trace::trace_step!(future);
                // Обновляем `action_values[current_state][action]` результатом текущего шага.
                action_values[current_state][action] = (|| -> f64 {
                    // Обновляем значение результатом текущего вычисления.
                    /* Формула Q-learning: новая оценка = старая + alpha * (reward + gamma*future − старая). */
                    // Сохраняем результат этого шага в `current_value`.
                    let current_value: f64 = action_values[current_state][action];
                    lesson_trace::trace_step!(current_value);
                    // Сохраняем рассчитанное значение `reward` для следующих операций.
                    let reward: f64 = reward;
                    lesson_trace::trace_step!(reward);
                    // Сохраняем рассчитанное значение `best_future_value` для следующих операций.
                    let best_future_value: f64 = future;
                    lesson_trace::trace_step!(best_future_value);
                    // 0.2 смешивает 20% новой оценки ценности действия с 80% старой.
                    let learning_rate: f64 = 0.2;
                    lesson_trace::trace_step!(learning_rate);
                    // 0.95 оставляет будущей награде 95% её веса на каждом переходе.
                    let discount_factor: f64 = 0.95;
                    lesson_trace::trace_step!(discount_factor);
                    // Используем ранее рассчитанное значение `current_value` в текущем выражении.
                    current_value
                        // Складываем или вычитаем величины согласно используемой формуле.
                        + learning_rate
                            // Добавляем этот член в составное арифметическое выражение.
                            * (reward + discount_factor * best_future_value - current_value)
                })();
                lesson_trace::trace_step!(action_values);
                // Обновляем `current_state` результатом текущего шага.
                current_state = next_state;
                lesson_trace::trace_step!(current_state);
                // Проверяем условие и выбираем соответствующую ветку алгоритма.
                if current_state == 4 {
                    // Останавливаем цикл после достижения условия завершения.
                    break;
                }
            }
        }
        // Используем ранее рассчитанное значение `action_values` в текущем выражении.
        action_values
    })();
    lesson_trace::trace_step!(action_values);
    // Шаг: Начинаем новый проход из стартовой клетки.
    let mut current_state: usize = 0;
    lesson_trace::trace_step!(current_state);
    // Шаг: Выбираем лучшее известное действие и проверяем путь к цели.
    while current_state < 4 {
        // Сохраняем рассчитанное значение `action` для следующих операций.
        let action: usize =
            // `usize` задаёт соответствующее входное значение или поле структуры.
            usize::from(action_values[current_state][1] >= action_values[current_state][0]);
        lesson_trace::trace_step!(action);
        // Сохраняем рассчитанное значение `(next_state, _)` для следующих операций.
        let (next_state, _): (usize, f64) =
            move_agent_in_bounded_world_and_calculate_reward(current_state, action);
        lesson_trace::trace_step!(next_state);
        // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
        println!("{current_state} --{action}--> {next_state}");
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if next_state == current_state {
            // Останавливаем цикл после достижения условия завершения.
            break;
        }
        // Обновляем `current_state` результатом текущего шага.
        current_state = next_state;
        lesson_trace::trace_step!(current_state);
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_reinforcement_learning_with_state_action_reward_and_policy(action_values);
}

// Строим график по результатам урока.
fn visualize_practice_reinforcement_learning_with_state_action_reward_and_policy(
    action_values: [[f64; 2]; 5],
) {
    // Значения из этого урока на графике.
    let reinforcement_learning_points: Vec<(f64, f64)> = action_values
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(item_index, row)| (item_index as f64, row[0].max(row[1])))
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Ценность состояния после обучения",
        // Указываем подпись горизонтальной оси.
        "состояние",
        // Указываем подпись вертикальной оси.
        "лучшее Q",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "Q-таблица",
            // Передаём рассчитанные координаты точек.
            points: &reinforcement_learning_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
