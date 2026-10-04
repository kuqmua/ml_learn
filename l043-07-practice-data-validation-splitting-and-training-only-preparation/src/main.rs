// Урок 043. Соединять разбор строк, разделение данных и вычисление статистики только по обучающей
// части.
// Вычитаем обучающее среднее из тестовых признаков, сохраняя независимость итоговой проверки.

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

    // Выполняем вычисления из примера.
    let _ = (&records, training_mean);

    let centered: Vec<f64> = test_records.iter().map(|r| r.0 - training_mean).collect();
    println!(
        "Размеры: обучение={}, выбор={}, тест={}",
        training_records.len(),
        validation_records.len(),
        test_records.len()
    );
    println!("Среднее только обучения={training_mean}; тест после вычитания={centered:?}");
    assert_eq!(training_mean, 3.5);
    assert_eq!(centered, vec![5.5, 6.5]);
}

// Чему учит этот урок:
// Учимся соединять разбор строк, разделение данных и вычисление статистики только по обучающей
// части.
// Вычитаем обучающее среднее из тестовых признаков, сохраняя независимость итоговой проверки.
