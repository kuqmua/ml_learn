// Урок 38.2. Ограничение числа найденных фрагментов для размещения в контексте ответа.
// Связь с принятой терминологией: Ограничение числа найденных фрагментов длиной контекстного окна.
// Зачем здесь эта тема: Контекстное окно ограничено; все найденные фрагменты не всегда поместятся.
// Почему код устроен так: Ограничиваем число уже упорядоченных фрагментов; это упрощённая модель
//   лимита контекста без подсчёта токенов.
// Представь: Если в окно помещаются только два фрагмента, третий найденный фрагмент не попадёт в
//   контекст.
//
// Что изучаем: Ограничение длины контекста.
// Зачем это нужно: В доступное окно помещается ограниченное число фрагментов; выбираем наиболее полезные
// до формирования ответа.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `passages` для следующего шага примера.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    let passages: [(&str, f64); 3] = [
        ("важный фрагмент", 0.9),
        ("дополнительный", 0.5),
        ("слабый", 0.1),
    ];
    lesson_trace::trace_step!(passages);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `capacity` для следующих операций.");
    let capacity: usize = 2;
    lesson_trace::trace_step!(capacity);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for (text, score) in passages.into_iter().take(capacity) {
        lesson_trace::trace_step!(text);
        lesson_trace::trace_step!(score);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("в контексте: {text}, оценка={score}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_included_document_count_for_different_context_limits();
}

// Строим график по результатам урока.
fn plot_included_document_count_for_different_context_limits() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let context_length_points: Vec<(f64, f64)> = (0..=6)
        .map(|limit| (limit as f64, limit.min(3) as f64))
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Лимит контекста",
        "максимум фрагментов",
        "число включённых",
        &[lesson_visualization::Series {
            name: "3 фрагмента",

            points: &context_length_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
