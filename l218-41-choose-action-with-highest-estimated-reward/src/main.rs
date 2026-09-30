// Урок 41.4. Выбор действия с наибольшей оценкой ожидаемой награды.
// Связь с принятой терминологией: Выбор действия агента по оценкам политики.
// Зачем здесь эта тема: После появления наград политика должна выбирать действие по оценкам
//   состояния.
// Почему код устроен так: Сопоставляем доступные действия с оценками и выбираем предсказанный
//   лучший вариант.
// Представь: В клетке агент сравнивает оценки «влево» и «вправо» и выбирает действие.
//
// Что изучаем: Политика агента.
// Зачем это нужно: Политика выбирает действие по оценкам доступных вариантов в текущем состоянии.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let left_action_value: f64 = 0.2;
    let right_action_value: f64 = 0.8;
    let _action: &str = if right_action_value > left_action_value {
        "вправо"
    } else {
        "влево"
    };

    plot_estimated_rewards_for_available_actions(left_action_value, right_action_value);
}

// Строим график по результатам урока.
fn plot_estimated_rewards_for_available_actions(left_action_value: f64, right_action_value: f64) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Оценки действий политики",
        "Q-значение",
        &[("влево", left_action_value), ("вправо", right_action_value)],
    )
    .expect("не удалось сохранить график");
}
