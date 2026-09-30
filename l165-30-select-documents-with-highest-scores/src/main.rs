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

fn main() {
    let mut ranked: [(&str, f64); 3] = [("doc-a", 0.8), ("doc-b", 0.3), ("doc-c", 0.9)];
    ranked.sort_by(|first_candidate, second_candidate| {
        second_candidate.1.total_cmp(&first_candidate.1)
    });
    let highest_ranked_items: &[(&str, f64)] = &ranked[..2];

    plot_scores_of_highest_ranked_documents(highest_ranked_items);
}

// Строим график по результатам урока.
fn plot_scores_of_highest_ranked_documents(highest_ranked_items: &[(&str, f64)]) {
    let highest_ranked_item_points: Vec<(f64, f64)> = highest_ranked_items
        .iter()
        .enumerate()
        .map(|(item_index, (_, score))| ((item_index + 1) as f64, *score))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
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
}
