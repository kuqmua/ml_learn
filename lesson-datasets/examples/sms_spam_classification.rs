use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::count_binary_classification_outcomes_from_targets_and_predictions;
use l063_11_calculate_accuracy_as_correct_predictions_divided_by_all_predictions::calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong;
use l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calc_pos_detection_recall_as_true_poss_divided_by_actual_poss_where_1_means_all_found_and_0_means_all_missed;

fn report_majority_baseline(
    records: &[lesson_datasets::SmsSpamRecord],
    indices: &[usize],
    majority_is_spam: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let truth: Vec<bool> = indices
        .iter()
        .map(|&index| records[index].is_spam)
        .collect();
    let predictions = vec![majority_is_spam; indices.len()];
    let counts =
        count_binary_classification_outcomes_from_targets_and_predictions(&truth, &predictions)?;
    let _ = (
        &(calc_prediction_accuracy_as_correct_predictions_divided_by_all_predictions_where_1_means_all_correct_and_0_means_all_wrong(counts)
            .unwrap()),
        &(calc_pos_detection_recall_as_true_poss_divided_by_actual_poss_where_1_means_all_found_and_0_means_all_missed(
            counts,
        )
        .unwrap()),
        &(counts.false_negs_as_missed_pos_cases),
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = lesson_datasets::load_sms_spam_records()?;
    let targets: Vec<u8> = records
        .iter()
        .map(|record| u8::from(record.is_spam))
        .collect();
    let split = lesson_datasets::split_indices_stratified_by_class(&targets, 42)?;
    let training_spam_count = split
        .training_indices
        .iter()
        .filter(|&&index| records[index].is_spam)
        .count();
    let majority_is_spam = training_spam_count * 2 > split.training_indices.len();
    let _ = (
        &(records.len()),
        &(split.training_indices.len()),
        &(split.validation_indices.len()),
        &(split.test_indices.len()),
    );
    report_majority_baseline(&records, &split.validation_indices, majority_is_spam)?;
    report_majority_baseline(&records, &split.test_indices, majority_is_spam)?;
    Ok(())
}
