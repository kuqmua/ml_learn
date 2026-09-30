// Урок 10.1. Вероятность класса через сигмоиду: единица, делённая на сумму единицы и e в степени, противоположной оценке модели.
// Связь с принятой терминологией: Преобразование логита в вероятность класса сигмоидой.
// Зачем здесь эта тема: Линейная сумма может быть любым числом, а для двух классов нужна величина
//   от 0 до 1.
// Почему код устроен так: Применяем сигмоиду к логиту и сравниваем значения по обе стороны нуля.
// Представь: Логит 0 даёт вероятность 0,5; положительный логит повышает её, отрицательный понижает.
//
// Отрицательный logit даёт вероятность ниже 0.5, нулевой — 0.5,
// положительный — выше 0.5. Значение всегда находится между 0 и 1.

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    trace_note!("Оценку модели до преобразования в вероятность называют logit.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, raw_model_score, expected_side) in [
        ("отрицательный", -2.0, -1),
        ("нулевой", 0.0, 0),
        ("положительный", 2.0, 1),
    ] {
        trace_step!(description);
        trace_step!(raw_model_score);
        trace_step!(expected_side);
        trace_note!("Сохраняем результат этого шага в `term`.");
        let mut term: f64 = 1.0;
        trace_step!(term);
        trace_note!("Сохраняем результат этого шага в `exponential`.");
        let mut exponential: f64 = 1.0;
        trace_step!(exponential);
        trace_note!(
            "Берём 30 членов ряда Тейлора exp(−logit)=Σ(−logit)^k/k! для небольших учебных logit."
        );
        for index in 1..=30 {
            trace_step!(index);
            trace_note!("Обновляем значение результатом текущего вычисления.");
            term *= -raw_model_score / index as f64;
            trace_step!(term);
            trace_note!("Обновляем значение результатом текущего вычисления.");
            exponential += term;
            trace_step!(exponential);
        }
        trace_note!("Сохраняем результат этого шага в `probability`.");
        let probability: f64 = 1.0 / (1.0 + exponential);
        trace_step!(probability);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(probability > 0.0 && probability < 1.0);
        trace_note!(
            "0.5 — середина диапазона вероятностей: ниже неё знак отрицательный, выше положительный."
        );
        let side: i32 = if probability < 0.5 {
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            -1
        } else if probability > 0.5 {
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            1
        } else {
            trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            0
        };
        trace_step!(side);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(side, expected_side);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description} logit {raw_model_score}: вероятность {probability:.4}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_class_probability_as_one_over_one_plus_e_to_negative_score();
}

// Строим график по результатам урока.
fn plot_class_probability_as_one_over_one_plus_e_to_negative_score() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let sigmoid_points: Vec<(f64, f64)> = (-60..=60)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (horizontal_value, 1.0 / (1.0 + (-horizontal_value).exp()))
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
        "Сигмоида",
        "логит",
        "вероятность",
        &[lesson_visualization::Series {
            name: "σ(x)",

            points: &sigmoid_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
