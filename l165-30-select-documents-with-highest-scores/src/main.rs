// Урок 165. Сортировать документы по убыванию готовой оценки и брать первые k результатов.
// Это завершающий шаг поиска: из набора кандидатов выбираем наиболее высоко оценённые.

fn main() {
    let mut ranked: [(&str, f64); 3] = [("doc-a", 0.8), ("doc-b", 0.3), ("doc-c", 0.9)];
    ranked.sort_by(|candidate1, candidate2| candidate2.1.total_cmp(&candidate1.1));
    let highest_ranked_items: &[(&str, f64)] = &ranked[..2];

    // Выполняем вычисления из примера.
    let _ = highest_ranked_items;

    println!("Результаты по убыванию: {ranked:?}; выбраны={highest_ranked_items:?}");
    assert_eq!(highest_ranked_items[0].0, "doc-c");
    assert_eq!(highest_ranked_items[1].0, "doc-a");
}

// Чему учит этот урок:
// Учимся сортировать документы по убыванию готовой оценки и брать первые k результатов.
// Это завершающий шаг поиска: из набора кандидатов выбираем наиболее высоко оценённые.
