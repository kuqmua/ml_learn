// Урок 11.6. Качество поиска положительных примеров: сумма точности, умноженной на прирост полноты.
// Связь с принятой терминологией: Площадь под кривой точности и полноты по оценкам бинарного классификатора.
// Зачем здесь эта тема: При редком положительном классе ложные положительные особенно влияют на
//   precision.
// Почему код устроен так: Меняем порог и смотрим площадь под кривой precision–recall на тех же
//   оценках.
// Представь: Для редкого класса несколько ложных тревог способны сильно снизить долю верных
//   положительных прогнозов.
//
// Что изучаем: Площадь под PR-кривой.
// Зачем это нужно: PR-AUC суммирует precision при увеличении recall и полезна при редком положительном
// классе.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let ranked_labels: [bool; 4] = [true, false, true, false];
    let positive_count: f64 = ranked_labels.iter().filter(|&&label| label).count() as f64;
    let mut found_positive: f64 = 0.0;
    let mut _area: f64 = 0.0;
    for (rank, label) in ranked_labels.into_iter().enumerate() {
        if label {
            found_positive += 1.0;
            let precision: f64 = found_positive / (rank + 1) as f64;
            _area += precision / positive_count;
        }
    }

    plot_correct_positive_prediction_share_against_detected_positive_share();
}

// Строим график по результатам урока.
fn plot_correct_positive_prediction_share_against_detected_positive_share() {
    let precision_recall_area_under_curve_points: Vec<(f64, f64)> =
        [(0.0, 1.0), (0.5, 1.0), (1.0, 2.0 / 3.0)].to_vec();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "PR-кривая по ранжированным меткам",
        "полнота",
        "precision",
        &[lesson_visualization::Series {
            name: "метки +−+−",

            points: &precision_recall_area_under_curve_points,
        }],
    )
    .expect("не удалось сохранить график");
}
