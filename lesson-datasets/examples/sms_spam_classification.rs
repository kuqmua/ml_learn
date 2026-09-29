fn report_majority_baseline(
    name: &str,
    records: &[lesson_datasets::SmsSpamRecord],
    indices: &[usize],
    majority_is_spam: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let truth: Vec<bool> = indices
        .iter()
        .map(|&index| records[index].is_spam)
        .collect();
    let predictions = vec![majority_is_spam; indices.len()];
    let counts = part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::count_binary_classification_outcomes_from_true_and_predicted_labels(
        &truth,
        &predictions,
    )?;
    println!(
        "{name}: accuracy={:.3}, spam recall={:.3}, missed spam={}",
        part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::calculate_accuracy_from_binary_classification_counts(
            counts
        )
        .unwrap(),
        part_062_lesson_11_recall_from_binary_classification_counts::recall_from_binary_classification_counts(counts).unwrap(),
        counts.false_negatives
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = lesson_datasets::load_sms_spam_records()?;
    let labels: Vec<u8> = records
        .iter()
        .map(|record| u8::from(record.is_spam))
        .collect();
    let split = lesson_datasets::split_indices_stratified_by_class(&labels, 42)?;
    let training_spam_count = split
        .training_indices
        .iter()
        .filter(|&&index| records[index].is_spam)
        .count();
    let majority_is_spam = training_spam_count * 2 > split.training_indices.len();
    println!(
        "SMS Spam: {} строк, метка bool + текст String, train={}, validation={}, test={}",
        records.len(),
        split.training_indices.len(),
        split.validation_indices.len(),
        split.test_indices.len()
    );
    report_majority_baseline(
        "validation",
        &records,
        &split.validation_indices,
        majority_is_spam,
    )?;
    report_majority_baseline("test", &records, &split.test_indices, majority_is_spam)?;
    Ok(())
}
