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
    let counts = l060_11_count_correct_and_incorrect_positive_and_negative_predictions::count_binary_classification_outcomes_from_true_and_predicted_labels(
        &truth,
        &predictions,
    )?;
    println!(
        "{name}: accuracy={:.3}, spam recall={:.3}, missed spam={}",
        l060_11_count_correct_and_incorrect_positive_and_negative_predictions::calculate_prediction_accuracy_as_correct_predictions_divided_by_all_predictions(
            counts
        )
        .unwrap(),
        l062_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(counts).unwrap(),
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
