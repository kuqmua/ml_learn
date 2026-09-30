// Урок 11.4. Оценка F1: удвоенное произведение точности и полноты, делённое на их сумму.
// Связь с принятой терминологией: Гармоническое среднее точности и полноты бинарной классификации.
// Зачем здесь эта тема: Precision и recall могут расходиться; F1 сводит их в число, чувствительное
//   к меньшему из двух.
// Почему код устроен так: Используем гармоническое среднее и рассматриваем нулевые знаменатели.
// Представь: Если precision высокий, а recall низкий, F1 не позволит одному хорошему числу скрыть
//   другое.
//
// Объединяем precision и recall из двух предыдущих уроков.
// Если обе равны нулю, формула даёт 0/0, поэтому возвращаем None.

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, precision, recall, expected) in [
        ("обе метрики высоки", 1.0, 1.0, Some(1.0)),
        ("одна ниже", 1.0, 0.5, Some(2.0 / 3.0)),
        ("одна равна нулю", 0.0, 0.5, Some(0.0)),
        ("обе равны нулю", 0.0, 0.0, None),
    ] {
        trace_step!(description);
        trace_step!(precision);
        trace_step!(recall);
        trace_step!(expected);
        trace_note!(
            "Сохраняем результат этого шага в `calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum`."
        );
        let harmonic_mean_score: Option<f64> = l066_11_calculate_f1_score_as_twice_precision_times_recall_over_their_sum::calculate_f1_score_as_twice_precision_times_recall_divided_by_their_sum(
            Some(precision),
            Some(recall),
        );
        trace_step!(harmonic_mean_score);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(harmonic_mean_score, expected);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!(
            "{description}: precision={precision}, recall={recall}, F1={harmonic_mean_score:?}"
        );
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_f1_score_as_twice_precision_times_recall_over_their_sum();
}

// Строим график по результатам урока.
fn plot_f1_score_as_twice_precision_times_recall_over_their_sum() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let harmonic_mean_score_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `recall_value`.");
            let recall_value: f64 = plot_step_index as f64 / 100.0;
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Выбираем дальнейший шаг по выполнению условия.");
            (
                recall_value,
                if recall_value == 0.0 {
                    trace_note!("Используем подготовленное значение в следующем шаге примера.");
                    0.0
                } else {
                    trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                    trace_note!("Вычисляем значение по указанной формуле.");
                    2.0 * 0.8 * recall_value / (0.8 + recall_value)
                },
            )
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
        "F1 при precision=0.8",
        "recall",
        "F1",
        &[lesson_visualization::Series {
            name: "F1",

            points: &harmonic_mean_score_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
