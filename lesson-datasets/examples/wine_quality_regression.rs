use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;
use l050_09_calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count::calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count;
use l051_09_calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count::calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count;

fn report_error(
    records: &[lesson_datasets::WineQualityRedRecord],
    indices: &[usize],
    baseline_quality: f64,
    slope: f64,
    intercept: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let targets: Vec<f64> = indices
        .iter()
        .map(|&index| records[index].quality)
        .collect();
    let baseline = vec![baseline_quality; indices.len()];
    let predictions: Vec<f64> = indices
        .iter()
        .map(|&index| slope * records[index].features[10] + intercept)
        .collect();
    let _ = (
        &(calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(
            &targets, &baseline,
        )?),
        &(calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
            &targets, &baseline,
        )?),
        &(calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(
            &targets,
            &predictions,
        )?),
        &(calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
            &targets,
            &predictions,
        )?),
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = lesson_datasets::load_wine_quality_red_records()?;
    let split = lesson_datasets::split_indices(records.len(), 42)?;
    let training_targets: Vec<f64> = split
        .training_indices
        .iter()
        .map(|&index| records[index].quality)
        .collect();
    let training_alcohol: Vec<f64> = split
        .training_indices
        .iter()
        .map(|&index| records[index].features[10])
        .collect();
    let target_mean = calc_mean_by_summing_values_and_dividing_by_count(&training_targets)?;
    let alcohol_mean = calc_mean_by_summing_values_and_dividing_by_count(&training_alcohol)?;
    let covariance: f64 = training_alcohol
        .iter()
        .zip(&training_targets)
        .map(|(&alcohol, &quality)| (alcohol - alcohol_mean) * (quality - target_mean))
        .sum();
    let variance: f64 = training_alcohol
        .iter()
        .map(|&alcohol| (alcohol - alcohol_mean).powi(2))
        .sum();
    let slope = covariance / variance;
    let intercept = target_mean - slope * alcohol_mean;
    let _ = (
        &(records.len()),
        &(split.training_indices.len()),
        &(split.validation_indices.len()),
        &(split.test_indices.len()),
    );
    report_error(
        &records,
        &split.validation_indices,
        target_mean,
        slope,
        intercept,
    )?;
    report_error(&records, &split.test_indices, target_mean, slope, intercept)?;
    Ok(())
}
