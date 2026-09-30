// Урок 29.3. Неопределённость языковой модели (перплексия): e в степени среднего отрицательного логарифма вероятности.
// Связь с принятой терминологией: Perplexity из средней кросс энтропии языковой модели.
// Зачем здесь эта тема: Среднюю кросс энтропию трудно читать как число вариантов продолжения.
// Почему код устроен так: Возводим e в среднюю ошибку и получаем perplexity на той же
//   последовательности.
// Представь: Одинаковая средняя ошибка может читаться как эффективное число возможных продолжений
//   через exp.
//
// Что изучаем: Perplexity.
// Зачем это нужно: Perplexity — экспонента средней cross-entropy; меньшая величина означает лучшее
// вероятностное предсказание.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Инициализируем значение `predicted_probability_error` начальным состоянием.");
    trace_note!("Ошибку предсказанного распределения вероятностей называют cross-entropy.");
    let predicted_probability_error: f64 = 0.7;
    trace_step!(predicted_probability_error);
    trace_note!("Создаём изменяемое значение `term` для следующих операций.");
    let mut term: f64 = 1.0;
    trace_step!(term);
    trace_note!("Создаём изменяемое значение `effective_choice_count` для следующих операций.");
    trace_note!(
        "Эффективное число вариантов, соответствующее ошибке языковой модели, называют perplexity."
    );
    let mut effective_choice_count: f64 = 1.0;
    trace_step!(effective_choice_count);
    trace_note!("Perplexity = exp(cross-entropy); 30 членов ряда Σx^k/k! приближают exp(0.7).");
    for order in 1..=30 {
        trace_step!(order);
        trace_note!("Умножаем накопленное значение на очередной множитель.");
        term *= predicted_probability_error / order as f64;
        trace_step!(term);
        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        effective_choice_count += term;
        trace_step!(effective_choice_count);
    }
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("perplexity={effective_choice_count:.3}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_perplexity_as_e_to_average_negative_log_probability();
}

// Строим график по результатам урока.
fn plot_perplexity_as_e_to_average_negative_log_probability() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let effective_choice_count_points: Vec<(f64, f64)> = (0..=40)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `loss_value`.");
            let loss_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (loss_value, loss_value.exp())
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
        "Perplexity",
        "cross-entropy",
        "perplexity",
        &[lesson_visualization::Series {
            name: "exp(loss)",

            points: &effective_choice_count_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
