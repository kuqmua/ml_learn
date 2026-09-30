// Урок 29.2. Ошибка прогноза следующей части текста: среднее отрицательных логарифмов правильных вероятностей.
// Связь с принятой терминологией: Кросс энтропия вероятностей правильных следующих токенов.
// Зачем здесь эта тема: Чтобы обучать вероятности следующего токена, нужна ошибка по правильному
//   продолжению.
// Почему код устроен так: Берём отрицательный логарифм назначенной ему вероятности и усредняем по
//   позициям.
// Представь: Если правильному слову дали вероятность 0,9, ошибка меньше, чем при вероятности 0,1.
//
// Чем выше вероятность правильных токенов, тем меньше ошибка. Для вероятности 1
// вклад равен нулю; вероятность 0 запрещена, потому что её логарифм не определён.

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64]); 3] = [
        ("идеальная уверенность", &[1.0, 1.0]),
        ("правильные токены вероятны", &[0.8, 0.5]),
        ("правильные токены маловероятны", &[0.2, 0.1]),
    ];
    trace_step!(cases);
    trace_note!("Сохраняем результат этого шага в `previous_error`.");
    let mut previous_error: f64 = 0.0;
    trace_step!(previous_error);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (index, (description, probabilities)) in cases.into_iter().enumerate() {
        trace_step!(index);
        trace_step!(description);
        trace_step!(probabilities);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(!probabilities.is_empty(), "нужна хотя бы одна вероятность");
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        trace_note!("Обновляем значение результатом текущего вычисления.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            probabilities.iter().all(|&p| p > 0.0 && p <= 1.0),
            "вероятность должна быть больше 0 и не больше 1"
        );
        trace_note!("Сохраняем результат этого шага в `negative_log_sum`.");
        let mut negative_log_sum: f64 = 0.0;
        trace_step!(negative_log_sum);
        trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for &probability in probabilities {
            trace_step!(probability);
            trace_note!(
                "Учебный аналог `f64::ln`: ряд показывает шаги вычисления, но может быть медленнее и менее точным."
            );
            trace_note!("ln(x) ≈ 2·(t+t³/3+t⁵/5+...), где t=(x−1)/(x+1).");
            let ratio: f64 = (probability - 1.0) / (probability + 1.0);
            trace_step!(ratio);
            trace_note!("Сохраняем результат этого шага в `term`.");
            let mut term: f64 = ratio;
            trace_step!(term);
            trace_note!("Сохраняем результат этого шага в `logarithm`.");
            let mut logarithm: f64 = 0.0;
            trace_step!(logarithm);
            trace_note!("Повторяем расчёт для каждого элемента последовательности.");
            for odd_divisor in (1..=99).step_by(2) {
                trace_step!(odd_divisor);
                trace_note!("Обновляем значение результатом текущего вычисления.");
                logarithm += term / odd_divisor as f64;
                trace_step!(logarithm);
                trace_note!("Обновляем значение результатом текущего вычисления.");
                term *= ratio * ratio;
                trace_step!(term);
            }
            trace_note!("Обновляем значение результатом текущего вычисления.");
            negative_log_sum -= 2.0 * logarithm;
            trace_step!(negative_log_sum);
        }
        trace_note!("Определяем размер данных и сохраняем его в `error`.");
        let error: f64 = negative_log_sum / probabilities.len() as f64;
        trace_step!(error);
        trace_note!("Выбираем дальнейший шаг по выполнению условия.");
        if index > 0 {
            trace_note!("Проверяем ожидаемое свойство учебного примера.");
            assert!(error > previous_error);
        }
        trace_note!("Обновляем значение результатом текущего вычисления.");
        previous_error = error;
        trace_step!(previous_error);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {probabilities:?} → cross-entropy {error:.3}");
    }
    trace_note!("Задаём учебные значения для `invalid`.");
    let invalid: [f64; 2] = [0.0, 0.5];
    trace_step!(invalid);
    trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if invalid.iter().any(|&probability| probability <= 0.0) {
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("нулевая вероятность правильного токена: конечную ошибку вычислить нельзя");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_next_token_loss_as_negative_log_of_correct_text_unit_probability();
}

// Строим график по результатам урока.
fn plot_next_token_loss_as_negative_log_of_correct_text_unit_probability() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Ошибку предсказанного распределения вероятностей называют cross-entropy.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let predicted_probability_error_points: Vec<(f64, f64)> = (1..=100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (probability, -probability.ln())
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
        "Cross-entropy правильного токена",
        "вероятность",
        "ошибка",
        &[lesson_visualization::Series {
            name: "-ln(p)",

            points: &predicted_probability_error_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
