// Урок 41.5. Поиск выгодных действий: периодический случайный выбор вместо лучшего известного действия.
// Связь с принятой терминологией: Исследование случайного действия и выбор лучшего известного действия.
// Зачем здесь эта тема: Постоянный выбор известного лучшего действия может не обнаружить лучший
//   путь.
// Почему код устроен так: С вероятностью epsilon исследуем, иначе используем текущую лучшую оценку.
// Представь: Даже если «вправо» пока кажется лучшим, иногда пробуем «влево», чтобы получить новые
//   сведения.
//
// Случайное число ниже epsilon ведёт к исследованию, иначе выбираем лучшее известное действие.
// На границе random=epsilon выбираем использование.

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Сохраняем результат этого шага в `exploration_probability`.");
    let exploration_probability: f64 = 0.1;
    trace_step!(exploration_probability);
    trace_note!("Сохраняем результат этого шага в `best_known_action`.");
    let best_known_action: &str = "вправо";
    trace_step!(best_known_action);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    trace_note!("Число от 0 до 1 задаёт долю единичного интервала; такую долю называют fraction.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, random_number_between_zero_and_one, expected_action) in [
        ("исследование", 0.05, "влево"),
        ("граница", 0.1, "вправо"),
        ("использование", 0.8, "вправо"),
    ] {
        trace_step!(description);
        trace_step!(random_number_between_zero_and_one);
        trace_step!(expected_action);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((0.0..1.0).contains(&random_number_between_zero_and_one));
        trace_note!("Сохраняем результат этого шага в `action`.");
        let action: &str = if random_number_between_zero_and_one < exploration_probability {
            trace_note!("Передаём подпись или текстовое значение для следующего шага.");
            "влево"
        } else {
            trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            best_known_action
        };
        trace_step!(action);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(action, expected_action);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!(
            "{description}: случайное число={random_number_between_zero_and_one}, действие={action}"
        );
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_random_action_choice_below_exploration_threshold();
}

// Строим график по результатам урока.
fn plot_random_action_choice_below_exploration_threshold() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let exploration_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `random_value`.");
            let random_value: f64 = plot_step_index as f64 / 100.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (random_value, if random_value < 0.2 { 1.0 } else { 0.0 })
        })
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
        "Исследование и использование",
        "случайное число",
        "выбор",
        &[lesson_visualization::Series {
            name: "порог ε=0.2: 1=исследование",

            points: &exploration_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
