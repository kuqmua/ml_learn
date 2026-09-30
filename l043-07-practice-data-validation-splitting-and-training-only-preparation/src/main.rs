// Урок 07.7. Практика: проверка данных, разделение и подготовка только по обучающей части.
// Связь с принятой терминологией: Схема данных, типы признаков, разбиение и подготовка без утечки.
// Зачем здесь эта тема: Безопасная обработка данных зависит от порядка шагов: схема, разбиение,
//   обучение преобразований, применение.
// Почему код устроен так: Собираем эти шаги в один конвейер и проверяем отсутствие утечки на
//   validation и test.
// Представь: Новый CSV сначала проходит проверку полей, затем разбиение, и только после этого
//   вычисляются статистики train.
//
// Что повторяем вместе: схема данных, типы признаков, train/validation/test, утечка данных.
// Зачем это нужно: Разделение данных и расчёт статистик только по train защищают оценку модели от утечки
//   информации.
// Что показывает программа: Читаем учебную таблицу признаков и меток. Отделяем обучающую, проверочную и
//   тестовую части. Считаем среднее только по train: validation и test не влияют на подготовку признаков.
// Что проверить при изменении примера: Проверь отсутствие пересечений между частями и повторяемость
//   разбиения.
// Дополнительная практика: Прочитай локальный CSV, проверь схему, разбей данные с seed; вычисляй параметры
//   нормализации только на train.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    const SAMPLE_COMMA_SEPARATED_VALUES: &str =
        "feature,target\n1,0\n2,0\n3,1\n4,1\n5,0\n6,1\n7,0\n8,1\n9,1\n10,0\n";

    let records: Vec<(f64, u8)> = (|| -> Vec<(f64, u8)> {
        SAMPLE_COMMA_SEPARATED_VALUES
            .lines()
            .skip(1)
            .map(|line| {
                let (feature_value, target_value): (&str, &str) = line.split_once(',').unwrap();
                (
                    feature_value.parse().unwrap(),
                    target_value.parse().unwrap(),
                )
            })
            .collect()
    })();
    assert!(
        records.len() >= 9,
        "для разделения нужны строки train, validation и test"
    );
    let training_records: &[(f64, u8)] = &records[..6];
    let validation_records: &[(f64, u8)] = &records[6..8];
    let test_records: &[(f64, u8)] = &records[8..];

    let training_mean: f64 = (|| -> f64 {
        let training_records: &[(f64, u8)] = training_records;
        let mut feature_sum: f64 = 0.0;
        for &(feature_value, _) in training_records {
            feature_sum += feature_value;
        }
        feature_sum / training_records.len() as f64
    })();
    let _ = (
        &(training_records),
        &(validation_records),
        &(test_records),
        &(test_records
            .iter()
            .map(|record| record.0 - training_mean)
            .collect::<Vec<_>>()),
    );

    plot_feature_means_for_training_data_and_whole_dataset(records, training_mean);
}

// Строим график по результатам урока.
fn plot_feature_means_for_training_data_and_whole_dataset(
    records: std::vec::Vec<(f64, u8)>,
    training_mean: f64,
) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Среднее признака",
        "значение",
        &[
            ("train", training_mean),
            (
                "весь набор",
                records.iter().map(|record| record.0).sum::<f64>() / records.len() as f64,
            ),
        ],
    )
    .expect("не удалось сохранить график");
}
