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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Мир из пяти клеток: старт 0, цель 4, действия — влево и вправо.");
    fn move_agent_in_bounded_world_and_calculate_reward(
        current_state: usize,
        action: usize,
    ) -> (usize, f64) {
        trace_note!("Сохраняем рассчитанное значение `next_state` для следующих операций.");
        let next_state: usize = if action == 0 {
            trace_note!("Сдвигаемся влево, не уходя за нулевую клетку.");
            current_state.saturating_sub(1)
        } else {
            trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
            trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
            if current_state < 4 {
                trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
                current_state + 1
            } else {
                trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                trace_note!("Используем фиксированное значение для этого варианта примера.");
                4
            }
        };
        trace_step!(next_state);
        trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
        (next_state, if next_state == 4 { 1. } else { -0.01 })
    }

    trace_note!("Шаг: Обучаем таблицу ценности действий на эпизодах GridWorld.");
    let action_values: [[f64; 2]; 5] = (|| -> [[f64; 2]; 5] {
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!(
            "Q-learning обновляет ценность действия через награду и лучшую оценку следующего состояния."
        );
        trace_note!("Задаём учебные значения для `action_values`.");
        let mut action_values: [[f64; 2]; 5] = [[0.; 2]; 5];
        trace_step!(action_values);
        trace_note!("Создаём изменяемое значение `generator_state` для следующих операций.");
        let mut generator_state: u64 = 42u64;
        trace_step!(generator_state);
        trace_note!(
            "2000 учебных эпизодов дают значениям действий время распространить награду от клетки 4 к старту."
        );
        trace_note!(
            "Каждый эпизод начинается в нулевой клетке; в первых эпизодах исследуем случайные действия."
        );
        for episode in 0..2000 {
            trace_step!(episode);
            trace_note!(
                "Инициализируем изменяемый накопитель `current_state` начальным состоянием."
            );
            let mut current_state: usize = 0;
            trace_step!(current_state);
            trace_note!(
                "Ограничиваем эпизод 30 переходами, чтобы блуждание не длилось бесконечно."
            );
            trace_note!("При достижении целевой клетки 4 цикл завершается раньше.");
            for _ in 0..30 {
                trace_note!(
                    "Линейный конгруэнтный генератор: фиксированный множитель и +1 по mod 2⁶⁴."
                );
                trace_note!(
                    "Используем арифметику с переполнением для воспроизводимого генератора."
                );
                trace_note!(
                    "Используем арифметику с переполнением для воспроизводимого генератора."
                );
                generator_state = generator_state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1);
                trace_step!(generator_state);
                trace_note!(
                    "Первые 500 эпизодов исследуем: остаток 0 при делении на 5 даёт примерно 20% случайных ходов."
                );
                let explore: bool = episode < 500 && generator_state % 5 == 0;
                trace_step!(explore);
                trace_note!("Сохраняем рассчитанное значение `action` для следующих операций.");
                let action: usize = if explore {
                    trace_note!(
                        "Составляем результат из вычисленных значений в указанном порядке."
                    );
                    trace_note!("Обновляем значение результатом текущего вычисления.");
                    (generator_state >> 8) as usize % 2
                } else if action_values[current_state][1] >= action_values[current_state][0] {
                    trace_note!("Используем фиксированное значение для этого варианта примера.");
                    1
                } else {
                    trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                    trace_note!("Используем фиксированное значение для этого варианта примера.");
                    0
                };
                trace_step!(action);
                trace_note!(
                    "Сохраняем рассчитанное значение `(next_state, reward)` для следующих операций."
                );
                let (next_state, reward): (usize, f64) =
                    move_agent_in_bounded_world_and_calculate_reward(current_state, action);
                trace_step!(next_state);
                trace_step!(reward);
                trace_note!("Цель Беллмана = награда сейчас + лучшая оценка следующего состояния.");
                let future: f64 = if action_values[next_state][0] > action_values[next_state][1] {
                    trace_note!(
                        "Берём ценность действия из следующего состояния для обновления Q-оценки."
                    );
                    action_values[next_state][0]
                } else {
                    trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                    trace_note!(
                        "Берём ценность действия из следующего состояния для обновления Q-оценки."
                    );
                    action_values[next_state][1]
                };
                trace_step!(future);
                trace_note!(
                    "Обновляем `action_values[current_state][action]` результатом текущего шага."
                );
                action_values[current_state][action] = (|| -> f64 {
                    trace_note!("Обновляем значение результатом текущего вычисления.");
                    trace_note!(
                        "Формула Q-learning: новая оценка = старая + alpha * (reward + gamma*future − старая)."
                    );
                    trace_note!("Сохраняем результат этого шага в `current_value`.");
                    let current_value: f64 = action_values[current_state][action];
                    trace_step!(current_value);
                    trace_note!("Сохраняем рассчитанное значение `reward` для следующих операций.");
                    let reward: f64 = reward;
                    trace_step!(reward);
                    trace_note!(
                        "Сохраняем рассчитанное значение `best_future_value` для следующих операций."
                    );
                    let best_future_value: f64 = future;
                    trace_step!(best_future_value);
                    trace_note!("0.2 смешивает 20% новой оценки ценности действия с 80% старой.");
                    let learning_rate: f64 = 0.2;
                    trace_step!(learning_rate);
                    trace_note!("0.95 оставляет будущей награде 95% её веса на каждом переходе.");
                    let discount_factor: f64 = 0.95;
                    trace_step!(discount_factor);
                    trace_note!(
                        "Используем ранее рассчитанное значение `current_value` в текущем выражении."
                    );
                    trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
                    trace_note!("Добавляем этот член в составное арифметическое выражение.");
                    current_value
                        + learning_rate
                            * (reward + discount_factor * best_future_value - current_value)
                })();
                trace_step!(action_values);
                trace_note!("Обновляем `current_state` результатом текущего шага.");
                current_state = next_state;
                trace_step!(current_state);
                trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
                if current_state == 4 {
                    trace_note!("Останавливаем цикл после достижения условия завершения.");
                    break;
                }
            }
        }
        trace_note!("Используем ранее рассчитанное значение `action_values` в текущем выражении.");
        action_values
    })();
    trace_step!(action_values);
    trace_note!("Шаг: Начинаем новый проход из стартовой клетки.");
    let mut current_state: usize = 0;
    trace_step!(current_state);
    trace_note!("Шаг: Выбираем лучшее известное действие и проверяем путь к цели.");
    while current_state < 4 {
        trace_note!("Сохраняем рассчитанное значение `action` для следующих операций.");
        trace_note!("`usize` задаёт соответствующее входное значение или поле структуры.");
        let action: usize =
            usize::from(action_values[current_state][1] >= action_values[current_state][0]);
        trace_step!(action);
        trace_note!("Сохраняем рассчитанное значение `(next_state, _)` для следующих операций.");
        let (next_state, _): (usize, f64) =
            move_agent_in_bounded_world_and_calculate_reward(current_state, action);
        trace_step!(next_state);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        println!("{current_state} --{action}--> {next_state}");
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if next_state == current_state {
            trace_note!("Останавливаем цикл после достижения условия завершения.");
            break;
        }
        trace_note!("Обновляем `current_state` результатом текущего шага.");
        current_state = next_state;
        trace_step!(current_state);
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_best_estimated_action_reward_at_each_position(action_values);
}

// Строим график по результатам урока.
fn plot_best_estimated_action_reward_at_each_position(action_values: [[f64; 2]; 5]) {
    trace_note!("Значения из этого урока на графике.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let reinforcement_learning_points: Vec<(f64, f64)> = action_values
        .iter()
        .enumerate()
        .map(|(item_index, row)| (item_index as f64, row[0].max(row[1])))
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
