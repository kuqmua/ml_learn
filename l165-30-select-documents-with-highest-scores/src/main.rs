// Урок 30.4. Выбор документов с наибольшими оценками.
// Связь с принятой терминологией: Выбор k документов с наивысшими оценками.
// Зачем здесь эта тема: Поисковый ответ ограничен числом документов, которые можно прочитать или
//   передать модели.
// Почему код устроен так: Сортируем заданные оценки по убыванию и берём первые k; равные оценки
//   сохраняют исходный порядок.
// Представь: Если можно показать два источника из десяти, после сортировки берём только два с
//   наибольшей оценкой.
//
// Что изучаем: Возврат top-k документов.
// Зачем это нужно: После ранжирования оставляем только заданное число наиболее подходящих источников.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Создаём набор значений `ranked` для следующего шага примера.");
    let mut ranked: [(&str, f64); 3] = [("doc-a", 0.8), ("doc-b", 0.3), ("doc-c", 0.9)];
    trace_step!(ranked);
    trace_note!("Сортируем значения в порядке, заданном функцией сравнения.");
    ranked.sort_by(|first_candidate, second_candidate| {
        second_candidate.1.total_cmp(&first_candidate.1)
    });
    trace_note!("Сохраняем рассчитанное значение `highest_ranked_items` для следующих операций.");
    let highest_ranked_items: &[(&str, f64)] = &ranked[..2];
    trace_step!(highest_ranked_items);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("лучшие источники: {highest_ranked_items:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_scores_of_highest_ranked_documents(highest_ranked_items);
}

// Строим график по результатам урока.
fn plot_scores_of_highest_ranked_documents(highest_ranked_items: &[(&str, f64)]) {
    trace_note!("Значения из этого урока на графике.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let highest_ranked_item_points: Vec<(f64, f64)> = highest_ranked_items
        .iter()
        .enumerate()
        .map(|(item_index, (_, score))| ((item_index + 1) as f64, *score))
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
        "Оценки top-k",
        "ранг",
        "оценка",
        &[lesson_visualization::Series {
            name: "выбранные документы",

            points: &highest_ranked_item_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
