// Урок 09.4. Ограничение сложности модели: добавление штрафа за большой вес.
// Связь с принятой терминологией: Штраф за большой вес линейной модели.
// Зачем здесь эта тема: Без ограничения веса линейная модель может подгоняться под шум обучающих
//   данных.
// Почему код устроен так: Добавляем штраф к ошибке обучения и смотрим, как меняется предпочтение
//   больших коэффициентов.
// Представь: Если модель увеличивает вес до огромного числа ради пары строк, штраф делает такое
//   решение менее выгодным.
//
// Что изучаем: Регуляризация коэффициентов.
// Зачем это нужно: Штраф за большой вес добавляется к ошибке обучения и побуждает модель выбирать более
// простое решение.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Сохраняем рассчитанное значение `prediction_error` для следующих операций.");
    let prediction_error: f64 = 1.0;
    trace_step!(prediction_error);
    trace_note!("Сохраняем рассчитанное значение `weight` для следующих операций.");
    let weight: f64 = 3.0;
    trace_step!(weight);
    trace_note!("Инициализируем значение `penalty_strength` начальным состоянием.");
    let penalty_strength: f64 = 0.2;
    trace_step!(penalty_strength);
    trace_note!("Умножаем значения и сохраняем результат в `squared_weight`.");
    let squared_weight: f64 = weight * weight;
    trace_step!(squared_weight);
    trace_note!("Умножаем значения и сохраняем результат в `objective`.");
    let objective: f64 = prediction_error + penalty_strength * squared_weight;
    trace_step!(objective);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    trace_note!("Присваиваем вычисленное значение соответствующей переменной или полю.");
    trace_note!("Умножаем величины согласно используемой формуле.");
    println!(
        "ошибка={prediction_error}, штраф={}, итог={objective}",
        penalty_strength * squared_weight
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_error_with_and_without_squared_weight_penalty();
}

// Строим график по результатам урока.
fn plot_error_with_and_without_squared_weight_penalty() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    let unregularized_model_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| (plot_step_index as f64 / 10.0, 1.0))
        .collect();
    trace_note!("Собираем значения для `penalty_constrained_model_points` в коллекцию.");
    trace_note!("Штраф за сложность модели называют regularization.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let penalty_constrained_model_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `weight_value`.");
            let weight_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (weight_value, 1.0 + weight_value * weight_value)
        })
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Штраф за большой вес",
        "вес",
        "целевая функция",
        &[
            lesson_visualization::Series {
                name: "без регуляризации",

                points: &unregularized_model_points,
            },
            lesson_visualization::Series {
                name: "со штрафом",

                points: &penalty_constrained_model_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
