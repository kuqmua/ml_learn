use lesson_datasets::{WineQualityRedRecord, load_wine_quality_red_records, split_indices};
use part_029_lesson_06_mean::arithmetic_mean_of_values;
use part_049_lesson_09_mean_squared_error::mean_squared_error;
use part_050_lesson_09_mean_absolute_error::mean_absolute_error;
use std::error::Error;

fn report_error(
    name: &str,
    records: &[WineQualityRedRecord],
    indices: &[usize],
    baseline_quality: f64,
    slope: f64,
    intercept: f64,
) -> Result<(), Box<dyn Error>> {
    let targets: Vec<f64> = indices
        .iter()
        .map(|&index| records[index].quality)
        .collect();
    let baseline = vec![baseline_quality; indices.len()];
    let predictions: Vec<f64> = indices
        .iter()
        .map(|&index| slope * records[index].features[10] + intercept)
        .collect();
    println!(
        "{name}: baseline MAE={:.3}, MSE={:.3}; alcohol model MAE={:.3}, MSE={:.3}",
        mean_absolute_error(&targets, &baseline)?,
        mean_squared_error(&targets, &baseline)?,
        mean_absolute_error(&targets, &predictions)?,
        mean_squared_error(&targets, &predictions)?
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let records = load_wine_quality_red_records()?;
    let split = split_indices(records.len(), 42)?;
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
    let target_mean = arithmetic_mean_of_values(&training_targets)?;
    let alcohol_mean = arithmetic_mean_of_values(&training_alcohol)?;
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
    println!(
        "Wine Quality red: {} строк, признаки [f64; 11], train={}, validation={}, test={}",
        records.len(),
        split.training_indices.len(),
        split.validation_indices.len(),
        split.test_indices.len()
    );
    report_error(
        "validation",
        &records,
        &split.validation_indices,
        target_mean,
        slope,
        intercept,
    )?;
    report_error(
        "test",
        &records,
        &split.test_indices,
        target_mean,
        slope,
        intercept,
    )?;
    Ok(())
}
